use crate::object::Object;
use crate::{power::Effect, shooter::Shooter, window, COLS, LINES};
use ncurses::*;

pub struct Printer;

impl Printer {
    pub fn clear(win: WINDOW) {
        let x = getmaxx(win);
        let y = getmaxy(win);

        for i in 1..x - 1 {
            for j in 1..y - 1 {
                mvwaddch(win, j, i, ' ' as u32);
            }
        }
    }

    pub fn header(score: i32, win: WINDOW, lives: i8) {
        mvwprintw(win, 1, 1, &format!("SCORE: {score}"));

        let mut live_str: String = "SHIPS: ".to_string();
        if lives > 0 {
            for _ in 0..lives {
                live_str += "*";
            }
        } else {
            live_str += "LAST";
        }
        let max_x = getmaxx(win);
        let x = max_x - (live_str.len() as i32) - 1;

        mvwprintw(win, 1, x, &live_str);
    }

    pub fn footer(effects: Vec<Effect>) {
        const MAX_STR_SIZE: i32 = COLS - 15;
        let window = window::get_mid_window(3, COLS, LINES + 13);
        box_(window, 0, 0);
        let mut effects_string = String::new();
        for effect in effects {
            effects_string += &format!("{effect},");
        }
        if !effects_string.is_empty() {
            effects_string.pop();
        }
        if effects_string.len() as i32 >= MAX_STR_SIZE {
            effects_string = effects_string.split_at(MAX_STR_SIZE as usize).0.to_string();
            effects_string += "...";
        }
        mvwprintw(window, 1, 1, &format!("PowerUps: {effects_string}"));
        wrefresh(window);
    }

    pub fn quit(score: i32) {
        const LINES: i32 = 10;
        const COLS: i32 = 20;

        let quit_window = window::get_centralized_window(LINES, COLS);

        box_(quit_window, 0, 0);
        mvwaddstr(quit_window, 2, 5, "The Aliens");
        mvwaddstr(quit_window, 3, 8, "Have");
        mvwaddstr(quit_window, 4, 6, "INVADED!");
        let score_str = format!("Score {}", score);
        mvwaddstr(
            quit_window,
            7,
            (COLS - score_str.len() as i32) / 2,
            &score_str,
        );
        wgetch(quit_window);
        delwin(quit_window);
    }

    pub fn objects<T>(win: WINDOW, objects: &[T], color: i16)
    where
        T: Object,
    {
        wattron(win, COLOR_PAIR(color));
        for power in objects {
            let pos = power.pos();
            mvwaddch(win, pos.0, pos.1, power.char());
        }
        wattroff(win, COLOR_PAIR(color));
    }

    pub fn object<T>(win: WINDOW, object: &T)
    where
        T: Object,
    {
        let pos = object.pos();
        wattron(win, COLOR_PAIR(object.color()));
        mvwaddch(win, pos.0, pos.1, object.char());
        wattroff(win, COLOR_PAIR(object.color()));
    }

    pub fn shooters(win: WINDOW, enemies: &[Shooter]) {
        for enemy in enemies {
            Printer::shooter(win, enemy);
        }
    }

    pub fn shooter(win: WINDOW, shooter: &Shooter) {
        let color = shooter.color();
        let pos = shooter.pos();
        wattron(win, COLOR_PAIR(color));
        mvwaddch(win, pos.0, pos.1, shooter.char());
        wattroff(win, COLOR_PAIR(color));

        for bullet in shooter.bullets() {
            Printer::object(win, bullet);
        }
    }
}
