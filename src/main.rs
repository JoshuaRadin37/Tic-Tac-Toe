pub mod game {

    pub struct Move<'a> {
        pub x_pos: u8,
        pub y_pos: u8,
        pub player: &'a player::Player,
    }

    impl<'a> Move<'a> {
        pub fn new(x_pos: u8, y_pos: u8, player: &'a player::Player) -> Self {
            Self {
                x_pos,
                y_pos,
                player,
            }
        }


    }

    pub mod board;
    pub mod player;

    pub mod cycle;
}

fn main() {
    println!("Hello, world!");
}
