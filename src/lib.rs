pub mod board;
pub mod fleet;
pub mod ship;
pub mod probability;
pub mod constants;
pub mod game_loop;
pub mod interface;
pub mod cli_interface;
pub mod embedded_interface;

pub use board::Board;
pub use fleet::Fleet;
pub use ship::Ship;
pub use constants::{GRID_SIZE, PlayerState};
pub use game_loop::{run_game, GameMode};
pub use cli_interface::CLIInterface;
pub use embedded_interface::EmbeddedInterface;
