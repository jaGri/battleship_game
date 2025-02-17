use battleship::embedded_interface::EmbeddedInterface;
use battleship::game_loop::{run_game, GameMode};

fn main() {
    // Initialization for embedded peripherals would go here.
    let ui = EmbeddedInterface;
    run_game(ui, GameMode::Multiplayer);
}
