use std::collections::{HashSet, VecDeque};

use rand::Rng;

use super::{direction::Direction, COLS, POSSIBLE_SNAKE_STARTING_RANGE, ROWS};

type Board = [[Tile; ROWS]; COLS];
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

struct Snake {
    dir: Direction,
    occupied: VecDeque<Position>,
    max_idx: usize,
}

impl Snake {
    fn new() -> Self {
        Default::default()
    }

    fn head(&self) -> &Position {
        // cannot be empty so its safe to unwrap
        self.occupied.front().unwrap()
    }

    fn tail(&self) -> &Position {
        self.occupied.back().unwrap()
    }

    // returns new head position
    fn move_(&mut self, dir: Direction) -> Position {
        let head = self.head();

        (0, 0)
    }
}

impl Default for Snake {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(POSSIBLE_SNAKE_STARTING_RANGE);
        let y = rng.gen_range(POSSIBLE_SNAKE_STARTING_RANGE);
        let mut deq = VecDeque::with_capacity(COLS * ROWS);

        for i in x..x + 4 {
            deq.push_back((i, y));
        }

        let max_idx = ROWS - 1;

        Self {
            dir: Direction::Left,
            occupied: deq,
            max_idx,
        }
    }
}

pub struct Game {
    world: Board,
    snake: Snake,
    vacant: HashSet<Position>,
    fruit: Position,
}

impl Game {
    pub fn new_random() -> Self {
        let mut board: Board = [[Tile::Empty; ROWS]; COLS];

        let snake = Snake::new();

        for pos in &snake.occupied {
            board[pos.0][pos.1] = Tile::Snake;
        }

        let vacant = board
            .iter()
            .enumerate()
            .flat_map(move |(idx, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, tile)| *tile != &Tile::Snake)
                    .map(|(idy, _)| (idx, idy))
                    .collect::<Vec<Position>>()
            })
            .collect::<HashSet<Position>>();

        Game {
            world: board,
            snake,
            vacant,
            fruit: (0, 0),
        }
    }

    fn progress(&mut self, dir: Direction) {}

    fn spawn_fruit(&mut self) {}
}
