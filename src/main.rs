mod bullet;
mod game;
mod logic;
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
    const TOTAL_COLORS: i16 = 8;
    init_pair(TOTAL_COLORS, 0, 0);
    for i in 1..TOTAL_COLORS {
        init_pair(i, i, -1);
    }
}

pub const LINES: i32 = 20;
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
