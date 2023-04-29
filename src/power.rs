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
    Explode,
    Follower,
    Grenade,
    Hijack,
    Invincible,
    Jump,
    Lock,
    Mindcontrol,
    Numb,
    Obstacle,
    Pierce,
    Quickshot,
    Reflect,
    Shield,
    Triple,
    Ultra,
    Vendetta,
    Warp,
    Xerox,
    Yield,
    Zombify,
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Distribution<Effect> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Effect {
        match rng.gen_range(0..=24) {
            0 => Effect::Antigravity,
            1 => Effect::Block,
            2 => Effect::Clear,
            3 => Effect::Double,
            4 => Effect::Explode,
            5 => Effect::Follower,
            6 => Effect::Grenade,
            7 => Effect::Hijack,
            8 => Effect::Invincible,
            9 => Effect::Jump,
            10 => Effect::Lock,
            11 => Effect::Mindcontrol,
            12 => Effect::Numb,
            13 => Effect::Obstacle,
            14 => Effect::Pierce,
            15 => Effect::Quickshot,
            16 => Effect::Reflect,
            17 => Effect::Shield,
            18 => Effect::Triple,
            19 => Effect::Ultra,
            20 => Effect::Vendetta,
            21 => Effect::Warp,
            22 => Effect::Xerox,
            23 => Effect::Yield,
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
