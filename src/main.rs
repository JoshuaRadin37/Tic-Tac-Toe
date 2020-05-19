use crate::game::board::Board;
use crate::game::player::{Player, PlayerBuilder};
use crate::game::player::controllers::HumanController;
use crate::game::cycle::GameCycle;

pub mod game;

#[macro_use]
extern crate crossterm;

fn main() {

    let player_builder =PlayerBuilder::new();
    let p1 = player_builder.new_player('x', HumanController.into()).unwrap();
    let p2 = player_builder.new_player('o', HumanController.into()).unwrap();

    let mut cycle = GameCycle::new(p1, p2);
    let outcome = cycle.cycle();


}

