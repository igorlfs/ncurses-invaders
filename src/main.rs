mod boss;
mod bullet;
mod direction;
mod game;
mod logic;
mod object;
mod power;
mod printer;
mod shield;
mod shooter;
mod util;
mod window;
use ncurses::*;

fn initialize() {
    initscr();
    cbreak();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    refresh();
    assert!(has_colors());
    start_color();
    use_default_colors();
}

fn finalize() {
    refresh();
    endwin();
}

fn colors() {
    const RAINBOW_COLORS: i16 = 7;
    init_pair(RAINBOW_COLORS, 7, 7);
    init_pair(RAINBOW_COLORS + 1, 0, -1);
    for i in 1..RAINBOW_COLORS {
        init_pair(i, i, -1);
    }
}

pub const LINES: i32 = 24;
pub const COLS: i32 = 40;

fn main() {
    initialize();
    colors();

    let game_window: WINDOW = window::get_centralized_window(LINES, COLS);
    let mut invaders = game::Invaders::new(game_window);
    invaders.init();

    const DELAY: i32 = 250;
    wtimeout(game_window, DELAY);

    invaders.game_loop();

    delwin(game_window);
    finalize();
}
