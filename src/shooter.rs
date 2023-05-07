use crate::{bullet::Bullet, direction::Direction, logic::COLOR_LASER, object::Object, util};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Shooter {
    pos: (i32, i32),
    char: u32,
    color: i16,
    bullets: VecDeque<Bullet>,
    is_mind_controlled: bool,
    is_numb: bool,
    revert: bool,
}

impl Object for Shooter {
    fn pos(&self) -> (i32, i32) {
        self.pos
    }
    fn char(&self) -> u32 {
        self.char
    }
    fn color(&self) -> i16 {
        self.color
    }
}

impl Shooter {
    pub fn new(pos: (i32, i32), char: u32, color: i16) -> Self {
        Self {
            pos,
            char,
            color,
            bullets: VecDeque::new(),
            is_mind_controlled: false,
            is_numb: false,
            revert: false,
        }
    }

    pub fn new_pos(&self, dir: &Direction) -> (i32, i32) {
        util::shift(&self.pos, dir)
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

    pub fn shoot(&mut self, mut dir: Direction, is_explosive: bool, char: u32, color: i16) {
        let pos = self.pos();
        if self.revert {
            dir = self.revert(&dir);
        }
        self.bullets.push_back(Bullet::new(pos, dir, char, color));
        self.handle_explosive(&is_explosive);
    }

    pub fn shoot_pos(
        &mut self,
        pos: &(i32, i32),
        mut dir: Direction,
        is_explosive: bool,
        char: u32,
        color: i16,
    ) {
        if self.revert {
            dir = self.revert(&dir);
        }
        self.bullets.push_back(Bullet::new(*pos, dir, char, color));
        self.handle_explosive(&is_explosive);
    }

    fn handle_explosive(&mut self, is_explosive: &bool) {
        if *is_explosive {
            self.bullets
                .iter_mut()
                .last()
                .expect("Handle explosives has been called without any explosive")
                .set_is_explosive(true);
        }
    }

    pub fn is_mind_controlled(&self) -> bool {
        self.is_mind_controlled
    }

    pub fn mind_control(&mut self) {
        self.is_mind_controlled = true;
    }

    pub fn set_x(&mut self, x: i32) {
        self.pos.0 = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.pos.1 = y;
    }

    pub fn revert(&mut self, dir: &Direction) -> Direction {
        match *dir {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::LeftDown => Direction::RightUp,
            Direction::LeftUp => Direction::RightDown,
            Direction::RightDown => Direction::LeftUp,
            Direction::RightUp => Direction::LeftDown,
        }
    }

    pub fn set_revert(&mut self, revert: bool) {
        self.revert = revert;
    }

    pub fn is_numb(&self) -> bool {
        self.is_numb
    }

    pub fn set_numb(&mut self) {
        self.is_numb = true;
        self.color = COLOR_LASER;
    }
}
