use std::collections::VecDeque;

use super::{direction::Direction, gen_random_idx_in_range, Position, COLS, ROWS};

pub struct Snake {
    pub dir: Direction,
    pub occupied: VecDeque<Position>,
    max_idx: usize,
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

        let max_idx = ROWS - 1;

        Self {
            dir: Default::default(),
            occupied: deq,
            max_idx,
        }
    }
}
