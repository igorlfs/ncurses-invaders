use crate::{logic::COLOR_POWERS, object::Object};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum Effect {
    Antigravity,
    Block,
    Clear,
    Double,
    Follower,
    Grenade,
    Hijack,
    Invincible,
    Jump,
    Lock,
    Mindcontrol,
    Obstacle,
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
        match rng.gen_range(0..=20) {
            0 => Effect::Antigravity,
            1 => Effect::Block,
            2 => Effect::Clear,
            3 => Effect::Double,
            4 => Effect::Follower,
            5 => Effect::Grenade,
            6 => Effect::Hijack,
            7 => Effect::Invincible,
            8 => Effect::Jump,
            9 => Effect::Lock,
            10 => Effect::Mindcontrol,
            11 => Effect::Obstacle,
            12 => Effect::Pierce,
            13 => Effect::Quickshot,
            14 => Effect::Reflect,
            15 => Effect::Shield,
            16 => Effect::Triple,
            17 => Effect::Vendetta,
            18 => Effect::Warp,
            19 => Effect::Xerox,
            _ => Effect::Zombify,
        }
    }
}

#[derive(Clone)]
pub struct PowerUp {
    pos: (i32, i32),
    effect: Effect,
    char: u32,
    color: i16,
}

impl Object for PowerUp {
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

impl PowerUp {
    pub fn new(pos: (i32, i32), effect: Effect) -> Self {
        let first_char = effect.to_string().chars().next();
        match first_char {
            Some(char) => {
                let char = char as u32;
                Self {
                    pos,
                    effect,
                    char,
                    color: COLOR_POWERS,
                }
            }
            None => panic!("Woopsie. Effect {effect} contains no characters. How did this happen?"),
        }
    }

    pub fn effect(&self) -> &Effect {
        &self.effect
    }
}
