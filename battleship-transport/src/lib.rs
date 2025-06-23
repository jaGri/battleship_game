use async_trait::async_trait;
use battleship_core::GuessResult;

#[async_trait]
pub trait Transport {
    async fn send_move(&mut self, coord: (usize, usize));
    async fn recv_move(&mut self) -> (usize, usize);
    async fn send_result(&mut self, result: GuessResult);
    async fn recv_result(&mut self) -> GuessResult;
}

