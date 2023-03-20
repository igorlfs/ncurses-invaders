use crate::{COLS, LINES};

pub fn out_of_bounds(pos: (i32, i32)) -> bool {
    pos.0 <= 1 || pos.1 <= 0 || pos.0 >= LINES - 1 || pos.1 >= COLS - 1
}
