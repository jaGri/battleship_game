use crate::board::Board;
use crate::interface::GameInterface;
use crate::constants::PlayerState;

pub enum GameMode {
    SinglePlayer,
    Multiplayer,
}

pub fn run_game<T: GameInterface>(ui: T, mode: GameMode) {
    let mut player_board = Board::new();
    let mut opponent_board = Board::new();

    // For example, randomly place fleets for each player initially.
    player_board.randomly_place_fleet().unwrap();
    opponent_board.randomly_place_fleet().unwrap();

    // This is a simplified game loop.
    loop {
        ui.display_board(&player_board);
        ui.display_message("Your turn:");
        let move_coord = ui.get_move(&player_board);

        // Make a guess on the opponent's board, or handle the input for multiplayer.
        match opponent_board.guess(move_coord) {
            Ok(result) => ui.display_message(&format!("You: {}", result)),
            Err(e) => ui.display_message(&format!("Error: {:?}", e)),
        }

        // Check win condition.
        if opponent_board.player_state() == PlayerState::Dead {
            ui.display_message("You won!");
            break;
        }

        // For single player mode, advance the AI's turn.
        if let GameMode::SinglePlayer = mode {
            // Use automated AI move
            match player_board.educated_guess() {
                Ok(result) => ui.display_message(&format!("AI: {}", result)),
                Err(e) => ui.display_message(&format!("AI Error: {:?}", e)),
            }
        } else {
            // In multiplayer mode: switch turns,
            // and call ui.get_move for the other player, etc.
        }

        // Check player's board state.
        if player_board.player_state() == PlayerState::Dead {
            ui.display_message("Opponent won!");
            break;
        }
    }
}
