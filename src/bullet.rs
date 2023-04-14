use crate::{direction::Direction, object::Object, util};

#[derive(Clone, PartialEq)]
pub struct Bullet {
    pos: (i32, i32),
    char: u32,
    color: i16,
    dir: Direction,
    is_explosive: bool,
}

impl Object for Bullet {
    fn pos(&self) -> (i32, i32) {
        self.pos
    }
    fn char(&self) -> u32 {
        self.char
    }
    fn color(&self) -> i16 {
        self.color
    }
}

impl Bullet {
    pub fn new(pos: (i32, i32), dir: Direction, char: u32, color: i16) -> Self {
        Self {
            pos,
            dir,
            char,
            color,
            is_explosive: false,
        }
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

    pub fn is_explosive(&self) -> bool {
        self.is_explosive
    }

    pub fn set_is_explosive(&mut self, is_explosive: bool) {
        self.is_explosive = is_explosive;
    }
}
