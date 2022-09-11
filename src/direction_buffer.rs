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
        if self.idx == BUF_MAX_SIZE {
            return Err(anyhow::anyhow!("full buffer"));
        }
        if dir != self.current_dir.opposite() {
            self.buffer[self.idx] = Some(dir);
            self.idx += 1;
            return Ok(());
        }

        return Err(anyhow::anyhow!("cant go backwards"));
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

        println!("{}", dirs.len());
        let five_random: Vec<Direction> = dirs.choose_multiple(&mut rng, 5).cloned().collect();

        let map = five_random.iter().fold(HashMap::new(), |mut map, dir| {
            *map.entry(dir).or_insert(0) += 1;
            map
        });

        let max = map.values().max();

        let new_dir = match max {
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
        self.current_dir = new_dir;
        new_dir
    }

    pub fn set_dir(&mut self, dir: Direction) {
        self.current_dir = dir;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIRS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    #[test]
    fn test_push_on_full_buffer() {
        let mut buf = DirBuf::new();

        for _ in 0..BUF_MAX_SIZE {
            let _ = buf.push(Direction::Up);
        }

        let resp = buf.push(Direction::Left);
        assert!(resp.is_err());
        assert_eq!(
            resp.map_err(|e| e.to_string()),
            Err("full buffer".to_owned())
        );
    }

    #[test]
    fn test_cannot_push_opposite_direction() {
        let mut buf = DirBuf::new();
        for dir in DIRS{
            buf.set_dir(dir);
            let resp = buf.push(dir.opposite());
            assert!(resp.is_err());
            assert_eq!(
                resp.map_err(|e| e.to_string()),
                Err("cant go backwards".to_owned())
            );
        }
    }

    #[test]
    fn test_drain_nones_whole_buffer() {
        for dir in DIRS{
            let mut buf = DirBuf::new();
            buf.set_dir(dir);

            // fill buffer until full
            loop {
                let resp = buf.push(rand::random());
                if resp.map_err(|e| e.to_string()) == Err("full buffer".to_owned()) {
                    break;
                }
            }
            let _ = buf.drain_and_get_random();
            assert!(buf.buffer.iter().all(|item| item.is_none()));
        }
    }

    #[test]
    fn test_get_random_returns_correct_value_if_consists_of_one_element() {
        for dir in DIRS {
            let mut buf = DirBuf::new();
            buf.set_dir(dir);
            let _ = buf.push(dir);
            assert_eq!(dir, buf.drain_and_get_random());
        }
    }

}
