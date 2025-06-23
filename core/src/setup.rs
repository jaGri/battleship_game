//! Random ship placement.

use crate::board::Board;
use crate::constants::{SHIPS, GRID_SIZE};
use rand::{thread_rng, Rng};

pub fn random_setup() -> Board {
    let mut board = Board::new();
    let mut rng = thread_rng();
    for &(_, size) in SHIPS.iter() {
        loop {
            let x = rng.gen_range(0..GRID_SIZE);
            let y = rng.gen_range(0..GRID_SIZE);
            let horizontal = rng.gen_bool(0.5);
            if board.place_ship(x, y, size, horizontal) { break; }
        }
    }
    board
}
