use crate::board::Board;
use crate::interface::GameInterface;

pub struct EmbeddedInterface;

impl GameInterface for EmbeddedInterface {
    fn get_move(&self, _board: &Board) -> (usize, usize) {
        (0, 0) // Replace with embedded-specific logic.
    }

    fn display_board(&self, _board: &Board) {
        // Embedded display logic.
    }

    fn display_message(&self, _message: &str) {
        // Embedded message handling.
    }
}
