use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use ncurses::{getmaxx, getmaxy, WINDOW};
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crate::{
    bullet,
    power::{Effect, PowerUp},
    shooter::Shooter,
};

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Down,
}

pub struct Logic {
    enemies: Vec<Shooter>,
    player: Shooter,
    powers: Vec<PowerUp>,
    effects: HashMap<Effect, Instant>,
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
            player: Shooter::new((y - 2, x / 2)),
            height: y,
            width: x,
            dir: Direction::Left,
            last_attack: Instant::now(),
        }
    }

    pub fn create_enemies(&mut self) {
        const NUM_ENEMIES: i32 = 10;
        const ROWS: i32 = 4;

        for j in 0..ROWS {
            for i in 0..NUM_ENEMIES {
                self.enemies.push(Shooter::new((2 * (j + 1), 2 * i + 1)));
            }
        }
    }

    pub fn create_power(&mut self) {
        const POWER_PROBABILITY: f32 = 0.05;
        if Self::random_event(POWER_PROBABILITY) {
            let mut rng = rand::thread_rng();
            let y = rng.gen_range(2..self.height - 2);
            let x = rng.gen_range(1..self.width - 1);
            self.powers.push(PowerUp::new((y, x), Effect::Double));
        }
    }

    pub fn level_up(&mut self) -> bool {
        let defeated_enemies = self.enemies.is_empty();
        if defeated_enemies {
            self.create_enemies();
        }
        defeated_enemies
    }

    fn handle_double(&mut self) {
        let player_pos = self.player.pos();
        let pos = (player_pos.0 - 1, player_pos.1);
        self.player.shoot_pos(&pos);
    }

    fn handle_powers(&mut self) {
        const COOLDOWN: Duration = Duration::from_secs(10);
        for (power, time) in self.effects.clone().iter() {
            if time.elapsed() <= COOLDOWN {
                match power {
                    Effect::Double => self.handle_double(),
                }
            }
        }
    }

    pub fn player_fire(&mut self) {
        const COOLDOWN: Duration = Duration::from_millis(500);
        if self.last_attack.elapsed() >= COOLDOWN {
            self.player.shoot();
            self.handle_powers();
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
        const FIRE_PROBABILY: f32 = 0.05;

        for enemy in self.enemies.iter_mut() {
            if Self::random_event(FIRE_PROBABILY) {
                enemy.shoot();
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
        for bullet in self.player.bullets().iter() {
            self.powers.retain(|power| {
                if power.pos() != bullet.pos() {
                    true
                } else {
                    self.effects.insert(*power.effect(), Instant::now());
                    false
                }
            });
        }
    }

    pub fn hit_enemies(&mut self) -> usize {
        let previous_size = self.enemies.len();
        for bullet in self.player.bullets().iter() {
            self.enemies.retain(|enemy| enemy.pos() != bullet.pos());
        }
        let new_size = self.enemies.len();
        previous_size - new_size
    }

    pub fn move_bullets(&mut self) {
        for bullet in self.player.bullets_mut() {
            bullet.shift(&bullet::Direction::Up, &3);
        }
        self.player.clear_bullets();

        for enemy in self.enemies.iter_mut() {
            for bullet in enemy.bullets_mut() {
                bullet.shift(&bullet::Direction::Down, &(self.height - 3));
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

        let bottom = self.get_bottom();
        if bottom == self.height - 2 {
            return true;
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

        false
    }

    pub fn enemies(&self) -> &[Shooter] {
        self.enemies.as_ref()
    }

    pub fn player(&self) -> &Shooter {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Shooter {
        &mut self.player
    }

    pub fn powers(&self) -> &[PowerUp] {
        self.powers.as_ref()
    }
}
