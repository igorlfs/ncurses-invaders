use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Effect {
    Double,
    Triple,
}

impl Distribution<Effect> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Effect {
        match rng.gen_range(0..=1) {
            0 => Effect::Double,
            _ => Effect::Triple,
        }
    }
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
