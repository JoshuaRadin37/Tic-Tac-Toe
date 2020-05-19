use crate::game::player::{Player, Controller, SelfController};
use crate::game::board::*;
use std::rc::Rc;


pub struct GameCycle {
    player1: Rc<Player>,
    player2: Rc<Player>,
    board: Board
}

impl GameCycle {

    pub fn new(player1: Player, player2: Player) -> Self {
        Self {
            player1: Rc::new(player1),
            player2: Rc::new(player2),
            board: Board::new()
        }
    }

    pub fn cycle(&mut self) {

        let player_move = self.player1.next_move(&self.board);
        
        match self.board.make_move(player_move) {
            Ok(_) => {},
            Err(_) => {}
        }

    }
}
