use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum Effect {
    Antigravity,
    Clear,
    Double,
    Follower,
    Grenade,
    Hijack,
    Lock,
    Pierce,
    Quickshot,
    Reflect,
    Shield,
    Triple,
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Distribution<Effect> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Effect {
        match rng.gen_range(0..=11) {
            0 => Effect::Antigravity,
            1 => Effect::Clear,
            2 => Effect::Double,
            3 => Effect::Follower,
            4 => Effect::Grenade,
            5 => Effect::Hijack,
            6 => Effect::Lock,
            7 => Effect::Pierce,
            8 => Effect::Quickshot,
            9 => Effect::Reflect,
            10 => Effect::Shield,
            _ => Effect::Triple,
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
