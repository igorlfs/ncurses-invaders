use super::{
    handle::Handle, Logic, BOSS_PROPABILITY, ENEMIES_PER_ROW, ENEMY_ROWS, FIRE_PROBABILITY,
    POWER_PROBABILITY, SHIELDS,
};
use crate::{
    boss::Boss,
    direction::Direction,
    power::{Effect, PowerUp},
    shield::Shield,
    shooter::Shooter,
    util,
};
use rand::Rng;

pub struct Generate;

impl Generate {
    pub fn enemies(logic: &mut Logic) {
        for j in 0..ENEMY_ROWS {
            for i in 0..ENEMIES_PER_ROW {
                logic.enemies.push(Shooter::new((2 * (j + 1), 2 * i + 1)));
            }
        }
    }

    pub fn shields(logic: &mut Logic) {
        for i in 1..SHIELDS {
            logic
                .shields
                .push(Shield::new((logic.height - 3, 3 * i - 1), 3))
        }
    }

    pub fn boss(logic: &mut Logic) {
        if util::random_event(BOSS_PROPABILITY) && logic.boss.is_none() {
            logic.boss = Some(Boss::new(0));
        }
    }

    pub fn power(logic: &mut Logic) {
        if util::random_event(POWER_PROBABILITY) {
            let mut rng = rand::thread_rng();
            let y = rng.gen_range(2..logic.height - 2);
            let x = rng.gen_range(1..logic.width - 1);
            logic.powers.push(PowerUp::new((y, x), rand::random()));
        }
    }

    pub fn enemy_attack(logic: &mut Logic) {
        if !Handle::power(logic, &Effect::Hijack) {
            for enemy in logic.enemies.iter_mut() {
                if util::random_event(FIRE_PROBABILITY) {
                    enemy.shoot(Direction::Down);
                }
            }
        }
    }
}
