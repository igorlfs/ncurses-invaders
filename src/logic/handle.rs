use crate::object::Object;
use crate::shooter::Shooter;
use crate::util;
use crate::{direction::Direction, power::Effect};
use std::collections::HashMap;
use std::time::Instant;

use super::{
    Logic, ATTACK_COOLDOWN, CHAR_BULLET, CHAR_ULTRA, COLOR_BULLET, COLOR_ULTRA,
    COMBINED_ATTACK_COOLDOWN, DOUBLE_ATTACK_COOLDOWN, POWER_COOLDOWN, TRIPLE_ATTACK_COOLDOWN,
};

pub struct Handle;

impl Handle {
    pub fn power(effects: &HashMap<Effect, Instant>, effect: &Effect) -> bool {
        if let Some(time) = effects.get(effect) {
            if time.elapsed() < POWER_COOLDOWN {
                return true;
            }
        }
        false
    }

    pub fn kamizake(logic: &mut Logic) -> bool {
        if Handle::power(&logic.effects, &Effect::Kamizake) {
            let (height, dir) = if Handle::power(&logic.effects, &Effect::Jump) {
                (3, &Direction::Down)
            } else {
                (logic.height - 2, &Direction::Up)
            };
            let new_pos = logic.player.new_pos(dir);
            if util::out_of_bounds(new_pos) {
                logic.player.set_x(height);
                return true;
            } else {
                logic.player.shift(dir);
                let mut exterminate = false;
                for enemy in &logic.enemies {
                    if enemy.pos() == logic.player.pos() {
                        exterminate = true;
                    }
                }
                if exterminate {
                    logic.enemies.clear();
                    logic.player.set_x(height);
                }
                return exterminate;
            }
        }
        false
    }

    pub fn mind_control(logic: &mut Logic) {
        if !Handle::power(&logic.effects, &Effect::Mindcontrol) {
            logic.enemies.retain(|enemy| !enemy.is_mind_controlled())
        }
    }

    fn double(logic: &mut Logic) {
        if Handle::power(&logic.effects, &Effect::Double) {
            logic.cooldown_attack = DOUBLE_ATTACK_COOLDOWN;
            let player_pos = logic.player.pos();
            let pos = (player_pos.0 - 1, player_pos.1);
            logic.player.shoot_pos(
                &pos,
                Direction::Up,
                Handle::power(&logic.effects, &Effect::Grenade),
                CHAR_BULLET,
                COLOR_BULLET,
            );
        }
    }

    fn triple(logic: &mut Logic) {
        if Handle::power(&logic.effects, &Effect::Triple) {
            logic.cooldown_attack = TRIPLE_ATTACK_COOLDOWN;
            let player_pos = logic.player.pos();
            let pos_left = (player_pos.0 - 1, player_pos.1 + 1);
            let grenade = Handle::power(&logic.effects, &Effect::Grenade);
            logic.player.shoot_pos(
                &pos_left,
                Direction::LeftUp,
                grenade,
                CHAR_BULLET,
                COLOR_BULLET,
            );
            let pos_right = (player_pos.0 - 1, player_pos.1 - 1);
            logic.player.shoot_pos(
                &pos_right,
                Direction::RightUp,
                grenade,
                CHAR_BULLET,
                COLOR_BULLET,
            );
        }
    }

    pub fn jump(logic: &mut Logic) {
        if Handle::power(&logic.effects, &Effect::Jump) {
            logic.player.set_x(3);
        } else if !Handle::power(&logic.effects, &Effect::Kamizake) {
            logic.player.set_x(logic.height - 2);
        }
    }

    pub fn yields(logic: &mut Logic) {
        logic.yield_counter -= 1;

        // We need to store the previous direction so we can restore
        if logic.dir != Direction::Up {
            logic.last_dir = Some(logic.dir);
        }

        logic.dir = Direction::Up;

        // Restore previous direction
        if logic.yield_counter == 0 {
            logic.dir = logic
                .last_dir
                .expect("Last direction is empty but has been called");
        }
    }

    pub fn explode(bomb: &(i32, i32), enemies: &mut Vec<Shooter>) {
        enemies.retain(|enemy| {
            let pos = enemy.pos();
            !((pos.0 >= bomb.0 - 2 && pos.0 <= bomb.0 + 2)
                && (pos.1 >= bomb.1 - 2 && pos.1 <= bomb.1 + 2))
        })
    }

    pub fn clear(enemies: &mut [Shooter]) {
        for enemy in enemies.iter_mut() {
            enemy.bullets_mut().clear();
        }
    }

    pub fn ultra(player: &mut Shooter, effects: &HashMap<Effect, Instant>, height: &i32) {
        let player_pos = player.pos();
        // When jumping, direction is reversed so we don't need to worry about changing direction,
        // only the range
        let range = if Handle::power(effects, &Effect::Jump) {
            player_pos.0 + 1..height - 1
        } else {
            3..player_pos.0
        };
        for i in range {
            player.shoot_pos(
                &(i, player_pos.1),
                Direction::Up,
                false,
                CHAR_ULTRA,
                COLOR_ULTRA,
            );
        }
    }

    pub fn attack(logic: &mut Logic) {
        if let Some(xerox) = &logic.xerox {
            let dir = if Handle::power(&logic.effects, &Effect::Jump) {
                Direction::Down
            } else {
                Direction::Up
            };
            logic
                .player
                .shoot_pos(&xerox.pos(), dir, false, CHAR_BULLET, COLOR_BULLET);
        }
        Handle::double(logic);
        Handle::triple(logic);
        let double = Handle::power(&logic.effects, &Effect::Double);
        let triple = Handle::power(&logic.effects, &Effect::Triple);
        if double && triple {
            logic.cooldown_attack = COMBINED_ATTACK_COOLDOWN;
        }
        if !double && !triple {
            logic.cooldown_attack = ATTACK_COOLDOWN;
        }
    }
}
