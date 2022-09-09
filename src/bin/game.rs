use std::{thread, time::Duration};

use http_snake::game::{direction::Direction, game::Game};

fn main() {
    let mut game = Game::new_random();
    loop {
        println!("{game}");
        let dir: Direction = rand::random();
        game.progress(dir);

        thread::sleep(Duration::from_millis(100));
    }
}
