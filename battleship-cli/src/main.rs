use battleship_core::{Board, constants::PlayerState};
use battleship_interface::{cli::CLIInterface, GameInterface};
use battleship_player::probability;

fn main() {
    let ui = CLIInterface;
    run_game(ui);
}

fn run_game<I: GameInterface>(ui: I) {
    let mut player_board = Board::new();
    let mut ai_board = Board::new();

    player_board.randomly_place_fleet().unwrap();
    ai_board.randomly_place_fleet().unwrap();

    loop {
        ui.display_message("Opponent board:");
        ui.display_message(&ai_board.format_board(false));
        ui.display_message("Your board:");
        ui.display_message(&player_board.format_board(true));
        ui.display_message("Your turn:");
        let coord = ui.get_move(&player_board);

        match ai_board.guess(coord) {
            Ok(result) => ui.display_message(&format!("You: {}", result)),
            Err(e) => {
                ui.display_message(&format!("Error: {:?}", e));
                continue;
            }
        }

        if ai_board.player_state() == PlayerState::Dead {
            ui.display_message("You won!");
            break;
        }

        let ai_guess = probability::calc_pdf_and_guess(&player_board);
        match player_board.guess(ai_guess) {
            Ok(result) => ui.display_message(&format!("AI: {}", result)),
            Err(e) => ui.display_message(&format!("AI Error: {:?}", e)),
        }

        if player_board.player_state() == PlayerState::Dead {
            ui.display_message("AI won!");
            break;
        }
    }

    ui.display_message("Final boards:");
    ui.display_message("AI board:");
    ui.display_message(&ai_board.format_board(true));
    ui.display_message("Player board:");
    ui.display_message(&player_board.format_board(true));
}

