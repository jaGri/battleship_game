use async_trait::async_trait;
pub mod adapters;
use adapters::{InMemTransport, TcpTransport, BtleTransport};
use battleship_core::message::{Envelope, Message};
use bincode;
use std::{io, time::Duration};
use tokio::time::timeout;

#[async_trait]
pub trait RawTransport {
    async fn send_bytes(&mut self, data: &[u8]) -> io::Result<()>;
    async fn recv_bytes(&mut self) -> io::Result<Vec<u8>>;
}

#[async_trait]
impl<T: ?Sized + RawTransport + Send> RawTransport for Box<T> {
    async fn send_bytes(&mut self, data: &[u8]) -> io::Result<()> {
        (**self).send_bytes(data).await
    }
    async fn recv_bytes(&mut self) -> io::Result<Vec<u8>> {
        (**self).recv_bytes().await
    }
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

    pub async fn send(&mut self, payload: Message) -> io::Result<()> {
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
        Err(io::Error::new(io::ErrorKind::TimedOut, "send retry limit"))
    }

    pub async fn recv(&mut self) -> io::Result<Message> {
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
        Err(io::Error::new(io::ErrorKind::TimedOut, "recv retry limit"))
    }
}
