use console::Term;
use http_snake::game::direction::Direction;
#[tokio::main]
async fn main() {
    let term = Term::stdout();
    loop {
        let client = reqwest::Client::new();
        if let Ok(key) = term.read_char() {
            if let Ok(dir) = Direction::try_from(key) {
                let res = client
                    .post(format!("http://127.0.0.1:3721/snake/{dir}"))
                    .send()
                    .await;
            }
        }
    }
}
