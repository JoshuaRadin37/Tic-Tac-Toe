use crate::game::Move;
use crate::game::board::Board;

use std::collections::HashSet;
use std::fmt::{Debug, Display};

pub mod controllers;
pub trait Controller {
    fn get_next_move<'a, 'b>(&self, board: &'b Board) -> Move<'a> where 'a : 'b;
}



pub struct Player {
    id: i32,
    symbol: char,
    controller: Box<dyn Controller>,
}

impl Player {
    fn new(id: i32, symbol: char, controller: Box<dyn Controller>) -> Self {
        
        Player {
            id,
            symbol,
            controller,
        }
    }
}

impl Controller for Player {
    fn get_next_move<'a, 'b>(&self, board: &'b Board) -> Move<'a> where 'a : 'b {
        self.controller.get_next_move(board)
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "Player {}", self.id)
     }
}


impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "Player {}", self.id)
     }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct PlayerBuilder {
    used_ids: HashSet<i32>,
    used_symbols: HashSet<char>
}

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolUsed(char);

impl PlayerBuilder {

    pub fn new() -> Self {
        Self {
            used_ids: HashSet::new(),
            used_symbols: HashSet::new()
        }
    }

    pub fn new_player(&mut self, symbol: char, controller: Box<dyn Controller>) -> Result<Player, SymbolUsed> {
        if self.used_symbols.contains(&symbol) {
            return Err(SymbolUsed(symbol));
        }

        let id = loop {
            let id_attempt: i32 = rand::random();

            if !self.used_ids.contains(&id_attempt) {
                break id_attempt;
            }
        };

        self.used_ids.insert(id);
        self.used_symbols.insert(symbol);

        Ok(Player::new(id, symbol, controller))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::player::controllers::*;

    #[test]
    fn create_unique_players() {
        let mut builder = PlayerBuilder::new();
        builder.new_player('c', Box::new(HumanController)).expect("A first player shouldn't fail no matter what");
        builder.new_player('d', Box::new(HumanController)).expect("Unique symbol and should not fail");
    }

    #[test]
    fn detect_repeated_symbols() {
        let mut builder = PlayerBuilder::new();
        builder.new_player('c', Box::new(HumanController)).expect("A first player shouldn't fail no matter what");
        let result = builder.new_player('c', Box::new(HumanController));
        assert_eq!(result, Err(SymbolUsed('c')));
    }
}