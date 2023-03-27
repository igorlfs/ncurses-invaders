pub struct Shield {
    pos: (i32, i32),
    lives: i8,
}

impl Shield {
    pub fn new(pos: (i32, i32), lives: i8) -> Self {
        Self { pos, lives }
    }

    pub fn pos(&self) -> (i32, i32) {
        self.pos
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
