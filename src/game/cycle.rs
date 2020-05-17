use crate::game::player::{Player, Controller};
use crate::game::board::*;
use crate::game::Move;


pub struct GameCycle<'a> {
    player1: Player,
    player2: Player,
    board: Board<'a>
}

impl <'a> GameCycle<'a> {

    pub fn new(player1: Player, player2: Player) -> Self {
        Self {
            player1,
            player2,
            board: Board::new()
        }
    }

    pub fn cycle(&mut self) {

        let player_move = self.player1.get_next_move(&self.board);
        
        match self.board.make_move(player_move) {
            Ok(_) => {},
            Err(_) => {}
        }

    }
}
