use std::fmt;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum Effect {
    Double,
    Triple,
    Shield,
    Pierce,
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Distribution<Effect> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Effect {
        match rng.gen_range(0..=3) {
            0 => Effect::Double,
            1 => Effect::Triple,
            2 => Effect::Shield,
            _ => Effect::Pierce,
        }
    }
}

#[derive(Clone)]
pub struct PowerUp {
    pos: (i32, i32),
    effect: Effect,
    char: u32,
}

impl PowerUp {
    pub fn new(pos: (i32, i32), effect: Effect) -> Self {
        let first_char = effect.to_string().chars().next();
        match first_char {
            Some(char) => {
                let char = char as u32;
                Self { pos, effect, char }
            }
            None => panic!("Woopsie. Effect {effect} contains no characters. How did this happen?"),
        }
    }

    pub fn pos(&self) -> (i32, i32) {
        self.pos
    }

    pub fn effect(&self) -> &Effect {
        &self.effect
    }

    pub fn char(&self) -> u32 {
        self.char
    }
}
