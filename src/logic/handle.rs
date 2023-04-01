use crate::{direction::Direction, power::Effect};

use super::{
    Logic, ATTACK_COOLDOWN, COMBINED_ATTACK_COOLDOWN, DOUBLE_ATTACK_COOLDOWN, POWER_COOLDOWN,
    TRIPLE_ATTACK_COOLDOWN,
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
        logic.cooldown_attack = DOUBLE_ATTACK_COOLDOWN;
        let player_pos = logic.player.pos();
        let pos = (player_pos.0 - 1, player_pos.1);
        logic
            .player
            .shoot_pos(&pos, Direction::Up, Handle::power(logic, &Effect::Grenade));
    }

    fn triple(logic: &mut Logic) {
        logic.cooldown_attack = TRIPLE_ATTACK_COOLDOWN;
        let player_pos = logic.player.pos();
        let pos_left = (player_pos.0 - 1, player_pos.1 + 1);
        let grenade = Handle::power(logic, &Effect::Grenade);
        logic
            .player
            .shoot_pos(&pos_left, Direction::LeftUp, grenade);
        let pos_right = (player_pos.0 - 1, player_pos.1 - 1);
        logic
            .player
            .shoot_pos(&pos_right, Direction::RightUp, grenade);
    }

    pub fn attack(logic: &mut Logic) {
        if let Some(xerox) = &logic.xerox {
            logic.player.shoot_pos(&xerox.pos(), Direction::Up, false);
        }
        let double = Handle::power(logic, &Effect::Double);
        let triple = Handle::power(logic, &Effect::Triple);
        if double {
            Handle::double(logic);
        }
        if triple {
            Handle::triple(logic);
        }
        if double && triple {
            logic.cooldown_attack = COMBINED_ATTACK_COOLDOWN;
        }
        if !double && !triple && !Handle::power(logic, &Effect::Quickshot) {
            logic.cooldown_attack = ATTACK_COOLDOWN;
        }
    }
}
