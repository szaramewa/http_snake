use std::{fmt::Display, ops::Range};

use rand::Rng;

pub mod direction;
pub mod game;
pub mod snake;

// world size
const ROWS: usize = 15;
const COLS: usize = 15;

const POSSIBLE_SNAKE_STARTING_RANGE: Range<usize> = 3..5;

const SNAKE_MAX_IDX: usize = ROWS - 1;

type Board<T> = [[T; ROWS]; COLS];
type Position = (usize, usize);

pub fn gen_random_idx_in_range() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(POSSIBLE_SNAKE_STARTING_RANGE)
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Snake,
    Empty,
    Fruit,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Snake => '*',
                Tile::Empty => ' ',
                Tile::Fruit => '@',
            }
        )
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}
