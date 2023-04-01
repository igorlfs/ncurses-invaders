use super::{handle::Handle, Logic};
use crate::{direction::Direction, power::Effect, shooter::Shooter, util};

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

    pub fn player(logic: &mut Logic, direction: &Direction) {
        let new_pos = logic.player.new_pos(direction);
        let warp = Handle::power(logic, &Effect::Warp) && util::out_of_bounds(new_pos);

        if *direction == Direction::Left && warp {
            logic.player.set_pos((logic.height - 2, logic.width - 2));
        } else if *direction == Direction::Right && warp {
            logic.player.set_pos((logic.height - 2, 1));
        } else {
            logic.player.shift(direction);
        }
    }

    pub fn follower(logic: &mut Logic) {
        let player_x = logic.player().pos().1;
        if let Some(follower) = logic.follower.as_mut() {
            follower.set_x(player_x);
        }
    }

    pub fn lasers(logic: &mut Logic) {
        for enemy in logic.enemies.iter_mut() {
            for bullet in enemy.bullets_mut() {
                bullet.shift();
            }
            enemy.clear_bullets();
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
    }

    pub fn enemies(logic: &mut Logic) -> bool {
        if !Handle::power(logic, &Effect::Lock) {
            let (left, right) = get_outermost_lateral_indexes(&logic.enemies);

            if logic.dir == Direction::Right && logic.enemies[right].pos().1 == logic.width - 2
                || logic.enemies[left].pos().1 == 1 && logic.dir == Direction::Left
            {
                logic.dir = Direction::Down;
            } else if logic.dir == Direction::Down
                && logic.enemies[right].pos().1 == logic.width - 2
            {
                logic.dir = Direction::Left;
            } else if logic.dir == Direction::Down && logic.enemies[left].pos().1 == 1 {
                logic.dir = Direction::Right;
            }

            if !(logic.dir == Direction::Down) || !Handle::power(logic, &Effect::Antigravity) {
                for enemy in logic.enemies.iter_mut() {
                    enemy.shift(&logic.dir);
                }
            }
        }

        if let Some(bottom) = logic.enemies.last() {
            bottom.pos().0 == logic.height - 2
        } else {
            false
        }
    }
}

fn get_outermost_lateral_indexes(shooters: &[Shooter]) -> (usize, usize) {
    let mut left_index = 0;
    let mut right_index = 0;
    for i in 1..shooters.len() {
        if shooters[i].pos().1 <= shooters[left_index].pos().1 {
            left_index = i;
        } else if shooters[i].pos().1 >= shooters[right_index].pos().1 {
            right_index = i;
        }
    }
    (left_index, right_index)
}
