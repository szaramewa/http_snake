use std::{
    fmt::Write,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use actix_web::{get, http::header::ContentType, post, web, App, HttpResponse, HttpServer};
use http_snake::game::{direction::Direction, game::Game};
use parking_lot::{Mutex, RwLock};
use rand::random;
use tokio::time;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // start server with some web framework
    // crate two threads, one with interval ticker sender
    // and the second one with reciver waiting to update game state
    // or maybe third one to print the board in console
    let mut game = Game::new_random();
    let bs = web::Data::new(BoardString {
        board: RwLock::new(game.to_string()),
    });

    let dir_buf = web::Data::new(DirBuffer {
        buffer: Mutex::new(Vec::new()),
    });

    let board_writer = bs.clone();
    let dir_buf_reader = dir_buf.clone();
    // spawn thread to manage game state
    let handle = thread::spawn(move || {
        // let mut interval = time::interval(Duration::from_millis(100));
        loop {
            thread::sleep(Duration::from_secs(1));
            // interval.tick().await;
            // new scope to drop locks
            {
                println!("tick");
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


                // tx.send(game.to_string()).unwrap();
            }
        }
    });

    // let (tx, mut rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    HttpServer::new(move || {
        App::new()
            .app_data(bs.clone())
            .app_data(dir_buf.clone())
            .service(get_board)
            .service(post_dir)
    })
    .bind(("127.0.0.1", 3721))?
    .run()
    .await;
    handle.join().unwrap();

    // thread::spawn(move || {
    //     while let Ok(board) = rx.recv() {
    //         println!("{board}");
    //     }
    // });

    Ok(())
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

// #[post("/snake/up")]
// async fn post_up(buf: web::Data<DirBuffer>) -> HttpResponse {
//     let dir = Direction::Up;
//
//     buf.buffer.lock().push(dir);
//     HttpResponse::Ok().finish()
// }
