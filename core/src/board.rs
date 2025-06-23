//! Board representation and ship placement logic.

use crate::constants::GRID_SIZE;

/// Cell state in the game board.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Ship,
    Hit,
    Miss,
}

/// Represents a Battleship board.
#[derive(Clone, Debug)]
pub struct Board {
    pub grid: [[Cell; GRID_SIZE]; GRID_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Self { grid: [[Cell::Empty; GRID_SIZE]; GRID_SIZE] }
    }

    pub fn place_ship(&mut self, x: usize, y: usize, length: usize, horizontal: bool) -> bool {
        if horizontal {
            if x + length > GRID_SIZE { return false; }
        } else {
            if y + length > GRID_SIZE { return false; }
        }
        for i in 0..length {
            let (cx, cy) = if horizontal { (x + i, y) } else { (x, y + i) };
            if self.grid[cy][cx] != Cell::Empty { return false; }
        }
        for i in 0..length {
            let (cx, cy) = if horizontal { (x + i, y) } else { (x, y + i) };
            self.grid[cy][cx] = Cell::Ship;
        }
        true
    }

    pub fn fire(&mut self, x: usize, y: usize) -> bool {
        match self.grid[y][x] {
            Cell::Ship => { self.grid[y][x] = Cell::Hit; true }
            Cell::Empty => { self.grid[y][x] = Cell::Miss; false }
            _ => false,
        }
    }

    pub fn all_ships_sunk(&self) -> bool {
        self.grid.iter().flatten().all(|&c| c != Cell::Ship)
    }
}
