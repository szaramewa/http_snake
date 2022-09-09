use std::fmt::Display;

use rand::prelude::SliceRandom;

use super::{direction::Direction, snake::Snake, Board, Position, Tile, COLS, ROWS};

pub struct Game {
    world: Board<Tile>,
    pub snake: Snake,
    fruit: Position,
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

    pub fn progress(&mut self, dir: Direction) {
        let new_head = self.snake.move_(dir);

        // inspect this condition, might be wrong
        // if self.world[new_head.0][new_head.1] == tile::snake {
        //     panic!("GAME OVER")
        // }

        if new_head == self.fruit {
            self.spawn_fruit();
        } else {
            let tail = self.snake.occupied.pop_back().unwrap();
            self.world[tail.0][tail.1] = Tile::Empty;
        }

        self.world[new_head.0][new_head.1] = Tile::Snake;
    }

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

        let fruit = vacant_positions
            .choose(&mut rand::thread_rng())
            .unwrap_or(&vacant_positions[0]);
        self.fruit = *fruit;
        self.world[fruit.0][fruit.1] = Tile::Fruit;
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let border: String = std::iter::repeat('-').take(ROWS + 2).collect();
        let board = self.world.iter().fold(String::new(), |mut acc, row| {
            acc.push('|');
            for tile in row {
                acc.push_str(&tile.to_string())
            }
            acc.push_str(&"|\n");
            acc
        });

        write!(f, "{}\n{}{}", border, board, border)
    }
}
