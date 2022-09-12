use std::fmt::Display;

use rand::prelude::SliceRandom;

use super::{direction::Direction, snake::Snake, Board, Position, Tile, COLS, ROWS};

pub struct Game {
    world: Board<Tile>,
    pub snake: Snake,
    fruit: Position,
}

#[derive(PartialEq, Debug)]
pub enum GameState {
    Continues,
    Over,
    SnakeGrows,
}

impl Game {
    //initialize the fields first so random position of fruit can be chosen in
    //the new method
    fn pre_init() -> Self {
        let mut board: Board<Tile> = [[Tile::Empty; ROWS]; COLS];

        let snake = Snake::new();

        for pos in &snake.occupied {
            board[pos.0][pos.1] = Tile::Snake;
        }

        Game {
            world: board,
            snake,
            fruit: (0, 0),
        }
    }
    pub fn new_random() -> Self {
        let mut game = Game::pre_init();
        game.spawn_fruit();
        game
    }

    pub fn progress(&mut self, dir: Direction) -> GameState {
        let new_head = self.snake.move_(dir);

        if self.world[new_head.0][new_head.1] == Tile::Snake {
            return GameState::Over;
        }

        self.world[new_head.0][new_head.1] = Tile::Snake;

        if new_head == self.fruit {
            self.spawn_fruit();
            return GameState::SnakeGrows;
        } else {
            let tail = self.snake.occupied.pop_back().unwrap();
            self.world[tail.0][tail.1] = Tile::Empty;
            return GameState::Continues;
        }
    }

    // lets hope nobody beats the game bcs it will crash then
    fn spawn_fruit(&mut self) {
        let vacant_positions = self
            .world
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, tile)| *tile != &Tile::Snake)
                    .map(move |(y, _)| (x, y))
            })
            .collect::<Vec<Position>>();

        let fruit = vacant_positions.choose(&mut rand::thread_rng()).unwrap();
        self.fruit = *fruit;
        self.world[fruit.0][fruit.1] = Tile::Fruit;
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let border: String = std::iter::repeat('-').take(COLS + 2).collect();
        let head = self.snake.head();
        let board = self
            .world
            .iter()
            .enumerate()
            .fold(String::new(), |mut acc, (idy, row)| {
                acc.push('|');
                for (idx, tile) in row.iter().enumerate() {
                    if (idy, idx) == head {
                        acc.push('#');
                    } else {
                        acc.push_str(&tile.to_string());
                    }
                }
                acc.push_str(&"|\n");
                acc
            });

        write!(f, "{}\n{}{}", border, board, border)
    }
}

#[cfg(test)]
mod test_game {
    use std::collections::VecDeque;

    use crate::snake_game::SNAKE_MAX_IDX;

    use super::*;

    #[test]
    fn test_progress_returns_correct_game_state() {
        let occupied = VecDeque::from((5..10).map(|idx| (idx, 5)).collect::<Vec<_>>());
        let snake = Snake {
            dir: Direction::Up,
            occupied: occupied.clone(),
            max_idx: SNAKE_MAX_IDX,
        };
        let mut world = [[Tile::Empty; ROWS]; COLS];
        for (y, x) in occupied {
            world[y][x] = Tile::Snake;
        }
        // fruit directly above snakes head
        let fruit = (4, 5);
        let mut game = Game {
            world,
            snake,
            fruit,
        };

        assert_eq!(game.progress(Direction::Up), GameState::SnakeGrows);
        // make a circle
        assert_eq!(game.progress(Direction::Right), GameState::Continues);
        assert_eq!(game.progress(Direction::Down), GameState::Continues);
        // snake should eat itself
        assert_eq!(game.progress(Direction::Left), GameState::Over);
    }
}
