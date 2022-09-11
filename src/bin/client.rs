use console::Term;
use http_snake::game::direction::Direction;

#[tokio::main]
async fn main() {
    let term = Term::stdout();
    // use wsad to move snake around
    loop {
        if let Ok(key) = term.read_char() {
            println!("{key}");
            if let Ok(dir) = Direction::try_from(key) {
                tokio::spawn(async move {
                    let client = reqwest::Client::new();
                    let _ = client
                        .post(format!("http://127.0.0.1:3721/snake/{}", dir))
                        .send()
                        .await;
                });
            }
        }
    }
}
