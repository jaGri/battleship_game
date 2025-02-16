mod ship;
mod fleet;
mod constants;
mod board;
mod probability;

use constants::GRID_SIZE;

use crate::constants::PlayerState;
use crate::board::Board;
use crate::probability::calc_pdf_and_guess;
 
fn main() {
    let mut player_board = Board::new();
    let mut opponent_board = Board::new();
    let _ = player_board.randomly_place_fleet();
    let _ = opponent_board.randomly_place_fleet();
    let mut turn = 0;
    while turn < GRID_SIZE*GRID_SIZE {
        if opponent_board.player_state() == PlayerState::Dead {
            println!("Player Won"); 
            break
        }
        if player_board.player_state() == PlayerState::Dead {
            println!("Opponent Won");
            break;
        }
        match opponent_board.educated_guess() {
            Ok(guess_result) => (),//println!("Player attacked opponent: {}", guess_result), 
            Err(e) => println!("Guess Error: {:?}", e)
        }
        match player_board.educated_guess() {
            Ok(guess_result) => (),//println!("Opponent attacked player: {}", guess_result), 
            Err(e) => println!("Guess Error: {:?}", e)
        }
        turn += 1;
    }
    println!("{}", opponent_board);
    println!("{}", player_board);
    println!("Turns: {}", turn);
    // let mut x_board = Board::new();
    // let _ = x_board.place_ship("Carrier", (0,0), true);
    // let _ = x_board.place_ship("Battleship", (3,0), true);
    // let _ = x_board.place_ship("Cruiser", (4,0), true);
    // let _ = x_board.place_ship("Submarine", (5,0), true);
    // let _ = x_board.place_ship("Destroyer", (0,9), false);
    // for guess in [(0,9), (1,9), (0,0), (0,1), (0,2), (0,3), (0,4), (2,0), (3,2), (4,2), (4,3)] {
    //     let _ = x_board.guess(guess);
    // }
    // println!("{}", x_board);
    // calc_pdf_and_guess(&x_board);
}