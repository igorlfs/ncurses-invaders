use crate::{
    direction::Direction,
    logic::{Logic, COLOR_OBSTACLES, COLOR_POWERS, COLOR_SHIELDS},
    printer::Printer,
};
use ncurses::{box_, keypad, leaveok, wgetch, KEY_LEFT, KEY_RIGHT, WINDOW};
use std::time::{Duration, Instant};

const MAX_SHIPS: i8 = 3;
const UPDATE_COOLDOWN: Duration = Duration::from_millis(50);

pub struct Invaders {
    ships: i8,
    level: i32,
    input: i32,
    score: i32,
    gate: Logic,
    last_update: Instant,
    window: WINDOW,
}

impl Invaders {
    pub fn new(win: WINDOW) -> Self {
        Self {
            ships: MAX_SHIPS,
            level: 0,
            input: 0,
            score: 0,
            last_update: Instant::now(),
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
            self.gate.level_up(&mut self.level);
            if self.ships < MAX_SHIPS {
                self.ships += 1;
            }
        }

        if self.input == ' ' as i32 {
            self.gate.player_shoot();
        } else if self.input == KEY_RIGHT {
            self.gate.player_move(&Direction::Right);
        } else if self.input == KEY_LEFT {
            self.gate.player_move(&Direction::Left);
        } else if self.input == 'q' as i32 {
            self.ships = -1;
        }

        if self.last_update.elapsed() >= UPDATE_COOLDOWN {
            self.gate.generate();
            if self.gate.shift(&self.level) {
                self.ships = -1;
            }
            if self.gate.hit(&self.level) {
                self.ships -= 1;
            }
            self.score += self.gate.score_increment();
            self.gate.score_reset();
            self.last_update = Instant::now();
        }
    }

    fn print(&self) {
        Printer::clear(self.window);
        Printer::header(self.score, self.window, self.ships);
        let enemies = self.gate.enemies();
        Printer::shooters(self.window, enemies);
        let player = self.gate.player();
        Printer::shooter(self.window, player);
        let powers = self.gate.powers();
        Printer::objects(self.window, powers, COLOR_POWERS);
        let shields = self.gate.shields();
        Printer::objects(self.window, shields, COLOR_SHIELDS);
        let obstacles = self.gate.obstacles();
        Printer::objects(self.window, obstacles, COLOR_OBSTACLES);
        if let Some(xerox) = self.gate.xerox() {
            Printer::shooter(self.window, xerox);
        }
        if let Some(follower) = self.gate.follower() {
            Printer::object(self.window, follower);
        }
        if let Some(boss) = self.gate.boss() {
            Printer::object(self.window, boss);
        }
        Printer::footer(self.gate.active_effects());
    }

    fn is_over(&self) -> bool {
        self.ships <= -1
    }

    fn quit(&self) {
        Printer::quit(self.score);
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
