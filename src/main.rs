pub mod game {

    pub struct Move<'a> {
        x_pos: u8,
        y_pos: u8,
        player: &'a player::Player,
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
}

fn main() {
    println!("Hello, world!");
}
