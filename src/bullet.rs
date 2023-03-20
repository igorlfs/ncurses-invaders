#[derive(PartialEq, Clone)]
pub enum Direction {
    LeftUp,
    RightUp,
    Up,
    Down,
}

#[derive(Clone, PartialEq)]
pub struct Bullet {
    pos: (i32, i32),
    dir: Direction,
}

impl Bullet {
    pub fn new(pos: (i32, i32), dir: Direction) -> Self {
        Self { pos, dir }
    }

    pub fn pos(&self) -> (i32, i32) {
        self.pos
    }

    pub fn shift(&mut self) {
        let previous = self.pos();
        let new_pos = match self.dir {
            Direction::Up => (previous.0 - 1, previous.1),
            Direction::LeftUp => (previous.0 - 1, previous.1 - 1),
            Direction::RightUp => (previous.0 - 1, previous.1 + 1),
            Direction::Down => (previous.0 + 1, previous.1),
        };
        self.pos = new_pos;
    }
}
