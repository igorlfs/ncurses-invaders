use ncurses::*;

use crate::{
    boss::Boss,
    power::{Effect, PowerUp},
    shield::Shield,
    shooter::Shooter,
    window, COLS, LINES,
};

pub struct Printer {}

#[derive(Clone, Copy)]
struct Bundle {
    char_shooter: u32,
    color_shooter: i16,
    char_bullet: u32,
    color_bullet: i16,
}

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
        let window = window::get_mid_window(3, COLS, LINES + 3);
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

    pub fn powers(win: WINDOW, powers: &[PowerUp]) {
        wattron(win, COLOR_PAIR(5));
        for power in powers {
            let pos = power.pos();
            mvwaddch(win, pos.0, pos.1, power.char());
        }
        wattroff(win, COLOR_PAIR(5));
    }

    pub fn boss(win: WINDOW, boss: &Boss) {
        wattron(win, COLOR_PAIR(8));
        mvwaddch(win, 2, boss.left_pos(), '\\' as u32);
        mvwaddch(win, 2, boss.left_pos() + 1, '/' as u32);
        wattroff(win, COLOR_PAIR(8));
    }

    pub fn shields(win: WINDOW, shields: &[Shield]) {
        wattron(win, COLOR_PAIR(6));
        for shield in shields {
            let pos = shield.pos();
            mvwaddch(win, pos.0, pos.1, '_' as u32);
        }
        wattroff(win, COLOR_PAIR(6));
    }

    pub fn follower(win: WINDOW, follower: &Shield) {
        wattron(win, COLOR_PAIR(3));
        let pos = follower.pos();
        mvwaddch(win, pos.0, pos.1, '_' as u32);
        wattroff(win, COLOR_PAIR(3));
    }

    pub fn enemies(win: WINDOW, enemies: &[Shooter]) {
        let bundle = Bundle {
            char_shooter: 'v' as u32,
            color_shooter: 2,
            char_bullet: ':' as u32,
            color_bullet: 1,
        };
        for enemy in enemies {
            Printer::shooter_helper(win, enemy, &bundle);
        }
    }

    pub fn player(win: WINDOW, player: &Shooter) {
        let bundle = Bundle {
            char_shooter: '^' as u32,
            color_shooter: 3,
            char_bullet: '.' as u32,
            color_bullet: 4,
        };
        Printer::shooter_helper(win, player, &bundle);
    }

    fn shooter_helper(win: WINDOW, shooter: &Shooter, bundle: &Bundle) {
        wattron(win, COLOR_PAIR(bundle.color_shooter));
        let pos = shooter.pos();
        mvwaddch(win, pos.0, pos.1, bundle.char_shooter);
        wattroff(win, COLOR_PAIR(bundle.color_shooter));

        wattron(win, COLOR_PAIR(bundle.color_bullet));

        for bullet in shooter.bullets() {
            let pos = bullet.pos();
            mvwaddch(win, pos.0, pos.1, bundle.char_bullet);
        }

        wattroff(win, COLOR_PAIR(bundle.color_bullet));
    }
}
