use std::ops::Range;

pub mod direction;
pub mod game;

// world size
const ROWS: usize = 40;
const COLS: usize = 40;

const POSSIBLE_SNAKE_STARTING_RANGE: Range<usize> = 15..26;
