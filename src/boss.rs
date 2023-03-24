#[derive(Clone, Copy)]
pub struct Boss {
    left_pos: i32,
}

impl Boss {
    pub fn new(left_pos: i32) -> Self {
        Self { left_pos }
    }

    pub fn left_pos(&self) -> i32 {
        self.left_pos
    }

    pub fn shift(&mut self) {
        self.left_pos += 1;
    }
}
