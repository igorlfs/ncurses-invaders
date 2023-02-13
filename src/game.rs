use ncurses::{box_, keypad, leaveok, wgetch, KEY_LEFT, KEY_RIGHT, WINDOW};

use crate::{logic::Logic, printer::Printer, shooter::Shooter};

pub struct Invaders {
    lives: i8,
    input: i32,
    score: i32,
    gate: Logic,
    window: WINDOW,
}

impl Invaders {
    pub fn new(win: WINDOW) -> Self {
        Self {
            lives: 3,
            input: 0,
            score: 0,
            window: win,
            gate: Logic::new(win),
        }
    }
    pub fn init(&mut self) {
        box_(self.window, 0, 0);
        leaveok(self.window, true);
        keypad(self.window, true);

        self.gate.create_enemies();
    }

    fn read_input(&mut self) {
        self.input = wgetch(self.window);
    }

    fn update(&mut self) {
        let previous = self.gate.player().pos();
        const QUIT: i32 = 'q' as i32;
        const ACTION: i32 = ' ' as i32;
        let player = self.gate.player_mut();
        match self.input {
            QUIT => self.lives = -1,
            KEY_LEFT => player.set_pos((previous.0, previous.1 - 1)),
            KEY_RIGHT => player.set_pos((previous.0, previous.1 + 1)),
            ACTION => self.gate.player_fire(),
            _ => (),
        };
        self.gate.enemy_fire();
        self.gate.move_enemies();
        self.gate.move_bullets();
        if self.gate.hit_player() {
            self.lives -= 1;
        }
        const MULTIPLIER: i32 = 20;
        self.score += (self.gate.hit_enemies() as i32) * MULTIPLIER;
        if self.gate.level_up() && self.lives < 3 {
            self.lives += 1;
        }
    }

    fn print(&self) {
        Printer::clear(self.window);
        Printer::header(self.score, self.window, self.lives);
        let enemies: &[Shooter] = self.gate.enemies();
        Printer::enemies(self.window, enemies);
        let player = self.gate.player();
        Printer::player(self.window, player);
    }

    fn is_over(&self) -> bool {
        self.lives <= -1
    }

    pub fn game_loop(&mut self) {
        loop {
            if self.is_over() {
                break;
            }
            self.print();
            self.read_input();
            self.update();
        }
    }
}
