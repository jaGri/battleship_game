use battleship_common::BoardView;
use crate::GameInterface;

/// Stub implementation of a user interface for embedded targets.
///
/// In a real embedded environment this would interface with
/// hardware such as buttons and displays.
pub struct EmbeddedInterface;

impl GameInterface for EmbeddedInterface {
    fn get_move(&self, _board: &dyn BoardView) -> (usize, usize) {
        (0, 0) // Replace with embedded-specific logic.
    }

    fn display_board(&self, _board: &dyn BoardView) {
        // Embedded display logic.
    }

    fn display_message(&self, _message: &str) {
        // Embedded message handling.
    }
}

