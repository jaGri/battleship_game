//! Console UI.

use battleship_core::{Board, GameMove, InputSource, Renderer, Player, CoreError};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::stdout;
use async_trait::async_trait;
use tokio::sync::mpsc;

pub struct ConsoleInput { rx: mpsc::Receiver<GameMove> }
impl ConsoleInput {
    pub fn new() -> Self { /* spawn reader */ ConsoleInput { rx: mpsc::channel(1).1 } }
}
#[async_trait]
impl InputSource for ConsoleInput {
    async fn poll(&mut self) -> Option<GameMove> { self.rx.recv().await }
}

#[async_trait]
impl Player for ConsoleInput {
    async fn next_move(&mut self, _opponent_board: &Board) -> GameMove {
        self.rx.recv().await.unwrap_or(GameMove { x: 0, y: 0 })
    }
}

pub struct ConsoleRenderer;
impl ConsoleRenderer { pub fn new() -> Self { Self } }
#[async_trait]
impl Renderer for ConsoleRenderer {
    async fn render(&self, own: &Board, opp: &Board) -> Result<(), CoreError> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))
            .map_err(|e| CoreError::RenderError(e.to_string()))?;
        Ok(())
    }
}