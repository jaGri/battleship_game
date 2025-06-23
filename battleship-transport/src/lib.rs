use async_trait::async_trait;
use battleship_core::GuessResult;
use std::sync::mpsc::{channel, Receiver, Sender};

#[async_trait]
pub trait Transport {
    async fn send_move(&mut self, coord: (usize, usize));
    async fn recv_move(&mut self) -> (usize, usize);
    async fn send_result(&mut self, result: GuessResult);
    async fn recv_result(&mut self) -> GuessResult;
}

/// Local in-memory transport using standard channels.
///
/// Instances are created as connected pairs via [`LocalTransport::pair`].
pub struct LocalTransport {
    move_tx: Sender<(usize, usize)>,
    move_rx: Receiver<(usize, usize)>,
    result_tx: Sender<GuessResult>,
    result_rx: Receiver<GuessResult>,
}

impl LocalTransport {
    /// Create a pair of connected transports.
    ///
    /// The first transport's outgoing messages become the second transport's
    /// incoming messages and vice versa.
    pub fn pair() -> (Self, Self) {
        let (tx_move_1, rx_move_1) = channel();
        let (tx_move_2, rx_move_2) = channel();
        let (tx_res_1, rx_res_1) = channel();
        let (tx_res_2, rx_res_2) = channel();

        let t1 = LocalTransport {
            move_tx: tx_move_1,
            move_rx: rx_move_2,
            result_tx: tx_res_1,
            result_rx: rx_res_2,
        };

        let t2 = LocalTransport {
            move_tx: tx_move_2,
            move_rx: rx_move_1,
            result_tx: tx_res_2,
            result_rx: rx_res_1,
        };

        (t1, t2)
    }
}

#[async_trait]
impl Transport for LocalTransport {
    async fn send_move(&mut self, coord: (usize, usize)) {
        let _ = self.move_tx.send(coord);
    }

    async fn recv_move(&mut self) -> (usize, usize) {
        self.move_rx.recv().expect("transport channel closed")
    }

    async fn send_result(&mut self, result: GuessResult) {
        let _ = self.result_tx.send(result);
    }

    async fn recv_result(&mut self) -> GuessResult {
        self.result_rx.recv().expect("transport channel closed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn local_transport_pair_communication() {
        let (mut a, mut b) = LocalTransport::pair();

        block_on(a.send_move((1, 2)));
        assert_eq!(block_on(b.recv_move()), (1, 2));

        block_on(b.send_result(GuessResult::Hit));
        assert_eq!(block_on(a.recv_result()), GuessResult::Hit);
    }
}
