use battleship_core::Board;
use crate::GameInterface;
use std::io::{self, Write};

/// Simple command line user interface implementation.
///
/// This implementation reads moves from standard input and prints
/// the board and messages to standard output.
pub struct CLIInterface;

impl GameInterface for CLIInterface {
    fn get_move(&self, _board: &Board) -> (usize, usize) {
        // Prompt the user for input like "A5" and convert it to coordinates.
        print!("Enter your move (e.g., A5): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        // Convert input to board coordinates (this is just a placeholder).
        // You would include proper parsing and error handling.
        (0, 0)
    }

    fn display_board(&self, board: &Board) {
        println!("{}", board);
    }

    fn display_message(&self, message: &str) {
        println!("{}", message);
    }
}

