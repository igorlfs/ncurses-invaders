#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Effect {
    Double,
}

pub struct PowerUp {
    pos: (i32, i32),
    effect: Effect,
}

impl PowerUp {
    pub fn new(pos: (i32, i32), effect: Effect) -> Self {
        Self { pos, effect }
    }

    pub fn pos(&self) -> (i32, i32) {
        self.pos
    }

    pub fn effect(&self) -> &Effect {
        &self.effect
    }
}
