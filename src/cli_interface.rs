// src/cli_interface.rs
use crate::board::Board;
use crate::interface::GameInterface;
use std::io::{self, Write};

pub struct CLIInterface;

impl GameInterface for CLIInterface {
    fn get_move(&self, _board: &Board) -> (usize, usize) {
        // For example, prompt the user for input like "A5" and convert it to coordinates.
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
