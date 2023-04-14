use crate::{
    logic::{CHAR_BOSS, COLOR_BOSS},
    object::Object,
};

#[derive(Clone, Copy)]
pub struct Boss {
    pos: (i32, i32),
    char: u32,
    color: i16,
}

impl Object for Boss {
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

impl Boss {
    pub fn new(pos: (i32, i32)) -> Self {
        Self {
            pos,
            char: CHAR_BOSS,
            color: COLOR_BOSS,
        }
    }

    pub fn left_pos(&self) -> i32 {
        self.pos.1
    }

    pub fn shift(&mut self) {
        self.pos.1 += 1;
    }
}
