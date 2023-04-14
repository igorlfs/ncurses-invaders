use super::{
    handle::Handle, Logic, BOSS_PROPABILITY, ENEMIES_PER_ROW, ENEMY_ROWS, FIRE_PROBABILITY,
    OBSTACLES, POWER_PROBABILITY, SHIELDS,
};
use super::{
    CHAR_BULLET, CHAR_ENEMY, CHAR_FOLLOWER, CHAR_LASER, CHAR_OBSTACLE, CHAR_PLAYER, CHAR_SHIELD,
    COLOR_ALLY, COLOR_BULLET, COLOR_ENEMY, COLOR_FOLLOWER, COLOR_LASER, COLOR_OBSTACLES,
    COLOR_SHIELDS,
};
use crate::object::Object;
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
                logic.enemies.push(Shooter::new(
                    (2 * (j + 2), 2 * i + 1),
                    CHAR_ENEMY,
                    COLOR_ENEMY,
                ));
            }
        }
    }

    pub fn shields(logic: &mut Logic) {
        for i in 1..SHIELDS {
            logic.shields.push(Shield::new(
                (logic.height - 3, 3 * i - 1),
                3,
                COLOR_SHIELDS,
                CHAR_SHIELD,
            ))
        }
    }

    pub fn obstacles(logic: &mut Logic) {
        let obstacle = Handle::power(logic, &Effect::Obstacle);
        if obstacle && logic.obstacles.is_empty() {
            for i in 0..OBSTACLES {
                logic.obstacles.push(Shield::new(
                    (logic.height - (4 + 2 * i), logic.width / 2),
                    3,
                    COLOR_OBSTACLES,
                    CHAR_OBSTACLE,
                ));
            }
        }
    }

    pub fn xerox(logic: &mut Logic) {
        let xerox = Handle::power(logic, &Effect::Xerox);
        if logic.xerox.is_none() && xerox {
            let player_pos = logic.player.pos();
            let y = logic.width - player_pos.1;
            logic.xerox = Some(Shooter::new((player_pos.0, y), CHAR_PLAYER, COLOR_ALLY));
            if let Some(xerox) = &mut logic.xerox {
                xerox.mind_control();
            }
        }
        if !xerox {
            logic.xerox = None
        }
    }

    pub fn follower(logic: &mut Logic) {
        let player_x = logic.player().pos().1;
        logic.follower = Some(Shield::new(
            (logic.height - 3, player_x),
            1,
            COLOR_FOLLOWER,
            CHAR_FOLLOWER,
        ));
    }

    pub fn boss(logic: &mut Logic) {
        if util::random_event(BOSS_PROPABILITY) && logic.boss.is_none() {
            logic.boss = Some(Boss::new((2, 1)));
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
                    if enemy.is_mind_controlled() {
                        logic.player.shoot_pos(
                            &enemy.pos(),
                            rand::random(),
                            false,
                            CHAR_BULLET,
                            COLOR_BULLET,
                        );
                    } else {
                        enemy.shoot(Direction::Down, false, CHAR_LASER, COLOR_LASER);
                    }
                }
            }
        }
    }
}
