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
        print!("Enter your move (e.g., A5): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_uppercase();
        if input.len() < 2 {
            return (0, 0);
        }
        let row_char = input.chars().next().unwrap();
        let row = row_char as usize - b'A' as usize;
        let col = input[1..].parse::<usize>().unwrap_or(1);
        if row >= battleship_core::constants::GRID_SIZE || col == 0 || col > battleship_core::constants::GRID_SIZE {
            (0, 0)
        } else {
            (row, col - 1)
        }
    }

    fn display_board(&self, board: &Board) {
        println!("{}", board);
    }

    fn display_message(&self, message: &str) {
        println!("{}", message);
    }
}

impl CLIInterface {
    pub fn get_move_with_default(&self, _board: &Board, default: (usize, usize)) -> (usize, usize) {
        let row_char = (b'A' + default.0 as u8) as char;
        let col = default.1 + 1;
        print!("Enter your move (e.g., {}{}): ", row_char, col);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_uppercase();
        if input.is_empty() {
            return default;
        }
        if input.len() < 2 {
            return (0, 0);
        }
        let row_char = input.chars().next().unwrap();
        let row = row_char as usize - b'A' as usize;
        let col = input[1..].parse::<usize>().unwrap_or(1);
        if row >= battleship_core::constants::GRID_SIZE || col == 0 || col > battleship_core::constants::GRID_SIZE {
            (0, 0)
        } else {
            (row, col - 1)
        }
    }
}
