use std::collections::{HashSet, VecDeque};

use rand::thread_rng;

use super::{
    direction::Direction, snake::Snake, Board, Position, Tile, COLS, POSSIBLE_SNAKE_STARTING_RANGE,
    ROWS,
};

pub struct Game {
    world: Board<Tile>,
    snake: Snake,
    vacant: Board<bool>,
    fruit: Position,
}

impl Game {
    pub fn new_random() -> Self {
        let mut board: Board<Tile> = [[Tile::Empty; ROWS]; COLS];

        let mut vacant: Board<bool> = [[true; ROWS]; COLS];

        let snake = Snake::new();

        for pos in &snake.occupied {
            board[pos.0][pos.1] = Tile::Snake;
            vacant[pos.0][pos.1] = false;
        }


        Game {
            world: board,
            snake,
            vacant,
            fruit: (19, 19),
        }
    }

    fn progress(&mut self, dir: Direction) {
        let snake_head = self.snake.move_(dir);

        if snake_head == self.fruit {
            self.spawn_fruit();
        } else {
            let tail= self.snake.occupied.pop_back().unwrap();
            self.vacant[tail.0][tail.1] = true;
        }

        self.vacant[snake_head.0][snake_head.1] = false;
    }

    fn spawn_fruit(&mut self) {
        let vacant_positions = 

        let mut rng = thread_rng();
        
        let fruit = rng.
    }
}
