use battleship_core::{Board, BoardState, GuessResult, PlayerState};
use battleship_transport::Transport;

pub struct Engine<T1: Transport, T2: Transport> {
    board1: Board,
    board2: Board,
    t1: T1,
    t2: T2,
}

impl<T1: Transport, T2: Transport> Engine<T1, T2> {
    pub fn new(t1: T1, t2: T2) -> Self {
        Self {
            board1: Board::new(),
            board2: Board::new(),
            t1,
            t2,
        }
    }

    async fn sync_states(&mut self) {
        let p1_own = BoardState::new(&self.board1, true);
        let p1_op = BoardState::new(&self.board2, false);
        let p2_own = BoardState::new(&self.board2, true);
        let p2_op = BoardState::new(&self.board1, false);
        self.t1.send_board_state(p1_own).await;
        self.t1.send_board_state(p1_op).await;
        self.t2.send_board_state(p2_own).await;
        self.t2.send_board_state(p2_op).await;
    }

    pub async fn run(&mut self) {
        self.board1.randomly_place_fleet().unwrap();
        self.board2.randomly_place_fleet().unwrap();
        self.sync_states().await;
        loop {
            let mv1 = self.t1.recv_move().await;
            let res1 = match self.board2.guess(mv1) {
                Ok(r) => r,
                Err(_) => GuessResult::Miss,
            };
            self.t1.send_result(res1).await;
            self.sync_states().await;
            if self.board2.player_state() == PlayerState::Dead {
                break;
            }
            let mv2 = self.t2.recv_move().await;
            let res2 = match self.board1.guess(mv2) {
                Ok(r) => r,
                Err(_) => GuessResult::Miss,
            };
            self.t2.send_result(res2).await;
            self.sync_states().await;
            if self.board1.player_state() == PlayerState::Dead {
                break;
            }
        }
    }
}
