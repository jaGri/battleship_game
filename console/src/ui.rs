//! Console UI.

use battleship_core::{Board, Cell, Move as GameMove, InputSource, Renderer, CoreError};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::stdout;
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::io::{AsyncBufReadExt, BufReader};

pub struct ConsoleInput { rx: mpsc::Receiver<GameMove> }
impl ConsoleInput {
    pub fn new() -> Self { /* spawn reader */ ConsoleInput { rx: mpsc::channel(1).1 } }
}
#[async_trait]
impl InputSource for ConsoleInput {
    async fn poll(&mut self) -> Option<GameMove> { self.rx.recv().await }
}

pub struct ConsoleRenderer;
impl ConsoleRenderer { pub fn new() -> Self { Self } }
#[async_trait]
impl Renderer for ConsoleRenderer {
    async fn render(&self, own: &Board, opp: &Board) -> Result<(), CoreError> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        Ok(())
    }
}