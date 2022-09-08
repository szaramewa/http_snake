use std::ops::Range;

pub mod direction;
pub mod game;
pub mod snake;

// world size
const ROWS: usize = 40;
const COLS: usize = 40;

const POSSIBLE_SNAKE_STARTING_RANGE: Range<usize> = 15..26;

type Board<T> = [[T; ROWS]; COLS];
type Position = (usize, usize);

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Snake,
    Empty,
    Fruit,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}
