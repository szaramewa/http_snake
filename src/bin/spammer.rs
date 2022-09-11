use std::time::Duration;

use http_snake::snake_game::direction::Direction;

#[tokio::main]
async fn main() {
    loop {
        tokio::spawn(async move {
            // this might be too slow
            let dir: Direction = rand::random();
            let client = reqwest::Client::new();
            let _ = client
                .post(format!("http://127.0.0.1:3721/snake/{}", dir))
                .send()
                .await;
        });
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
}
