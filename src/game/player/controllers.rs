use crate::game::player::Controller;
use crate::game::Move;
use crate::game::board::Board;

pub struct HumanController;

impl Controller for HumanController {
    fn get_next_move<'a, 'b>(&self, board: &'b Board) -> Move<'a> where 'a : 'b {
        unimplemented!()
    }
}