use std::rc::Rc;

pub struct Move {
    pub x_pos: u8,
    pub y_pos: u8,
    pub player: Rc<player::Player>,
}

impl Move {
    pub fn new(x_pos: u8, y_pos: u8, player: &Rc<player::Player>) -> Self {
        Self {
            x_pos,
            y_pos,
            player: player.clone(),
        }
    }


}

pub mod board;
pub mod player;

pub mod cycle;
