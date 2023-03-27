use crate::{direction::Direction, util};

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
        self.pos = util::shift(&self.pos, &self.dir);
    }

    pub fn dir(&self) -> &Direction {
        &self.dir
    }

    pub fn set_dir(&mut self, dir: Direction) {
        self.dir = dir;
    }
}
