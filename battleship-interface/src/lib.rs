use battleship_common::BoardView;

/// Abstraction over user interaction for the Battleship game.
///
/// Different implementations can provide various ways of obtaining
/// moves and displaying information, such as a command line interface
/// or an embedded system display.
pub trait GameInterface {
    /// Request the next move from the player.
    ///
    /// The implementation is responsible for validating and parsing
    /// any user input into board coordinates.
    fn get_move(&self, board: &dyn BoardView) -> (usize, usize);

    /// Render the current state of the provided board to the user.
    fn display_board(&self, board: &dyn BoardView);

    /// Show an informational message to the player.
    fn display_message(&self, message: &str);
}

pub mod cli;
pub mod embedded;

