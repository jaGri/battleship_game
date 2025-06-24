pub mod board;
pub mod constants;
pub mod fleet;
pub mod ship;

pub use battleship_common::BoardView;
pub use battleship_config::{GRID_SIZE, SHIPS};
pub use board::Board;
pub use constants::{Cell, GameplayError, GuessError, PlayerState};
pub use battleship_common::GuessResult;
pub use fleet::Fleet;
pub use ship::Ship;
