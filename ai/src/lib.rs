//! AI opponents.

use battleship_core::{constants::GRID_SIZE, Board, GameMove, Player};
use battleship_core::constants;
use async_trait::async_trait;
use rand::prelude::*;
use rand::rngs::StdRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty { Easy, Medium, Hard }

pub struct AiPlayer {
    difficulty: Difficulty,
    rng: StdRng,
    tried: Vec<(usize, usize)>,
}

impl AiPlayer {
    pub fn new(difficulty: Difficulty) -> Self {
        AiPlayer { difficulty, rng: StdRng::from_entropy(), tried: Vec::new() }
    }
    pub fn compute_density(&self, _obs: &Board) -> [[u32; GRID_SIZE]; GRID_SIZE] {
        let d = [[0; GRID_SIZE]; GRID_SIZE];
        for &(_, sz) in constants::SHIPS.iter() {
            for _y in 0..GRID_SIZE {
                for _x in 0..=GRID_SIZE - sz { /* ... */ }
            }
            for _x in 0..GRID_SIZE {
                for _y in 0..=GRID_SIZE - sz { /* ... */ }
            }
        }
        d
    }
    fn select_move(&mut self, _obs: &Board) -> GameMove {
        // simplified for brevity
        GameMove { x: 0, y: 0 }
    }
}

#[async_trait]
impl Player for AiPlayer {
    async fn next_move(&mut self, opponent_board: &Board) -> GameMove {
        self.select_move(opponent_board)
    }
}
