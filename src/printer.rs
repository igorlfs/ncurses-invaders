use ncurses::*;

use crate::shooter::Shooter;

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
