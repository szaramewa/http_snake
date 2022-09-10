use std::collections::HashMap;

use rand::prelude::SliceRandom;

use crate::game::direction::Direction;

const BUF_MAX_SIZE: usize = 1024;

pub struct DirBuf {
    buffer: [Option<Direction>; BUF_MAX_SIZE],
    idx: usize,
    pub current_dir: Direction,
}

impl DirBuf {
    pub fn new() -> Self {
        Self {
            buffer: [None; BUF_MAX_SIZE],
            idx: 0,
            current_dir: Default::default(),
        }
    }

    // push to the buffer, if buffer is full or pushed direction is opposite to
    // the current one return err
    pub fn push(&mut self, dir: Direction) -> Result<(), anyhow::Error> {
        if self.idx > BUF_MAX_SIZE {
            return Err(anyhow::anyhow!("full buffer"));
        }
        if dir != self.current_dir.opposite() {
            self.buffer[self.idx] = Some(dir);
            self.idx += 1;
            return Ok(());
        }

        return Err(anyhow::anyhow!("Cant go backwards"));
    }

    // clears buffer and returns random direction, if buffer is empty returns
    // current direction
    pub fn drain_and_get_random(&mut self) -> Direction {
        let mut rng = rand::thread_rng();

        let dirs: Vec<Direction> = self
            .buffer
            .iter_mut()
            .filter_map(|dir| dir.take())
            .collect();


        let five_random: Vec<Direction> = dirs.choose_multiple(&mut rng, 5).cloned().collect();

        let map = five_random.iter().fold(HashMap::new(), |mut map, dir| {
            *map.entry(dir).or_insert(0) += 1;
            map
        });

        let max = map.values().max();

        let random_dir = match max {
            Some(max) => {
                let most_common = map
                    .iter()
                    .filter(|(_, occur)| *occur == max)
                    .map(|(dir, _)| **dir)
                    .collect::<Vec<Direction>>();

                *most_common.choose(&mut rng).unwrap()
            }
            None => self.current_dir,
        };

        self.idx = 0;
        self.current_dir = random_dir;
        random_dir
    }
}
