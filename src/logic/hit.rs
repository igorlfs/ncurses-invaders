use super::{
    generate::Generate, handle::Handle, Logic, BOSS_SCORE, CHAR_LASER, COLOR_LASER, ENEMY_SCORE,
    POWER_COOLDOWN,
};
use crate::object::Object;
use crate::{bullet::Bullet, power::Effect};
use std::time::{Duration, Instant};

pub struct Hit;

impl Hit {
    pub fn player(logic: &mut Logic) -> bool {
        let enemies_copy = logic.enemies.to_vec();

        if Handle::power(logic, &Effect::Vendetta) {
            logic.enemies.retain(|enemy| {
                let mut retain = true;
                for bullet in enemy.bullets() {
                    if bullet.pos() == logic.player.pos() {
                        retain = false;
                    }
                }
                retain
            })
        }

        if !Handle::power(logic, &Effect::Invincible) {
            for enemy in enemies_copy {
                for bullet in enemy.bullets() {
                    if bullet.pos() == logic.player.pos() {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn obstacles(logic: &mut Logic) {
        if let Some(time) = logic.effects.get(&Effect::Obstacle) {
            if time.elapsed() > POWER_COOLDOWN {
                logic.obstacles.clear();
            } else {
                for enemy in &logic.enemies {
                    for obstacle in logic.obstacles.iter_mut() {
                        if enemy.pos() == obstacle.pos() {
                            obstacle.damage();
                        }
                    }
                }
                for obstacle in &logic.obstacles {
                    logic.enemies.retain(|enemy| enemy.pos() != obstacle.pos());
                }
                logic.obstacles.retain(|shield| shield.is_alive());
            }
        }
    }

    pub fn powers(logic: &mut Logic) {
        for bullet in logic.player.bullets() {
            logic.powers.retain(|power| {
                if power.pos() != bullet.pos() {
                    true
                } else {
                    if *power.effect() == Effect::Clear {
                        for enemy in logic.enemies.iter_mut() {
                            enemy.bullets_mut().clear();
                        }
                    } else {
                        logic.effects.insert(*power.effect(), Instant::now());
                    }
                    false
                }
            });
        }
    }

    pub fn shields(logic: &mut Logic) {
        if let Some(time) = logic.effects.get(&Effect::Shield) {
            if time.elapsed() > POWER_COOLDOWN {
                logic.shields.clear();
            } else {
                for enemy in &logic.enemies {
                    for bullet in enemy.bullets() {
                        for shield in logic.shields.iter_mut() {
                            if bullet.pos() == shield.pos() {
                                shield.damage();
                            }
                        }
                    }
                }
                for shield in &logic.shields {
                    for enemy in logic.enemies.iter_mut() {
                        enemy
                            .bullets_mut()
                            .retain(|bullet| bullet.pos() != shield.pos());
                    }
                }
                logic.shields.retain(|shield| shield.is_alive());
            }
        }
    }

    pub fn follower(logic: &mut Logic) {
        if let Some(follower) = logic.follower.as_mut() {
            for enemy in &logic.enemies {
                for bullet in enemy.bullets() {
                    if bullet.pos() == follower.pos() {
                        follower.damage();
                    }
                }
            }
            for enemy in logic.enemies.iter_mut() {
                enemy
                    .bullets_mut()
                    .retain(|bullet| bullet.pos() != follower.pos());
            }
            if !follower.is_alive() {
                logic.follower = None;
                logic
                    .effects
                    .insert(Effect::Follower, Instant::now() - Duration::from_secs(10));
            }
        }
    }

    pub fn boss(logic: &mut Logic) -> bool {
        if let Some(boss) = logic.boss {
            for bullet in logic.player.bullets() {
                if bullet.pos() == boss.pos() {
                    logic.boss = None;
                    return true;
                }
            }
        }
        false
    }

    pub fn enemies(logic: &mut Logic) -> usize {
        let previous_size = logic.enemies.len();
        let enemies_copy = logic.enemies.to_vec();
        let mind_control = Handle::power(logic, &Effect::Mindcontrol);
        let player_bullets_copy = logic.player.bullets().clone();
        let mut exploding_bullets: Vec<Bullet> = vec![];
        for bullet in player_bullets_copy {
            for enemy in logic.enemies_mut() {
                if enemy.pos() == bullet.pos() && mind_control {
                    enemy.mind_control();
                }
            }
            if !mind_control {
                logic.enemies.retain(|enemy| enemy.pos() != bullet.pos());
            }
        }

        if !Handle::power(logic, &Effect::Pierce) {
            for enemy in enemies_copy {
                logic.player.bullets_mut().retain(|bullet| {
                    if bullet.pos() == enemy.pos() {
                        exploding_bullets.push(bullet.clone());
                        false
                    } else {
                        true
                    }
                });
            }
        }

        for bullet in exploding_bullets {
            if bullet.is_explosive() {
                logic.player.shoot_pos(
                    &bullet.pos(),
                    rand::random(),
                    false,
                    CHAR_LASER,
                    COLOR_LASER,
                );
            }
        }

        let new_size = logic.enemies.len();
        previous_size - new_size
    }

    pub fn lasers(logic: &mut Logic) {
        let enemies_copy = logic.enemies.to_vec();

        for bullet in logic.player.bullets_mut() {
            for enemy in logic.enemies.iter_mut() {
                enemy
                    .bullets_mut()
                    .retain(|laser| laser.pos() != bullet.pos());
            }
        }

        for enemy in enemies_copy {
            for laser in enemy.bullets() {
                logic
                    .player
                    .bullets_mut()
                    .retain(|bullet| bullet.pos() != laser.pos())
            }
        }
    }

    pub fn moving(logic: &mut Logic, level: &i32) {
        if Handle::power(logic, &Effect::Block) {
            Hit::lasers(logic);
        }
        if Hit::boss(logic) {
            logic.score_increment += BOSS_SCORE * level;
        }
        logic.score_increment += (Hit::enemies(logic) as i32) * ENEMY_SCORE * level;
    }
}
