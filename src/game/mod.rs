use std::ops::Range;

mod direction;
mod game;

// world size
const ROWS: usize = 40;
const COLS: usize = 40;

const POSSIBLE_SNAKE_STARTING_RANGE: Range<usize> = 15..26;
