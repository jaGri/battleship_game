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

