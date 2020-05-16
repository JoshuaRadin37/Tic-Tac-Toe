use crate::game::board::Board;
use crate::game::Move;


pub trait Controller {
    fn get_next_move(&self, board: &Board) -> Move;
    fn get_symbol(&self) -> &char;
}


pub struct Player {
    id: i32,
    symbol: char,
    controller: Box<dyn Controller>
}

impl Player {

    pub fn new(symbol: char, controller: Box<dyn Controller>) -> Self {
        let id = rand::random::<i32>();
        Player {
            id,
            symbol,
            controller
        }
    }
}

impl PartialEq for Player {

    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}



