mod bullet;
mod game;
mod logic;
mod printer;
mod shooter;
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

fn get_centralized_window() -> WINDOW {
    const LINES: i32 = 20;
    const COLS: i32 = 40;

    let x = getmaxx(stdscr());
    let y = getmaxy(stdscr());

    newwin(LINES, COLS, (y - LINES) / 2, (x - COLS) / 2)
}

fn main() {
    initialize();
    colors();

    let game_window: WINDOW = get_centralized_window();
    let mut invaders = game::Invaders::new(game_window);
    invaders.init();

    const DELAY: i32 = 250;
    wtimeout(game_window, DELAY);

    invaders.game_loop();

    delwin(game_window);
    finalize();
}
