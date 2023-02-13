use std::time::{Duration, Instant};

use ncurses::{getmaxx, getmaxy, WINDOW};
use rand::distributions::{Distribution, Uniform};

use crate::{bullet, shooter::Shooter};

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Down,
}

pub struct Logic {
    enemies: Vec<Shooter>,
    player: Shooter,
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

    pub fn level_up(&mut self) -> bool {
        let defeated_enemies = self.enemies.is_empty();
        if defeated_enemies {
            self.create_enemies();
        }
        defeated_enemies
    }

    pub fn player_fire(&mut self) {
        const COOLDOWN: Duration = Duration::from_millis(400);
        if self.last_attack.elapsed() >= COOLDOWN {
            self.player.shoot();
            self.last_attack = Instant::now();
        }
    }

    pub fn enemy_fire(&mut self) {
        const DONT_FIRE_PROBABILY: f32 = 0.95;

        let step = Uniform::new(0., 1.);
        let mut rng = rand::thread_rng();

        for enemy in self.enemies.iter_mut() {
            let choice = step.sample(&mut rng);
            if choice > DONT_FIRE_PROBABILY {
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

    pub fn hit_enemies(&mut self) -> usize {
        let previous_size = self.enemies.len();
        for bullet in self.player.bullets().iter() {
            self.enemies.retain(|enemy| enemy.pos() != bullet.pos());
        }
        let new_size = self.enemies().len();
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

    fn get_indexes(&self) -> (usize, usize) {
        let mut left_index = 0;
        let mut right_index = 0;
        let enemies = self.enemies();
        for i in 1..enemies.len() {
            if enemies[left_index].pos().1 <= enemies[i].pos().1 {
                right_index = i;
            } else {
                left_index = i;
            }
        }
        (left_index, right_index)
    }

    pub fn move_enemies(&mut self) {
        let (left, right) = self.get_indexes();

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
}
