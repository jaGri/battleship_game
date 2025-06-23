//! AI opponents.

use battleship_core::{board::Cell, constants::GRID_SIZE, Board, Move as GameMove, Player};
use async_trait::async_trait;
use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty { Easy, Medium, Hard }

pub struct AiPlayer {
    difficulty: Difficulty,
    rng: ThreadRng,
    tried: Vec<(usize, usize)>,
}

impl AiPlayer {
    pub fn new(difficulty: Difficulty) -> Self {
        AiPlayer { difficulty, rng: thread_rng(), tried: Vec::new() }
    }
    pub fn compute_density(&self, obs: &Board) -> [[u32; GRID_SIZE]; GRID_SIZE] {
        let mut d = [[0; GRID_SIZE]; GRID_SIZE];
        for &(_, sz) in crate::battleship_core::constants::SHIPS.iter() {
            for y in 0..GRID_SIZE {
                for x in 0..=GRID_SIZE - sz { /* ... */ }
            }
            for x in 0..GRID_SIZE {
                for y in 0..=GRID_SIZE - sz { /* ... */ }
            }
        }
        d
    }
    fn select_move(&mut self, obs: &Board) -> GameMove {
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
