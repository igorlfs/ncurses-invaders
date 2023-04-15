use crate::object::Object;
use crate::{direction::Direction, power::Effect};

use super::{
    Logic, ATTACK_COOLDOWN, CHAR_BULLET, COLOR_BULLET, COMBINED_ATTACK_COOLDOWN,
    DOUBLE_ATTACK_COOLDOWN, POWER_COOLDOWN, TRIPLE_ATTACK_COOLDOWN,
};

pub struct Handle;

impl Handle {
    pub fn power(logic: &Logic, effect: &Effect) -> bool {
        if let Some(time) = logic.effects.get(effect) {
            if time.elapsed() < POWER_COOLDOWN {
                return true;
            }
        }
        false
    }

    pub fn mind_control(logic: &mut Logic) {
        if !Handle::power(logic, &Effect::Mindcontrol) {
            logic.enemies.retain(|enemy| !enemy.is_mind_controlled())
        }
    }

    fn double(logic: &mut Logic) {
        if !Handle::power(logic, &Effect::Double) {
            logic.cooldown_attack = DOUBLE_ATTACK_COOLDOWN;
            let player_pos = logic.player.pos();
            let pos = (player_pos.0 - 1, player_pos.1);
            logic.player.shoot_pos(
                &pos,
                Direction::Up,
                Handle::power(logic, &Effect::Grenade),
                CHAR_BULLET,
                COLOR_BULLET,
            );
        }
    }

    fn triple(logic: &mut Logic) {
        if !Handle::power(logic, &Effect::Triple) {
            logic.cooldown_attack = TRIPLE_ATTACK_COOLDOWN;
            let player_pos = logic.player.pos();
            let pos_left = (player_pos.0 - 1, player_pos.1 + 1);
            let grenade = Handle::power(logic, &Effect::Grenade);
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
        let player_pos = logic.player.pos();
        if Handle::power(logic, &Effect::Jump) {
            logic.player.set_pos((3, player_pos.1));
        } else {
            logic.player.set_pos((logic.height - 2, player_pos.1));
        }
    }

    pub fn attack(logic: &mut Logic) {
        if let Some(xerox) = &logic.xerox {
            let dir = if Handle::power(logic, &Effect::Jump) {
                Direction::Down
            } else {
                Direction::Up
            };
            logic
                .player
                .shoot_pos(&xerox.pos(), dir, false, CHAR_BULLET, COLOR_BULLET);
        }
        let double = Handle::power(logic, &Effect::Double);
        let triple = Handle::power(logic, &Effect::Triple);
        Handle::double(logic);
        Handle::triple(logic);
        if double && triple {
            logic.cooldown_attack = COMBINED_ATTACK_COOLDOWN;
        } else {
            logic.cooldown_attack = ATTACK_COOLDOWN;
        }
    }
}
