use super::{handle::Handle, Logic};
use crate::{direction::Direction, power::Effect, util};

pub struct Move;

impl Move {
    pub fn boss(logic: &mut Logic) {
        if let Some(boss) = logic.boss.as_mut() {
            boss.shift();
            if boss.left_pos() == logic.width - 2 {
                logic.boss = None;
            }
        }
    }
    pub fn bullets(logic: &mut Logic) {
        let reflect = Handle::power(logic, &Effect::Reflect);
        for bullet in logic.player.bullets_mut() {
            bullet.shift();
            if util::out_of_bounds(bullet.pos()) && reflect {
                let new_dir = match bullet.dir() {
                    Direction::Up => Direction::Down,
                    Direction::LeftUp => Direction::RightUp,
                    Direction::RightUp => Direction::LeftUp,
                    _ => Direction::Up,
                };
                bullet.set_dir(new_dir);
                bullet.shift();
            }
        }
        logic.player.clear_bullets();

        for enemy in logic.enemies.iter_mut() {
            for bullet in enemy.bullets_mut() {
                bullet.shift();
            }
            enemy.clear_bullets();
        }
    }
}
