//! Game configuration constants.

/// Size of the square grid (NxN).
pub const GRID_SIZE: usize = 10;

/// Ships available in the game with name and size.
pub const SHIPS: &[(&str, usize)] = &[
    ("Carrier", 5),
    ("Battleship", 4),
    ("Cruiser", 3),
    ("Submarine", 3),
    ("Destroyer", 2),
];
