use crate::game::player::Player;

pub struct Board<'a>([[Option<&'a Player>; 3]; 3]);

pub struct Winner<'a>(&'a Player);

pub enum MoveError<'a> {
    OutOfBounds(u8, u8),
    PositionAlreadyFilled(&'a Player),
}

pub type MoveResult<'a> = Result<Option<Winner<'a>>, MoveError<'a>>;

impl<'a> Board<'a> {
    pub fn new() -> Self {
        Self([[None; 3]; 3])
    }

    pub fn get_at_pos(&mut self, x_pos: u8, y_pos: u8) -> Result<&Option<&'a Player>, ()> {
        if x_pos >= 3 || y_pos >= 3 {
            return Err(());
        }

        Ok(&self.0[y_pos as usize][x_pos as usize])
    }

    fn get_at_pos_mut(&mut self, x_pos: u8, y_pos: u8) -> Result<&mut Option<&'a Player>, ()> {
        if x_pos >= 3 || y_pos >= 3 {
            return Err(());
        }

        Ok(&mut self.0[y_pos as usize][x_pos as usize])
    }

    pub fn make_move(&mut self, x_pos: u8, y_pos: u8, _player: &Player) -> MoveResult {
        if x_pos >= 3 || y_pos >= 3 {
            return Err(MoveError::OutOfBounds(x_pos, y_pos));
        }

        let position_result = self.get_at_pos_mut(x_pos, y_pos);
        match position_result {
            Err(()) => Err(MoveError::OutOfBounds(x_pos, y_pos)),
            Ok(Some(other_player)) => Err(MoveError::PositionAlreadyFilled(other_player)),
            Ok(_empty_space) => MoveResult::Ok(self.check_winner()),
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
            let row: &[Option<&'a Player>; 3] = &self.0[r];

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
                return Some(Winner(found_player.unwrap()));
            }
        }

        None
    }

    fn check_winner_column(&self) -> Option<Winner> {
        for c in 0..3 {
            let column = &[self.0[0][c], self.0[1][c], self.0[2][c]];

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
                return Some(Winner(found_player.unwrap()));
            }
        }

        None
    }

    fn check_winner_diagonal(&self) -> Option<Winner> {
        if self.0[0][0].is_some() && self.0[0][0] == self.0[1][1] && self.0[1][1] == self.0[2][2] {
            Some(Winner(self.0[0][0].unwrap()))
        } else if self.0[0][2].is_some()
            && self.0[0][2] == self.0[1][1]
            && self.0[1][1] == self.0[2][0]
        {
            Some(Winner(self.0[0][2].unwrap()))
        } else {
            None
        }
    }
}


#[cfg(test)]
mod test {

    #[test]
    fn can_place() {
        
    }
}