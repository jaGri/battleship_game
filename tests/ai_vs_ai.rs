use battleship_core::{Board, PlayerState};
use battleship_player::{AIPlayer, Player};
use futures::executor::block_on;

#[test]
fn ai_vs_ai_game_completes() {
    let mut board1 = Board::new();
    let mut board2 = Board::new();
    board1.randomly_place_fleet().unwrap();
    board2.randomly_place_fleet().unwrap();

    let mut ai1 = AIPlayer;
    let mut ai2 = AIPlayer;

    let mut turn = true;
    let mut steps = 0;
    while board1.player_state() == PlayerState::Alive
        && board2.player_state() == PlayerState::Alive
        && steps < 200
    {
        if turn {
            let coord = block_on(ai1.next_move(&board2));
            let res = board2.guess(coord).unwrap();
            block_on(ai1.on_move_result(res));
        } else {
            let coord = block_on(ai2.next_move(&board1));
            let res = board1.guess(coord).unwrap();
            block_on(ai2.on_move_result(res));
        }
        turn = !turn;
        steps += 1;
    }

    assert!(
        board1.player_state() == PlayerState::Dead
            || board2.player_state() == PlayerState::Dead
    );
}
