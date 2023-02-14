use ncurses::*;

pub fn get_centralized_window(lines: i32, cols: i32) -> WINDOW {
    let x = getmaxx(stdscr());
    let y = getmaxy(stdscr());

    newwin(lines, cols, (y - lines) / 2, (x - cols) / 2)
}
