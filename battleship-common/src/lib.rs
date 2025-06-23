pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub mod board {
    /// Trait representing a read-only view of a game board.
    ///
    /// Implementors should provide a textual display via [`std::fmt::Display`]
    /// and report the board's dimensions through [`grid_size`].
    pub trait BoardView: std::fmt::Display {
        /// Return the length of one side of the square board.
        fn grid_size(&self) -> usize;
    }
}

pub use board::BoardView;

/// Result of a guess on the game board.
#[derive(Debug, PartialEq)]
pub enum GuessResult {
    /// Shot missed all ships.
    Miss,
    /// Shot hit a ship but didn't sink it.
    Hit,
    /// Shot hit and sunk a ship (includes ship name).
    Sunk(&'static str),
}

impl std::fmt::Display for GuessResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuessResult::Miss => write!(f, "Miss"),
            GuessResult::Hit => write!(f, "Hit"),
            GuessResult::Sunk(name) => write!(f, "The {} was sunk!", name),
        }
    }
}

