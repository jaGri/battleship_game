use async_trait::async_trait;
use battleship_core::state::{GameState, PlayerId};
use battleship_core::probability::{ProbabilityEngine, ProbabilityGrid};
use crate::Move;

#[derive(Clone, Copy)]
pub enum Difficulty { Easy, Medium, Hard }

pub struct ProbAi {
    engine: ProbabilityEngine,
    difficulty: Difficulty,
    player_id: PlayerId,
}

impl ProbAi {
    pub fn new(width:u8, height:u8, difficulty:Difficulty, player_id:PlayerId) -> Self {
        Self { engine: ProbabilityEngine::new(width as usize, height as usize), difficulty, player_id }
    }
}

#[async_trait]
impl super::Player for ProbAi {
    async fn next_move(&mut self, state: &GameState) -> Move {
        let board = match self.player_id {
            PlayerId::One => &state.board_p2,
            PlayerId::Two => &state.board_p1,
        };
        let mut grid = self.engine.compute(board);
        match self.difficulty {
            Difficulty::Easy => grid.add_noise(0.3),
            Difficulty::Medium => grid.add_noise(0.1),
            Difficulty::Hard => (),
        }
        if let Some((x,y)) = grid.max_cell() {
            Move { x:x as u8, y:y as u8 }
        } else {
            for x in 0..board.width { for y in 0..board.height {
                if board.is_unattacked(x as u8, y as u8) {
                    return Move { x:x as u8, y:y as u8 };
                }
            }}
            Move { x:0, y:0 }
        }
    }
}
