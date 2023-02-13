#[derive(Clone, PartialEq)]
pub struct Bullet {
    pos: (i32, i32),
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
}

impl Bullet {
    pub fn new(pos: (i32, i32)) -> Self {
        Self { pos }
    }

    pub fn pos(&self) -> (i32, i32) {
        self.pos
    }

    pub const UNDEFINED: (i32, i32) = (-1, -1);

    pub fn shift(&mut self, direction: &Direction, reference: &i32) {
        let previous = self.pos();
        let mut new_pos = Bullet::UNDEFINED;
        if *direction == Direction::Up && previous.0 >= *reference {
            new_pos = (previous.0 - 1, previous.1);
        } else if *direction == Direction::Down && previous.0 <= *reference {
            new_pos = (previous.0 + 1, previous.1)
        }
        self.pos = new_pos;
    }
}
