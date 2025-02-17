use battleship::cli_interface::CLIInterface;
use battleship::game_loop::{run_game, GameMode};

fn main() {
    let ui = CLIInterface;
    // You can parse command-line arguments to choose single player or multiplayer.
    run_game(ui, GameMode::SinglePlayer);
}
