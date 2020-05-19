use crate::game::player::Player;
use crate::game::Move;
use std::rc::Rc;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Result as FmtResult;

pub struct Board([[Option<Rc<Player>>; 3]; 3]);

pub struct Winner(Rc<Player>);

#[derive(Debug)]
pub enum MoveError {
    OutOfBounds(u8, u8),
    PositionAlreadyFilled(Rc<Player>),
}

pub type MoveResult = Result<Option<Winner>, MoveError>;

impl Board {
    pub fn new() -> Self {
        let base: [Option<Rc<Player>>; 3] = Default::default();

        Self([base.clone(), base.clone(), base.clone()])
    }

    pub fn get_open_positions(&self) -> Vec<(u8, u8)> {
        let mut output = vec![];

        for (i, row) in self.0.iter().enumerate() {
            for (j, o) in row.iter().enumerate() {
                if o.is_some() {
                    output.push((i as u8, j as u8));
                }
            }
        }

        output
    }

    pub fn filled_positions(&self) -> u8 {
        let mut count = 0;
        for rows in &self.0 {
            for o in rows {
                if o.is_some() {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn get_at_pos(&mut self, x_pos: u8, y_pos: u8) -> Result<&Option<Rc<Player>>, ()> {
        if x_pos >= 3 || y_pos >= 3 {
            return Err(());
        }

        Ok(&self.0[y_pos as usize][x_pos as usize])
    }

    fn get_at_pos_mut(&mut self, x_pos: u8, y_pos: u8) -> Result<&mut Option<Rc<Player>>, ()> {
        if x_pos >= 3 || y_pos >= 3 {
            return Err(());
        }

        Ok(&mut self.0[y_pos as usize][x_pos as usize])
    }

    pub fn make_move(&mut self, next_move: Move) -> MoveResult {
        let Move {
            x_pos,
            y_pos,
            player,
        } = next_move;
        if x_pos >= 3 || y_pos >= 3 {
            return Err(MoveError::OutOfBounds(x_pos, y_pos));
        }

        let position_result = self.get_at_pos_mut(x_pos, y_pos);
        match position_result {
            Err(()) => Err(MoveError::OutOfBounds(x_pos, y_pos)),
            Ok(Some(other_player)) => Err(MoveError::PositionAlreadyFilled(other_player.clone())),
            Ok(empty_space) => {
                *empty_space = Some(player);
                MoveResult::Ok(self.check_winner())
            }
        }
    }

    fn check_winner(&self) -> Option<Winner> {
        if let Some(winner) = self.check_winner_row() {
            Some(winner)
        } else if let Some(winner) = self.check_winner_column() {
            Some(winner)
        } else if let Some(winner) = self.check_winner_diagonal() {
            Some(winner)
        } else {
            None
        }
    }

    fn check_winner_row(&self) -> Option<Winner> {
        for r in 0..3 {
            let row: &[Option<Rc<Player>>; 3] = &self.0[r];

            let mut found_player = None;
            for p in row {
                match p {
                    None => {
                        if found_player.is_some() {
                            found_player = None;
                        }
                        break;
                    }
                    Some(player) => {
                        if found_player.is_some() {
                            if found_player.unwrap() != player {
                                found_player = None;
                                break;
                            }
                        } else {
                            found_player = Some(player);
                        }
                    }
                }
            }

            if found_player.is_some() {
                return Some(Winner(found_player.unwrap().clone()));
            }
        }

        None
    }

    fn check_winner_column(&self) -> Option<Winner> {
        for c in 0..3 {
            let column = &[&self.0[0][c], &self.0[1][c], &self.0[2][c]];

            let mut found_player = None;
            for p in column {
                match p {
                    None => {
                        if found_player.is_some() {
                            found_player = None;
                        }
                        break;
                    }
                    Some(player) => {
                        if found_player.is_some() {
                            if found_player.unwrap() != player {
                                found_player = None;
                                break;
                            }
                        } else {
                            found_player = Some(player);
                        }
                    }
                }
            }

            if found_player.is_some() {
                return Some(Winner(found_player.unwrap().clone()));
            }
        }

        None
    }

    fn check_winner_diagonal(&self) -> Option<Winner> {
        if self.0[0][0].is_some() && self.0[0][0] == self.0[1][1] && self.0[1][1] == self.0[2][2] {
            if let Some(winner) = &self.0[0][0] {
                Some(Winner(winner.clone()))
            } else {
                unreachable!()
            }
        } else if self.0[0][2].is_some()
            && self.0[0][2] == self.0[1][1]
            && self.0[1][1] == self.0[2][0]
        {
            if let Some(winner) = &self.0[0][2] {
                Some(Winner(winner.clone()))
            } else {
                unreachable!()
            }
        } else {
            None
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for rows in &self.0 {
            for o in rows {
                match o {
                    Some(o) => write!(f, "{} ", o)?,
                    None => write!(f, "  ")?
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::player::*;
    use crate::game::player::controllers::HumanController;

    #[test]
    fn can_place_once() {
        let mut builder = PlayerBuilder::new();
        let player = Rc::new(
            builder.new_player(
                'x',
                Box::new(HumanController)
            )
                .expect("Should be able to create player"))
            ;
        let mut board = Board::new();

        let mov = Move::new(1, 1, &player);
        match board.make_move(mov) {
            Err(err) => {
                panic!("{:?}", err);
            },
            _ => {}
        }

        match board.get_at_pos(1, 1) {
            Ok(Some(player_found)) => {
                assert_eq!(player_found, &player);
            },
            Ok(None) => {
                panic!("There should be a player here")
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

    #[test]
    fn cant_replace() {
        let mut builder = PlayerBuilder::new();
        let player = Rc::new(builder.new_player('x', Box::new(HumanController)).expect("Should be able to create player"));
        let mut board = Board::new();

        let mov = Move::new(1, 1, &player);
        match board.make_move(mov) {
            Err(err) => {
                panic!("{:?}", err);
            },
            _ => {}
        }

        let mov = Move::new(1, 1, &player);
        match board.make_move(mov) {
            Ok(_) => {
                panic!("Should not be able to place here again");
            },
            Err(MoveError::OutOfBounds(_, _)) => {
                panic!("Incorrect error type")
            }
            _ => {
                /* intended behavior */
            }
        }
    }

    #[test]
    fn out_of_bounds_check() {
        let mut builder = PlayerBuilder::new();
        let player = Rc::new(builder.new_player('x', Box::new(HumanController)).expect("Should be able to create player"));
        let mut board = Board::new();

        let mov = Move::new(3, 3, &player);
        match board.make_move(mov) {
            Err(MoveError::OutOfBounds(_, _)) => { }
            Err(_) => {
                panic!("Should return out of bounds error");
            },
            Ok(_) => {
                panic!("Should not be able to place here");
            }
        }
    }
}
