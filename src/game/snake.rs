use std::collections::VecDeque;

use super::{direction::Direction, gen_random_idx_in_range, Position, COLS, ROWS, SNAKE_MAX_IDX};

pub struct Snake {
    pub dir: Direction,
    pub occupied: VecDeque<Position>,
    max_idx: usize,
}

impl Snake {
    pub fn new() -> Self {
        let x = gen_random_idx_in_range();
        let y = gen_random_idx_in_range();
        let mut deq = VecDeque::with_capacity(COLS * ROWS);

        for i in x..x + 8 {
            deq.push_back((i, y));
        }

        let max_idx = ROWS - 1;

        Self {
            dir: Default::default(),
            occupied: deq,
            max_idx,
        }
    }

    pub fn head(&self) -> Position {
        // cannot be empty so its safe to unwrap
        *self.occupied.front().unwrap()
    }

    pub fn tail(&self) -> Position {
        *self.occupied.back().unwrap()
    }

    // returns new head position
    pub fn move_(&mut self, dir: Direction) -> Position {
        if dir == self.dir.opposite() {
            return self.move_(self.dir);
        }
        let mut head = self.head();

        match dir {
            Direction::Down => {
                if head.0 == self.max_idx {
                    head.0 = 0
                } else {
                    head.0 += 1
                }
            }
            Direction::Up => {
                if head.0 == 0 {
                    head.0 = self.max_idx
                } else {
                    head.0 -= 1
                }
            }
            Direction::Right => {
                if head.1 == self.max_idx {
                    head.1 = 0
                } else {
                    head.1 += 1
                }
            }
            Direction::Left => {
                if head.1 == 0 {
                    head.1 = self.max_idx
                } else {
                    head.1 -= 1
                }
            }
        };

        // mofo
        self.dir = dir;

        self.occupied.push_front(head);

        head
    }
}

impl Default for Snake {
    fn default() -> Self {
        let x = gen_random_idx_in_range();
        let y = gen_random_idx_in_range();
        let mut deq = VecDeque::with_capacity(COLS * ROWS);

        for i in x..x + 8 {
            deq.push_back((i, y));
        }

        let max_idx = SNAKE_MAX_IDX;

        Self {
            dir: Default::default(),
            occupied: deq,
            max_idx,
        }
    }
}

#[cfg(test)]
mod snake_tests {
    use super::*;

    struct SnakeExpecteadHead {
        snake: Snake,
        expected: Position,
    }
    #[test]
    fn test_snake_appears_on_other_side() {
        let totally_random_number = 3usize;

        let mut snakes = vec![
            SnakeExpecteadHead {
                snake: Snake {
                    dir: Direction::Up,
                    occupied: VecDeque::from([(0usize, totally_random_number)]),
                    max_idx: SNAKE_MAX_IDX,
                },
                expected: (SNAKE_MAX_IDX, totally_random_number),
            },
            SnakeExpecteadHead {
                snake: Snake {
                    dir: Direction::Down,
                    occupied: VecDeque::from([(SNAKE_MAX_IDX, totally_random_number)]),
                    max_idx: SNAKE_MAX_IDX,
                },
                expected: (0, totally_random_number),
            },
            SnakeExpecteadHead {
                snake: Snake {
                    dir: Direction::Left,
                    occupied: VecDeque::from([(totally_random_number, 0usize)]),
                    max_idx: SNAKE_MAX_IDX,
                },
                expected: (totally_random_number, SNAKE_MAX_IDX),
            },
            SnakeExpecteadHead {
                snake: Snake {
                    dir: Direction::Right,
                    occupied: VecDeque::from([(totally_random_number, SNAKE_MAX_IDX)]),
                    max_idx: SNAKE_MAX_IDX,
                },
                expected: (totally_random_number, 0),
            },
        ];

        for snake_expected in snakes.iter_mut() {
            let snake = &mut snake_expected.snake;
            snake.move_(snake.dir);
            assert_eq!(snake.head(), snake_expected.expected);
        }
    }
}
