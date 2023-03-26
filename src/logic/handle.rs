use crate::{direction::Direction, power::Effect};

use super::{Logic, POWER_COOLDOWN};

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

    pub fn double(logic: &mut Logic) {
        let player_pos = logic.player.pos();
        let pos = (player_pos.0 - 1, player_pos.1);
        logic.player.shoot_pos(&pos, Direction::Up);
    }

    pub fn triple(logic: &mut Logic) {
        let player_pos = logic.player.pos();
        let pos_left = (player_pos.0 - 1, player_pos.1 + 1);
        logic.player.shoot_pos(&pos_left, Direction::LeftUp);
        let pos_right = (player_pos.0 - 1, player_pos.1 - 1);
        logic.player.shoot_pos(&pos_right, Direction::RightUp);
    }
}
