mod ai;
pub use ai::{ProbAi, Difficulty};

use async_trait::async_trait;
use battleship_core::state::GameState;
use interface_cli::{InputProvider, OutputRenderer, InputEvent};

#[derive(Clone, Copy, Debug)]
pub struct Move { pub x: u8, pub y: u8 }

#[async_trait]
pub trait Player: Send {
    async fn next_move(&mut self, state: &GameState) -> Move;
}

pub struct HumanPlayer<I>
where I: InputProvider + OutputRenderer + Send + Sync + Clone {
    interface: I,
}
impl<I> HumanPlayer<I>
where I: InputProvider + OutputRenderer + Send + Sync + Clone {
    pub fn new(interface: I) -> Self { Self { interface } }
}
#[async_trait]
impl<I> Player for HumanPlayer<I>
where I: InputProvider + OutputRenderer + Send + Sync + Clone {
    async fn next_move(&mut self, state: &GameState) -> Move {
        self.interface.render_state(state).await.unwrap();
        loop {
            match self.interface.next_input().await {
                Ok(InputEvent::Attack { x, y }) => return Move { x, y },
                Ok(InputEvent::Save) | Ok(InputEvent::Load) => println!("Save/Load elsewhere"),
                _ => println!("Invalid command"),
            }
        }
    }
}

pub struct AiPlayer;
#[async_trait]
impl Player for AiPlayer {
    async fn next_move(&mut self, state: &GameState) -> Move {
        for x in 0..10 { for y in 0..10 {
            if state.is_valid_attack(state.turn, x, y) {
                return Move { x, y };
            }
        }}
        Move { x:0, y:0 }
    }
}
