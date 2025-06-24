use async_trait::async_trait;
use crate::RawTransport;
use std::io;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct InMemTransport {
    rx: Receiver<Vec<u8>>,
    tx: Sender<Vec<u8>>,
}

impl InMemTransport {
    pub fn pair(buffer: usize) -> (Self, Self) {
        let (tx1, rx1) = tokio::sync::mpsc::channel(buffer);
        let (tx2, rx2) = tokio::sync::mpsc::channel(buffer);
        (InMemTransport { rx: rx1, tx: tx2 }, InMemTransport { rx: rx2, tx: tx1 })
    }
}

#[async_trait]
impl RawTransport for InMemTransport {
    type Error = io::Error;
    async fn send_bytes(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.tx.send(data.to_vec()).await.map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "dropped"))
    }
    async fn recv_bytes(&mut self) -> Result<Vec<u8>, Self::Error> {
        self.rx.recv().await.ok_or(io::Error::new(io::ErrorKind::UnexpectedEof, "closed"))
    }
}

pub struct TcpTransport { stream: TcpStream }

impl TcpTransport {
    pub fn new(stream: TcpStream) -> Self { Self { stream } }
}

#[async_trait]
impl RawTransport for TcpTransport {
    type Error = io::Error;
    async fn send_bytes(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        let len = data.len() as u32;
        self.stream.write_u32_le(len).await?;
        self.stream.write_all(data).await?;
        Ok(())
    }
    async fn recv_bytes(&mut self) -> Result<Vec<u8>, Self::Error> {
        let len = self.stream.read_u32_le().await?;
        let mut buf = vec![0; len as usize];
        self.stream.read_exact(&mut buf).await?;
        Ok(buf)
    }
}

pub struct BtleTransport;
#[async_trait]
impl RawTransport for BtleTransport {
    type Error = io::Error;
    async fn send_bytes(&mut self, _data: &[u8]) -> Result<(), Self::Error> { unimplemented!() }
    async fn recv_bytes(&mut self) -> Result<Vec<u8>, Self::Error> { unimplemented!() }
}
