use crate::object::Object;

pub struct Shield {
    pos: (i32, i32),
    lives: i8,
    color: i16,
    char: u32,
}

impl Object for Shield {
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

impl Shield {
    pub fn new(pos: (i32, i32), lives: i8, color: i16, char: u32) -> Self {
        Self {
            pos,
            lives,
            color,
            char,
        }
    }

    pub fn damage(&mut self) {
        self.lives -= 1;
    }

    pub fn set_x(&mut self, x: i32) {
        self.pos.1 = x;
    }

    pub fn is_alive(&self) -> bool {
        self.lives > 0
    }
}
