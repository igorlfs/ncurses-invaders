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

    pub fn lives(&self) -> i8 {
        self.lives
    }
}
