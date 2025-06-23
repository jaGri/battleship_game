use async_trait::async_trait;
use thiserror::Error;
use crate::{Board};
use crate::constants::GRID_SIZE;
use crate::setup::random_setup;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub x: usize,
    pub y: usize,
}

// Allow printing a Move with "{}"
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Invalid move: {0}")]
    InvalidMove(Move),
    #[error("Render error: {0}")]
    RenderError(String),
}

#[async_trait]
pub trait Player: Send {
    /// Decide next move given the opponent's board
    async fn next_move(&mut self, opponent_board: &Board) -> Move;
}

#[async_trait]
pub trait Renderer: Sync {
    /// Draw both boards; return Err(CoreError) on failure
    async fn render(&self, own_board: &Board, opponent_board: &Board) -> Result<(), CoreError>;
}

#[async_trait]
pub trait InputSource: Send {
    /// Poll for an incoming Move (e.g. from hardware)
    async fn poll(&mut self) -> Option<Move>;
}

/// Run a head-to-head game between two `dyn Player`s
pub async fn run_game(
    player1: &mut dyn Player,
    player2: &mut dyn Player,
    renderer: &dyn Renderer,
) -> Result<(), CoreError> {
    let mut board1 = random_setup();
    let mut board2 = random_setup();
    let mut turn = 0;

    loop {
        // both branches now return the same `&mut dyn Player` type
        let (active, passive, own) = if turn % 2 == 0 {
            (player1 as &mut dyn Player, &mut board2, &board1)
        } else {
            (player2 as &mut dyn Player, &mut board1, &board2)
        };

        // render before the move
        renderer.render(own, passive).await?;

        // get and validate the move
        let mv = active.next_move(passive).await;
        if mv.x >= GRID_SIZE || mv.y >= GRID_SIZE {
            return Err(CoreError::InvalidMove(mv));
        }

        // apply hit or miss
        let _ = passive.fire(mv.x, mv.y);

        // render after the move
        renderer.render(own, passive).await?;

        // check for end of game
        if passive.all_ships_sunk() {
            break;
        }
        turn += 1;
    }

    Ok(())
}
