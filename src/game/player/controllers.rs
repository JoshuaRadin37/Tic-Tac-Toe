use crate::game::board::Board;
use crate::game::player::{Controller, Player};
use crate::game::Move;

use device_query::{DeviceQuery, DeviceState, Keycode};

use std::cmp::Ordering;
use std::rc::Rc;

pub struct HumanController;

#[derive(PartialEq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub trait RelativeDirection<T = Self> {
    /// Returns the (RIGHT/LEFT/NONE, UP/DOWN/NONE)
    fn direction_to(&self, other: &T) -> (Option<Direction>, Option<Direction>);
}

impl RelativeDirection for (u8, u8) {
    fn direction_to(&self, other: &(u8, u8)) -> (Option<Direction>, Option<Direction>) {
        let x_cmp = self.0.cmp(&other.0);
        let y_cmp = self.1.cmp(&other.1);

        use Direction::*;

        (
            match x_cmp {
                Ordering::Greater => Some(Left),
                Ordering::Equal => None,
                Ordering::Less => Some(Right),
            },
            match y_cmp {
                Ordering::Greater => Some(Up),
                Ordering::Equal => None,
                Ordering::Less => Some(Down),
            },
        )
    }
}

trait FindNearPosition {
    fn nearest_position<'a>(&'a self, start: &'a (u8, u8), direction: Direction) -> &'a (u8, u8);
}

impl FindNearPosition for Vec<(u8, u8)> {
    fn nearest_position<'a>(&'a self, start: &'a (u8, u8), direction: Direction) -> &'a (u8, u8) {
        let mut valid_positions: Vec<&(u8, u8)> = self.iter().filter(
            |pt| {
                match start.direction_to(pt) {
                    (None, None) => { false },
                    (Some(lr), Some(ud)) => {
                        direction == lr || direction == ud
                    },
                    (Some(lr), None) => {
                        direction == lr
                    },
                    (None, Some(ud)) => {
                        direction == ud
                    }
                }
            }
        ).collect();

        if valid_positions.is_empty() {
            start
        } else {
            valid_positions.sort_by(
                |(x1, y1), (x2, y2)| {
                    let p1 = ((*x1 as f64 - start.0 as f64).powi(2) + (*y1 as f64 - start.1 as f64).powi(2)).sqrt();
                    let p2 = ((*x2 as f64 - start.0 as f64).powi(2) + (*y2 as f64 - start.1 as f64).powi(2)).sqrt();

                    p1.partial_cmp(&p2).unwrap()
                }
            );
            valid_positions[0]
        }
    }
}

impl Controller for HumanController {
    fn get_next_move(&self, player: &Rc<Player>, board: &Board) -> Move
    {
        let positions = board.get_open_positions();
        let mut position: &(u8, u8) = &positions[0];

        let write_line = |(x, y): &(u8, u8)| {
            print!("\r[Player {symbol}] - Playing at {x}, {y}", symbol = player, x = x, y = y);
        };

        write_line(position);


        loop {
            let device_state = DeviceState::new();
            let keys = device_state.get_keys();

            let mut position_updated = false;
            let mut selected = false;

            if keys.contains(&Keycode::Up) {
                position = positions.nearest_position(&position, Direction::Up);
                position_updated = true;
            } else if keys.contains(&Keycode::Right) {
                position = positions.nearest_position(&position, Direction::Right);
                position_updated = true;
            } else if keys.contains(&Keycode::Down) {
                position = positions.nearest_position(&position, Direction::Down);
                position_updated = true;
            } else if keys.contains(&Keycode::Left) {
                position = positions.nearest_position(&position, Direction::Left);
                position_updated = true;
            }

            if keys.contains(&Keycode::Enter) {
                selected = true;
            }

            if position_updated {
                write_line(position);
            }

            if selected {
                break;
            }
        }

        println!();
        let (x, y) = position;
        Move::new(*x, *y, player)

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn relative_position_correct() {
        let point = (1u8, 1u8);
        // Same
        let position = point.direction_to(&(1, 1));
        assert_eq!(position, (None, None));

        // Left
        let position = point.direction_to(&(0, 1));
        assert_eq!(position, (Some(Direction::Left), None));

        // Right
        let position = point.direction_to(&(2, 1));
        assert_eq!(position, (Some(Direction::Right), None));

        // Up
        let position = point.direction_to(&(1, 0));
        assert_eq!(position, (None, Some(Direction::Up)));

        // Down
        let position = point.direction_to(&(1, 2));
        assert_eq!(position, (None, Some(Direction::Down)));

        // Up and Left
        let position = point.direction_to(&(0, 0));
        assert_eq!(position, (Some(Direction::Left), Some(Direction::Up)));

        // Up and Right
        let position = point.direction_to(&(2, 0));
        assert_eq!(position, (Some(Direction::Right), Some(Direction::Up)));

        // Down and Left
        let position = point.direction_to(&(0, 2));
        assert_eq!(position, (Some(Direction::Left), Some(Direction::Down)));

        // Up and Right
        let position = point.direction_to(&(2, 2));
        assert_eq!(position, (Some(Direction::Right), Some(Direction::Down)));
    }

    #[test]
    fn nearest_position() {

        let start: (u8, u8) = (1, 1);
        let positions: Vec<(u8, u8)> = vec![(1, 1), (2, 1), (2, 0)];

        let nearest = positions.nearest_position(&start, Direction::Right);
        assert_eq!(nearest, &(2, 1));

        let positions: Vec<(u8, u8)> = vec![(1, 1), (2, 1), (2, 0)];
        let nearest = positions.nearest_position(&start, Direction::Left);
        assert_eq!(nearest, &start);
    }
}
