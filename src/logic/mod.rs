mod generate;
mod handle;
mod hit;
mod shift;

use self::handle::Handle;
use self::{generate::Generate, hit::Hit, shift::Move};
use crate::direction::Direction;
use crate::{
    boss::Boss,
    power::{Effect, PowerUp},
    shield::Shield,
    shooter::Shooter,
};
use ncurses::{getmaxx, getmaxy, WINDOW};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const BOSS_SCORE: i32 = 4000;
const ENEMY_SCORE: i32 = 20;
const ENEMY_ROWS: i32 = 5;
const ENEMIES_PER_ROW: i32 = 10;
const POWER_COOLDOWN: Duration = Duration::from_secs(10);
const ATTACK_COOLDOWN: Duration = Duration::from_millis(600);
const DOUBLE_ATTACK_COOLDOWN: Duration = Duration::from_millis(1000);
const TRIPLE_ATTACK_COOLDOWN: Duration = Duration::from_millis(1600);
const COMBINED_ATTACK_COOLDOWN: Duration = Duration::from_millis(2000);
const POWER_PROBABILITY: f32 = 0.08;
const FIRE_PROBABILITY: f32 = 0.05;
const BOSS_PROPABILITY: f32 = 0.001;
const SHIELDS: i32 = 14;
const OBSTACLES: i32 = 4;

pub struct Logic {
    enemies: Vec<Shooter>,
    player: Shooter,
    powers: Vec<PowerUp>,
    shields: Vec<Shield>,
    obstacles: Vec<Shield>,
    follower: Option<Shield>,
    effects: HashMap<Effect, Instant>,
    boss: Option<Boss>,
    height: i32,
    width: i32,
    dir: Direction,
    last_attack: Instant,
    cooldown_attack: Duration,
    score_increment: i32,
    xerox: Option<Shooter>,
    slow_down: bool,
}

impl Logic {
    pub fn new(win: WINDOW) -> Self {
        let y = getmaxy(win);
        let x = getmaxx(win);
        Self {
            enemies: vec![],
            powers: vec![],
            effects: HashMap::new(),
            shields: vec![],
            obstacles: vec![],
            player: Shooter::new((y - 2, x / 2)),
            follower: None,
            boss: None,
            xerox: None,
            height: y,
            width: x,
            dir: Direction::Right,
            last_attack: Instant::now(),
            cooldown_attack: ATTACK_COOLDOWN,
            score_increment: 0,
            slow_down: false,
        }
    }

    pub fn level_up(&mut self, level: &mut i32) {
        Generate::enemies(self);
        *level += 1;
    }

    pub fn player_shoot(&mut self) {
        if self.last_attack.elapsed() >= self.cooldown_attack {
            self.player
                .shoot(Direction::Up, Handle::power(self, &Effect::Grenade));
            Handle::attack(self);
            self.last_attack = Instant::now();
        }
    }

    pub fn player_move(&mut self, direction: &Direction) {
        Move::player(self, direction);
        Move::follower(self);
    }

    pub fn generate(&mut self) {
        Generate::enemy_attack(self);
        Generate::power(self);
        Generate::boss(self);
        Generate::xerox(self);
        Generate::obstacles(self);
    }

    pub fn shift(&mut self, level: &i32) -> bool {
        Move::bullets(self);
        Hit::moving(self, level);
        Move::foes(self)
    }

    pub fn hit(&mut self, level: &i32) -> bool {
        Hit::powers(self);
        Hit::shields(self);
        Hit::follower(self);
        Hit::moving(self, level);
        Hit::obstacles(self);
        Handle::mind_control(self);
        Hit::player(self)
    }

    pub fn enemies(&self) -> &[Shooter] {
        self.enemies.as_ref()
    }

    pub fn player(&self) -> &Shooter {
        &self.player
    }

    pub fn powers(&self) -> &[PowerUp] {
        self.powers.as_ref()
    }

    pub fn shields(&self) -> &[Shield] {
        self.shields.as_ref()
    }

    pub fn boss(&self) -> Option<&Boss> {
        self.boss.as_ref()
    }

    pub fn active_effects(&self) -> Vec<Effect> {
        let mut effects: Vec<Effect> = vec![];
        for (effect, time) in &self.effects {
            if time.elapsed() < POWER_COOLDOWN {
                effects.push(*effect);
            }
        }
        effects
    }

    pub fn follower(&self) -> Option<&Shield> {
        self.follower.as_ref()
    }

    pub fn score_increment(&self) -> i32 {
        self.score_increment
    }

    pub fn score_reset(&mut self) {
        self.score_increment = 0;
    }

    pub fn enemies_mut(&mut self) -> &mut Vec<Shooter> {
        &mut self.enemies
    }

    pub fn xerox(&self) -> Option<&Shooter> {
        self.xerox.as_ref()
    }

    pub fn obstacles(&self) -> &[Shield] {
        self.obstacles.as_ref()
    }
}
