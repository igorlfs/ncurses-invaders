use std::collections::VecDeque;

use crate::{bullet::Bullet, direction::Direction, util};

#[derive(Clone)]
pub struct Shooter {
    pos: (i32, i32),
    bullets: VecDeque<Bullet>,
}

impl Shooter {
    pub fn new(pos: (i32, i32)) -> Self {
        Self {
            pos,
            bullets: VecDeque::new(),
        }
    }

    pub fn pos(&self) -> (i32, i32) {
        self.pos
    }

    pub fn shift(&mut self, dir: &Direction) {
        let new_pos = util::shift(&self.pos, dir);
        if !util::out_of_bounds(new_pos) {
            self.pos = new_pos;
        }
    }

    pub fn bullets_mut(&mut self) -> &mut VecDeque<Bullet> {
        &mut self.bullets
    }

    pub fn bullets(&self) -> &VecDeque<Bullet> {
        &self.bullets
    }

    pub fn clear_bullets(&mut self) {
        self.bullets
            .retain(|bullet| !util::out_of_bounds(bullet.pos()))
    }

    pub fn shoot(&mut self, dir: Direction) {
        let pos = self.pos();
        self.bullets.push_back(Bullet::new(pos, dir));
    }

    pub fn shoot_pos(&mut self, pos: &(i32, i32), dir: Direction) {
        self.bullets.push_back(Bullet::new(*pos, dir));
    }
}
