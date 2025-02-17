use crate::board::Board;

pub trait GameInterface {
    fn get_move(&self, board: &Board) -> (usize, usize);
    fn display_board(&self, board: &Board);
    fn display_message(&self, message: &str);
}
