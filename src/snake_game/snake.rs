use std::collections::VecDeque;

use super::{direction::Direction, gen_random_idx_in_range, Position, COLS, ROWS, SNAKE_MAX_IDX};

pub struct Snake {
    pub dir: Direction,
    pub occupied: VecDeque<Position>,
    pub(crate) max_idx: usize,
}

impl Snake {
    pub fn new() -> Self {
        Default::default()
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
mod test_snake {
    use super::*;

    #[test]
    fn test_cant_go_opposite_direction() {
        let random_pos = (10, 10);
        let mut snakes = vec![
            Snake {
                dir: Direction::Up,
                occupied: VecDeque::from([random_pos]),
                max_idx: SNAKE_MAX_IDX,
            },
            Snake {
                dir: Direction::Down,
                occupied: VecDeque::from([random_pos]),
                max_idx: SNAKE_MAX_IDX,
            },
            Snake {
                dir: Direction::Left,
                occupied: VecDeque::from([random_pos]),
                max_idx: SNAKE_MAX_IDX,
            },
            Snake {
                dir: Direction::Right,
                occupied: VecDeque::from([random_pos]),
                max_idx: SNAKE_MAX_IDX,
            },
        ];

        for snake in &mut snakes {
            let dir = snake.dir.clone();
            // move shouldnt change direction
            snake.move_(snake.dir.opposite());
            assert_eq!(dir, snake.dir);
        }
    }

    #[test]
    fn test_snake_appears_on_other_side() {
        struct SnakeExpectedHead {
            snake: Snake,
            expected_head: Position,
        }

        let totally_random_number = 3usize;

        let mut snakes = vec![
            SnakeExpectedHead {
                snake: Snake {
                    dir: Direction::Up,
                    occupied: VecDeque::from([(0usize, totally_random_number)]),
                    max_idx: SNAKE_MAX_IDX,
                },
                expected_head: (SNAKE_MAX_IDX, totally_random_number),
            },
            SnakeExpectedHead {
                snake: Snake {
                    dir: Direction::Down,
                    occupied: VecDeque::from([(SNAKE_MAX_IDX, totally_random_number)]),
                    max_idx: SNAKE_MAX_IDX,
                },
                expected_head: (0, totally_random_number),
            },
            SnakeExpectedHead {
                snake: Snake {
                    dir: Direction::Left,
                    occupied: VecDeque::from([(totally_random_number, 0usize)]),
                    max_idx: SNAKE_MAX_IDX,
                },
                expected_head: (totally_random_number, SNAKE_MAX_IDX),
            },
            SnakeExpectedHead {
                snake: Snake {
                    dir: Direction::Right,
                    occupied: VecDeque::from([(totally_random_number, SNAKE_MAX_IDX)]),
                    max_idx: SNAKE_MAX_IDX,
                },
                expected_head: (totally_random_number, 0),
            },
        ];

        for snake_expected in snakes.iter_mut() {
            let snake = &mut snake_expected.snake;
            snake.move_(snake.dir);
            assert_eq!(snake.head(), snake_expected.expected_head);
        }
    }
}
