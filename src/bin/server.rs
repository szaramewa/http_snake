use std::{
    thread,
    time::Duration,
};

use actix_web::{get, http::header::ContentType, post, web, App, HttpResponse, HttpServer};
use http_snake::game::{direction::Direction, game::Game};
use parking_lot::{Mutex, RwLock};
use rand::random;
use tokio::{
    runtime::Handle,
    sync::mpsc::{self, Receiver, Sender},
    time,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // handle to async runtime for use in another thread
    // maybe change it to local thread so it doesnt span on other threads
    let handle = Handle::current();
    let mut game = Game::new_random();
    let bs = web::Data::new(BoardString {
        board: RwLock::new(game.to_string()),
    });

    let dir_buf = web::Data::new(DirBuffer {
        buffer: Mutex::new(Vec::new()),
    });

    let board_writer = bs.clone();
    let dir_buf_reader = dir_buf.clone();

    let (tx, mut rx): (Sender<String>, Receiver<String>) = mpsc::channel(100);

    // spawn thread to manage game state
    thread::spawn(move || {
        handle.spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));

            loop {
                interval.tick().await;
                {
                    let mut game_str = board_writer.board.write();
                    let dir = {
                        dir_buf_reader
                            .buffer
                            .lock()
                            .get(0)
                            .unwrap_or(&random())
                            .clone()
                    };

                    game.progress(dir);
                    *game_str = game.to_string();
                }
                let _ = tx.send(game.to_string()).await;
            }
        });
    });

    while let Some(board) = rx.recv().await {
        println!("{board}");
    }

    HttpServer::new(move || {
        App::new()
            .app_data(bs.clone())
            .app_data(dir_buf.clone())
            .service(get_board)
            .service(post_dir)
    })
    .bind(("127.0.0.1", 3721))?
    .run()
    .await
}

struct BoardString {
    board: RwLock<String>,
}

struct DirBuffer {
    buffer: Mutex<Vec<Direction>>,
}

#[get("/snake")]
async fn get_board(board: web::Data<BoardString>) -> String {
    println!("got get");
    {
        board.board.read().to_owned()
    }
}

#[post("/snake/{dir}")]
async fn post_dir(path: web::Path<(String,)>, buf: web::Data<DirBuffer>) -> HttpResponse {
    let dir = path.into_inner().0;
    match Direction::try_from(dir.as_str()) {
        Ok(dir) => {
            {
                buf.buffer.lock().push(dir);
            }
            HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("OK")
        }
        Err(err_msg) => HttpResponse::BadRequest()
            .content_type(ContentType::plaintext())
            .body(err_msg.to_string()),
    }
}
