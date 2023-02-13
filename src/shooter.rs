use std::collections::VecDeque;

use crate::bullet::Bullet;

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

    pub fn set_pos(&mut self, pos: (i32, i32)) {
        self.pos = pos;
    }

    pub fn bullets_mut(&mut self) -> &mut VecDeque<Bullet> {
        &mut self.bullets
    }

    pub fn bullets(&self) -> &VecDeque<Bullet> {
        &self.bullets
    }

    pub fn clear_bullets(&mut self) {
        loop {
            if self.bullets.is_empty() {
                return;
            }
            if self.bullets[0].pos() == Bullet::UNDEFINED {
                self.bullets.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn shoot(&mut self) {
        let pos = self.pos();
        self.bullets.push_back(Bullet::new(pos));
    }
}
