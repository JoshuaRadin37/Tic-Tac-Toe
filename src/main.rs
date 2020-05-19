use crate::game::board::Board;
use crate::game::player::{Player, PlayerBuilder};

pub mod game;

fn main() {
    let board = Board::new();
    let player_builder =PlayerBuilder::new();
    let p1 = player_builder.new_player('x');
}

