use crate::{direction::Direction, COLS, LINES};
use rand::{distributions::Uniform, prelude::Distribution};

pub fn out_of_bounds(pos: (i32, i32)) -> bool {
    pos.0 <= 1 || pos.1 <= 0 || pos.0 >= LINES - 1 || pos.1 >= COLS - 1
}

pub fn random_event(odds: f32) -> bool {
    let step = Uniform::new(0., 1.);
    let mut rng = rand::thread_rng();
    let choice = step.sample(&mut rng);
    choice <= odds
}

pub fn shift(pos: &(i32, i32), dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0, pos.1 + 1),
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::LeftUp => (pos.0 - 1, pos.1 - 1),
        Direction::RightUp => (pos.0 - 1, pos.1 + 1),
        Direction::Down => (pos.0 + 1, pos.1),
    }
}
