use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(PartialEq, Clone)]
pub enum Direction {
    Left,
    Right,
    LeftUp,
    RightUp,
    Up,
    Down,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=1) {
            0 => Direction::LeftUp,
            _ => Direction::RightUp,
        }
    }
}
