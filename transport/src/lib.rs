use async_trait::async_trait;
use crate::adapters::InMemTransport;
use crate::adapters::TcpTransport;
use crate::adapters::BtleTransport;
use crate::adapters;
use crate::RawTransport;
use crate::message::Envelope;
use bincode;
use std::{io, time::Duration};
use tokio::time::{sleep, timeout};

#[async_trait]
pub trait RawTransport {
    type Error;
    async fn send_bytes(&mut self, data: &[u8]) -> Result<(), Self::Error>;
    async fn recv_bytes(&mut self) -> Result<Vec<u8>, Self::Error>;
}

pub struct ReliableTransport<T: RawTransport + Send> {
    inner: T,
    send_seq: u64,
    recv_ack: u64,
    retry_limit: usize,
    timeout_ms: u64,
}

impl<T: RawTransport + Send> ReliableTransport<T> {
    pub fn new(inner: T) -> Self {
        Self { inner, send_seq: 0, recv_ack: 0, retry_limit: 5, timeout_ms: 200 }
    }
    pub fn with_retries(mut self, retries: usize) -> Self { self.retry_limit = retries; self }
    pub fn with_timeout(mut self, ms: u64) -> Self { self.timeout_ms = ms; self }

    pub async fn send(&mut self, payload: crate::message::Message) -> Result<(), T::Error> {
        let env = Envelope { seq: self.send_seq, ack: Some(self.recv_ack), payload };
        let buf = bincode::serialize(&env).unwrap();
        let mut attempts = 0;
        while attempts < self.retry_limit {
            attempts += 1;
            if timeout(Duration::from_millis(self.timeout_ms), self.inner.send_bytes(&buf)).await.is_err() {
                continue;
            }
            // try ack recv
            if let Ok(Ok(ack_buf)) = timeout(Duration::from_millis(self.timeout_ms), self.inner.recv_bytes()).await {
                if let Ok(ack_env) = bincode::deserialize::<Envelope>(&ack_buf) {
                    if let Some(a) = ack_env.ack { self.recv_ack = self.recv_ack.max(a); }
                }
            }
            if self.send_seq <= self.recv_ack {
                self.send_seq += 1;
                return Ok(());
            }
        }
        Err(io::Error::new(io::ErrorKind::TimedOut, "send retry limit").into())
    }

    pub async fn recv(&mut self) -> Result<crate::message::Message, T::Error> {
        let mut attempts = 0;
        while attempts < self.retry_limit {
            attempts += 1;
            let buf = match timeout(Duration::from_millis(self.timeout_ms), self.inner.recv_bytes()).await {
                Ok(Ok(data)) => data,
                _ => continue,
            };
            let env: Envelope = match bincode::deserialize(&buf) {
                Ok(e) => e,
                Err(_) => continue,
            };
            if env.seq <= self.recv_ack { continue; }
            if env.seq > self.recv_ack + 10 { self.recv_ack = env.seq; }
            if let Some(a) = env.ack { self.recv_ack = self.recv_ack.max(a); }
            self.recv_ack = env.seq;
            return Ok(env.payload);
        }
        Err(io::Error::new(io::ErrorKind::TimedOut, "recv retry limit").into())
    }
}
