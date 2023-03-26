use super::Logic;

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
        for bullet in logic.player.bullets_mut() {
            bullet.shift();
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
