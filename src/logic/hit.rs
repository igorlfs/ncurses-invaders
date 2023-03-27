use std::time::Instant;

use crate::power::Effect;

use super::{generate::Generate, handle::Handle, Logic, POWER_COOLDOWN};

pub struct Hit;

impl Hit {
    pub fn player(logic: &Logic) -> bool {
        for enemy in logic.enemies.iter() {
            for bullet in enemy.bullets() {
                if bullet.pos() == logic.player.pos() {
                    return true;
                }
            }
        }
        false
    }

    pub fn powers(logic: &mut Logic) {
        let mut shields = false;
        let mut clear = false;
        let mut quick = false;
        for bullet in logic.player.bullets() {
            logic.powers.retain(|power| {
                if power.pos() != bullet.pos() {
                    true
                } else {
                    let effect = *power.effect();
                    if effect == Effect::Clear {
                        clear = true;
                    } else {
                        if effect == Effect::Shield {
                            shields = true;
                        } else if effect == Effect::QuickShot {
                            quick = true;
                        }
                        logic.effects.insert(*power.effect(), Instant::now());
                    }
                    false
                }
            });
        }
        if Handle::power(logic, &Effect::Shield) && shields {
            Generate::shields(logic);
        }
        if Handle::power(logic, &Effect::QuickShot) && quick {
            logic.cooldown_attack /= 2;
        }
        if clear {
            for enemy in logic.enemies.iter_mut() {
                enemy.bullets_mut().clear();
            }
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
                logic.shields.retain(|shield| shield.lives() > 0);
            }
        }
    }

    pub fn boss(logic: &mut Logic) -> bool {
        if let Some(boss) = logic.boss {
            for bullet in logic.player.bullets() {
                if bullet.pos() == (2, boss.left_pos()) || bullet.pos() == (2, boss.left_pos() + 1)
                {
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
        for bullet in logic.player.bullets().iter() {
            logic.enemies.retain(|enemy| enemy.pos() != bullet.pos());
        }

        if !Handle::power(logic, &Effect::Pierce) {
            for enemy in enemies_copy {
                logic
                    .player
                    .bullets_mut()
                    .retain(|bullet| bullet.pos() != enemy.pos());
            }
        }
        let new_size = logic.enemies.len();
        previous_size - new_size
    }
}