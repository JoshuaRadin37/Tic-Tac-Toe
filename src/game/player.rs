use crate::game::board::Board;
use crate::game::Move;
use std::collections::HashSet;

pub trait Controller {
    fn get_next_move(&self, board: &Board) -> Move;
    fn get_symbol(&self) -> &char;
}

pub mod controllers;

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

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct PlayerBuilder {
    used_ids: HashSet<i32>,
    used_symbols: HashSet<char>
}

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

    #[test]
    fn create_unique_players() {
        let mut builder = PlayerBuilder::new();
        builder.new_player('c', Box::new(x: T))
    }
}