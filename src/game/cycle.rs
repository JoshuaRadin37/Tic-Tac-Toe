use crate::game::player::{Player, Controller, SelfController};
use crate::game::board::*;
use std::rc::Rc;
use rand::{random, Rng};
use rand::distributions::{Distribution, Standard};
use crate::game::cycle::CurrentPlayer::{Player2, Player1};


pub struct GameCycle {
    player1: Rc<Player>,
    player2: Rc<Player>,
    board: Board
}

enum CurrentPlayer {
    Player1,
    Player2
}


impl Distribution<CurrentPlayer> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CurrentPlayer {
        if rng.gen_bool(0.5) {
            CurrentPlayer::Player1
        } else {
            CurrentPlayer::Player2
        }
    }
}

impl GameCycle {

    pub fn new(player1: Player, player2: Player) -> Self {
        Self {
            player1: Rc::new(player1),
            player2: Rc::new(player2),
            board: Board::new()
        }
    }

    fn next_player(current: &mut CurrentPlayer) {
        *current = match current {
            CurrentPlayer::Player1 => Player2,
            CurrentPlayer::Player2 => Player1
        }
    }

    fn get_player(&self, current: &CurrentPlayer) -> &Rc<Player> {
        match current {
            CurrentPlayer::Player1 => &self.player1,
            CurrentPlayer::Player2 => &self.player2
        }
    }

    pub fn cycle(&mut self) -> Option<Winner> {

        let mut places = 0;


        let mut player = random::<CurrentPlayer>();

        while places < 9 {
            let result = self.player_place(self.get_player(&player).clone(), &mut places);
            if result.is_some() {
                return result;
            }

            places += 1;
            Self::next_player(&mut player);
        }

        None

    }

    fn player_place(&mut self, player: Rc<Player>, places: &mut i32) -> Option<Winner> {
        loop {
            let player_move = player.next_move(&self.board);

            match self.board.make_move(player_move) {
                Ok(None) => {
                    *places += 1;
                    break;
                },
                Ok(winner) => { return winner; },
                Err(_) => {}
            }
        };
        None
    }
}
