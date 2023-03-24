use ncurses::{box_, delwin, keypad, leaveok, mvwaddstr, wgetch, KEY_LEFT, KEY_RIGHT, WINDOW};

use crate::{logic::Logic, printer::Printer, window};

const MAX_SHIPS: i8 = 3;
const MULTIPLIER: i32 = 20;

pub struct Invaders {
    ships: i8,
    level: i32,
    input: i32,
    score: i32,
    gate: Logic,
    window: WINDOW,
}

impl Invaders {
    pub fn new(win: WINDOW) -> Self {
        Self {
            ships: MAX_SHIPS,
            level: 0,
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
    }

    fn read_input(&mut self) {
        self.input = wgetch(self.window);
    }

    fn update(&mut self) {
        if self.gate.enemies().is_empty() {
            self.gate.create_enemies();
            self.level += 1;
            if self.ships < MAX_SHIPS {
                self.ships += 1;
            }
        }
        if self.input == ' ' as i32 {
            self.gate.player_fire();
        } else if self.input == KEY_RIGHT || self.input == KEY_LEFT {
            self.gate.move_player(self.input);
        }
        self.gate.create_power();
        self.gate.enemy_fire();
        if self.gate.move_enemies() || self.input == 'q' as i32 {
            self.ships = -1;
        };
        self.gate.move_bullets();
        self.gate.hit_powers();
        self.gate.hit_shields();
        if self.gate.hit_player() {
            self.ships -= 1;
        }
        self.score += (self.gate.hit_enemies() as i32) * MULTIPLIER * self.level;
    }

    fn print(&self) {
        Printer::clear(self.window);
        Printer::header(self.score, self.window, self.ships);
        let enemies = self.gate.enemies();
        Printer::enemies(self.window, enemies);
        let powers = self.gate.powers();
        Printer::powers(self.window, powers);
        let player = self.gate.player();
        Printer::player(self.window, player);
        let shields = self.gate.shields();
        Printer::shields(self.window, shields);
    }

    fn is_over(&self) -> bool {
        self.ships <= -1
    }

    fn quit(&self) {
        const LINES: i32 = 10;
        const COLS: i32 = 20;

        let quit_window = window::get_centralized_window(LINES, COLS);

        box_(quit_window, 0, 0);
        mvwaddstr(quit_window, 2, 5, "The Aliens");
        mvwaddstr(quit_window, 3, 8, "Have");
        mvwaddstr(quit_window, 4, 6, "INVADED!");
        let score_str = format!("Score {}", self.score);
        mvwaddstr(
            quit_window,
            7,
            (COLS - score_str.len() as i32) / 2,
            &score_str,
        );
        wgetch(quit_window);
        delwin(quit_window);
    }

    pub fn game_loop(&mut self) {
        loop {
            if self.is_over() {
                break;
            }
            self.update();
            self.print();
            self.read_input();
        }
        self.quit();
    }
}
