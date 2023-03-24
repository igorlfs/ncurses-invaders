use crate::{
    boss::Boss,
    bullet,
    power::{Effect, PowerUp},
    shield::Shield,
    shooter::Shooter,
    util,
};
use ncurses::{getmaxx, getmaxy, WINDOW};
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Down,
}

const ENEMY_ROWS: i32 = 4;
const ENEMIES_PER_ROW: i32 = 10;
const POWER_COOLDOWN: Duration = Duration::from_secs(10);
const ATTACK_COOLDOWN: Duration = Duration::from_millis(500);
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
            dir: Direction::Left,
            last_attack: Instant::now(),
        }
    }

    pub fn create_enemies(&mut self) {
        for j in 0..ENEMY_ROWS {
            for i in 0..ENEMIES_PER_ROW {
                self.enemies.push(Shooter::new((2 * (j + 1), 2 * i + 1)));
            }
        }
    }

    pub fn create_shield(&mut self) {
        for i in 1..SHIELDS {
            self.shields
                .push(Shield::new((self.height - 3, 3 * i - 1), 3))
        }
    }

    pub fn create_boss(&mut self) {
        if Self::random_event(BOSS_PROPABILITY) && self.boss.is_none() {
            self.boss = Some(Boss::new(0));
        }
    }

    pub fn create_power(&mut self) {
        if Self::random_event(POWER_PROBABILITY) {
            let mut rng = rand::thread_rng();
            let y = rng.gen_range(2..self.height - 2);
            let x = rng.gen_range(1..self.width - 1);
            self.powers.push(PowerUp::new((y, x), rand::random()));
        }
    }

    fn handle_double(&mut self) {
        let player_pos = self.player.pos();
        let pos = (player_pos.0 - 1, player_pos.1);
        self.player.shoot_pos(&pos, bullet::Direction::Up);
    }

    fn handle_triple(&mut self) {
        let player_pos = self.player.pos();
        let pos_left = (player_pos.0 - 1, player_pos.1 + 1);
        self.player.shoot_pos(&pos_left, bullet::Direction::LeftUp);
        let pos_right = (player_pos.0 - 1, player_pos.1 - 1);
        self.player
            .shoot_pos(&pos_right, bullet::Direction::RightUp);
    }

    fn handle_shield(&mut self) {
        let effect = Effect::Shield;
        match self.effects.get(&effect) {
            Some(time) => {
                if time.elapsed() >= POWER_COOLDOWN {
                    self.effects.insert(effect, Instant::now());
                    self.create_shield();
                }
            }
            None => {
                self.effects.insert(effect, Instant::now());
                self.create_shield();
            }
        }
    }

    fn handle_fire_powers(&mut self, effect: &Effect) {
        if let Some(time) = self.effects.get(effect) {
            if time.elapsed() <= POWER_COOLDOWN {
                match effect {
                    Effect::Double => self.handle_double(),
                    Effect::Triple => self.handle_triple(),
                    _ => (),
                }
            }
        }
    }

    pub fn player_fire(&mut self) {
        if self.last_attack.elapsed() >= ATTACK_COOLDOWN {
            self.player.shoot(bullet::Direction::Up);
            self.handle_fire_powers(&Effect::Double);
            self.handle_fire_powers(&Effect::Triple);
            self.last_attack = Instant::now();
        }
    }

    fn random_event(reference: f32) -> bool {
        let step = Uniform::new(0., 1.);
        let mut rng = rand::thread_rng();
        let choice = step.sample(&mut rng);
        choice <= reference
    }

    pub fn enemy_fire(&mut self) {
        for enemy in self.enemies.iter_mut() {
            if Self::random_event(FIRE_PROBABILITY) {
                enemy.shoot(bullet::Direction::Down);
            }
        }
    }

    pub fn hit_player(&self) -> bool {
        for enemy in self.enemies.iter() {
            for bullet in enemy.bullets() {
                if bullet.pos() == self.player.pos() {
                    return true;
                }
            }
        }
        false
    }

    pub fn hit_powers(&mut self) {
        let mut shields = false;
        for bullet in self.player.bullets() {
            self.powers.retain(|power| {
                if power.pos() != bullet.pos() {
                    true
                } else {
                    if *power.effect() == Effect::Shield {
                        shields = true;
                    } else {
                        self.effects.insert(*power.effect(), Instant::now());
                    }
                    false
                }
            });
        }
        if shields {
            self.handle_shield();
        }
    }

    pub fn hit_enemies(&mut self) -> usize {
        let previous_size = self.enemies.len();
        let enemies_copy = self.enemies.to_vec();
        for bullet in self.player.bullets().iter() {
            self.enemies.retain(|enemy| enemy.pos() != bullet.pos());
        }
        for enemy in enemies_copy.iter() {
            self.player
                .bullets_mut()
                .retain(|bullet| bullet.pos() != enemy.pos());
        }
        let new_size = self.enemies.len();
        previous_size - new_size
    }

    pub fn hit_boss(&mut self) -> bool {
        if let Some(boss) = self.boss {
            for bullet in self.player.bullets() {
                if bullet.pos() == (2, boss.left_pos()) || bullet.pos() == (2, boss.left_pos() + 1)
                {
                    self.boss = None;
                    return true;
                }
            }
        }
        false
    }

    pub fn hit_shields(&mut self) {
        if let Some(time) = self.effects.get(&Effect::Shield) {
            if time.elapsed() > POWER_COOLDOWN {
                self.shields.clear();
            } else {
                for enemy in &self.enemies {
                    for bullet in enemy.bullets() {
                        for shield in self.shields.iter_mut() {
                            if bullet.pos() == shield.pos() {
                                shield.damage();
                            }
                        }
                    }
                }
                for shield in &self.shields {
                    for enemy in self.enemies.iter_mut() {
                        enemy
                            .bullets_mut()
                            .retain(|bullet| bullet.pos() != shield.pos());
                    }
                }
                self.shields.retain(|shield| shield.lives() > 0);
            }
        }
    }

    pub fn move_boss(&mut self) {
        if let Some(boss) = self.boss.as_mut() {
            boss.shift();
            if boss.left_pos() == self.width - 2 {
                self.boss = None;
            }
        }
    }

    pub fn move_player(&mut self, direction: i32) {
        let pos = self.player.pos();
        let new_pos = if direction == ncurses::KEY_LEFT {
            (pos.0, pos.1 - 1)
        } else {
            (pos.0, pos.1 + 1)
        };
        if !util::out_of_bounds(new_pos) {
            self.player.set_pos(new_pos)
        }
    }

    pub fn move_bullets(&mut self) {
        for bullet in self.player.bullets_mut() {
            bullet.shift();
        }
        self.player.clear_bullets();

        for enemy in self.enemies.iter_mut() {
            for bullet in enemy.bullets_mut() {
                bullet.shift();
            }
            enemy.clear_bullets();
        }
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
        let (left, right) = self.get_horizontal_indexes();

        if self.dir == Direction::Left && self.enemies[right].pos().1 == self.width - 2
            || self.enemies[left].pos().1 == 1 && self.dir == Direction::Right
        {
            self.dir = Direction::Down;
        } else if self.dir == Direction::Down && self.enemies[right].pos().1 == self.width - 2 {
            self.dir = Direction::Right;
        } else if self.dir == Direction::Down && self.enemies[left].pos().1 == 1 {
            self.dir = Direction::Left;
        }

        for enemy in self.enemies.iter_mut() {
            let previous = enemy.pos();
            let new_pos = match self.dir {
                Direction::Left => (previous.0, previous.1 + 1),
                Direction::Right => (previous.0, previous.1 - 1),
                Direction::Down => (previous.0 + 1, previous.1),
            };
            enemy.set_pos(new_pos);
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
}
