use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum Effect {
    Antigravity,
    Clear,
    Block,
    Double,
    Follower,
    Grenade,
    Hijack,
    Invincible,
    Lock,
    Mindcontrol,
    Pierce,
    Quickshot,
    Reflect,
    Shield,
    Triple,
    Vendetta,
    Warp,
    Xerox,
    Zombify,
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Distribution<Effect> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Effect {
        match rng.gen_range(0..=18) {
            0 => Effect::Antigravity,
            1 => Effect::Block,
            2 => Effect::Clear,
            3 => Effect::Double,
            4 => Effect::Follower,
            5 => Effect::Grenade,
            6 => Effect::Hijack,
            7 => Effect::Invincible,
            8 => Effect::Lock,
            9 => Effect::Mindcontrol,
            10 => Effect::Pierce,
            11 => Effect::Quickshot,
            12 => Effect::Reflect,
            13 => Effect::Shield,
            14 => Effect::Triple,
            15 => Effect::Vendetta,
            16 => Effect::Warp,
            17 => Effect::Xerox,
            _ => Effect::Zombify,
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
