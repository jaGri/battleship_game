#![allow(
    dead_code,
    clippy::doc_overindented_list_items,
    clippy::needless_range_loop,
    clippy::useless_conversion
)]

use async_trait::async_trait;
use battleship_common::BoardView;
use battleship_core::{Board, GuessResult};
use battleship_interface::GameInterface;
use battleship_transport::Transport;

pub mod posterior;
pub mod probability;

/// Core player trait used by the game engine.
#[async_trait]
pub trait Player<B: BoardView + Sync> {
    async fn next_move(&mut self, board: &B) -> (usize, usize);
    async fn on_move_result(&mut self, result: GuessResult);
}

/// Human player backed by an Interface implementation.
pub struct HumanPlayer<I: GameInterface> {
    interface: I,
}

impl<I: GameInterface> HumanPlayer<I> {
    pub fn new(interface: I) -> Self {
        Self { interface }
    }
}

#[async_trait]
impl<I, B> Player<B> for HumanPlayer<I>
where
    I: GameInterface + Send,
    B: BoardView + Sync,
{
    async fn next_move(&mut self, board: &B) -> (usize, usize) {
        self.interface.get_move(board)
    }

    async fn on_move_result(&mut self, result: GuessResult) {
        self.interface.display_message(&format!("{}", result));
    }
}

/// Placeholder AI player using probability module.
pub struct AIPlayer;

#[async_trait]
impl Player<Board> for AIPlayer {
    async fn next_move(&mut self, board: &Board) -> (usize, usize) {
        probability::calc_pdf_and_guess(board)
    }

    async fn on_move_result(&mut self, _result: GuessResult) {}
}

/// Remote player communicating over a Transport.
pub struct RemotePlayer<I: GameInterface, T: Transport> {
    iface: I,
    transport: T,
}

impl<I: GameInterface, T: Transport> RemotePlayer<I, T> {
    pub fn new(iface: I, transport: T) -> Self {
        Self { iface, transport }
    }
}

#[async_trait]
impl<I, T, B> Player<B> for RemotePlayer<I, T>
where
    I: GameInterface + Send,
    T: Transport + Send,
    B: BoardView + Sync,
{
    async fn next_move(&mut self, board: &B) -> (usize, usize) {
        self.iface.get_move(board)
    }

    async fn on_move_result(&mut self, result: GuessResult) {
        let _ = self.transport.send_result(result).await;
    }
}

/// Client that communicates exclusively via a [`Transport`] and
/// displays state through a [`GameInterface`].
pub struct InterfaceClient<I: GameInterface, T: Transport> {
    iface: I,
    transport: T,
}

impl<I: GameInterface, T: Transport> InterfaceClient<I, T> {
    pub fn new(iface: I, transport: T) -> Self {
        Self { iface, transport }
    }

    pub async fn run(&mut self) {
        loop {
            let my_state = self.transport.recv_board_state().await;
            let opp_state = self.transport.recv_board_state().await;

            self.iface.display_message("Opponent board:");
            self.iface.display_board(&opp_state);
            self.iface.display_message(&opp_state.ships);
            self.iface.display_message("Your board:");
            self.iface.display_board(&my_state);
            self.iface.display_message(&my_state.ships);

            if my_state.state == battleship_core::PlayerState::Dead
                || opp_state.state == battleship_core::PlayerState::Dead
            {
                break;
            }

            let mv = self.iface.get_move(&opp_state);
            self.transport.send_move(mv).await;
            let res = self.transport.recv_result().await;
            self.iface.display_message(&format!("{}", res));
        }
    }
}
