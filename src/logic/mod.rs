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
const ENEMY_ROWS: i32 = 4;
const ENEMIES_PER_ROW: i32 = 10;
const POWER_COOLDOWN: Duration = Duration::from_secs(10);
const ATTACK_COOLDOWN: Duration = Duration::from_millis(600);
const POWER_PROBABILITY: f32 = 0.08;
const FIRE_PROBABILITY: f32 = 0.05;
const BOSS_PROPABILITY: f32 = 0.001;
const SHIELDS: i32 = 14;

pub struct Logic {
    enemies: Vec<Shooter>,
    player: Shooter,
    powers: Vec<PowerUp>,
    shields: Vec<Shield>,
    effects: HashMap<Effect, Instant>,
    boss: Option<Boss>,
    height: i32,
    width: i32,
    dir: Direction,
    last_attack: Instant,
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
            player: Shooter::new((y - 2, x / 2)),
            boss: None,
            height: y,
            width: x,
            dir: Direction::Right,
            last_attack: Instant::now(),
        }
    }

    pub fn level_up(&mut self, level: &mut i32) {
        Generate::enemies(self);
        *level += 1;
    }

    pub fn player_shoot(&mut self) {
        let cooldown = if Handle::power(self, &Effect::QuickShot) {
            ATTACK_COOLDOWN / 2
        } else {
            ATTACK_COOLDOWN
        };
        if self.last_attack.elapsed() >= cooldown {
            self.player.shoot(Direction::Up);
            if Handle::power(self, &Effect::Double) {
                Handle::double(self);
            }
            if Handle::power(self, &Effect::Triple) {
                Handle::triple(self);
            }
            self.last_attack = Instant::now();
        }
    }

    pub fn player_move(&mut self, direction: &Direction) {
        self.player.shift(direction);
    }

    pub fn generate(&mut self) {
        Generate::enemy_attack(self);
        Generate::power(self);
        Generate::boss(self);
    }

    pub fn shift(&mut self) -> bool {
        Move::bullets(self);
        Move::boss(self);
        self.move_enemies()
    }

    pub fn hit(&mut self, score: &mut i32, level: &i32) -> bool {
        Hit::powers(self);
        Hit::shields(self);
        if Hit::boss(self) {
            *score += BOSS_SCORE * level;
        }
        *score += (Hit::enemies(self) as i32) * ENEMY_SCORE * level;
        Hit::player(self)
    }

    fn get_horizontal_indexes(&self) -> (usize, usize) {
        let mut left_index = 0;
        let mut right_index = 0;
        let enemies = self.enemies();
        for i in 1..enemies.len() {
            if enemies[i].pos().1 <= enemies[left_index].pos().1 {
                left_index = i;
            } else if enemies[i].pos().1 >= enemies[right_index].pos().1 {
                right_index = i;
            }
        }
        (left_index, right_index)
    }

    fn get_bottom(&self) -> i32 {
        let mut bottom = 0;
        let enemies = self.enemies();
        for i in 1..enemies.len() {
            if enemies[bottom].pos().0 <= enemies[i].pos().0 {
                bottom = i;
            }
        }
        enemies[bottom].pos().0
    }

    pub fn move_enemies(&mut self) -> bool {
        if !Handle::power(self, &Effect::Inactivate) {
            let (left, right) = self.get_horizontal_indexes();

            if self.dir == Direction::Right && self.enemies[right].pos().1 == self.width - 2
                || self.enemies[left].pos().1 == 1 && self.dir == Direction::Left
            {
                self.dir = Direction::Down;
            } else if self.dir == Direction::Down && self.enemies[right].pos().1 == self.width - 2 {
                self.dir = Direction::Left;
            } else if self.dir == Direction::Down && self.enemies[left].pos().1 == 1 {
                self.dir = Direction::Right;
            }

            if !(self.dir == Direction::Down) || !Handle::power(self, &Effect::Antigravity) {
                for enemy in self.enemies.iter_mut() {
                    enemy.shift(&self.dir);
                }
            }
        }

        self.get_bottom() == self.height - 2
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
}
