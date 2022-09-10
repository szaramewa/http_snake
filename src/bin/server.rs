use std::time::Duration;

use actix_web::{get, http::header::ContentType, post, web, App, HttpResponse, HttpServer};
use futures::join;
use http_snake::{
    direction_buffer::DirBuf,
    game::{direction::Direction, game::Game},
};
use parking_lot::{Mutex, RwLock};
use tokio::{
    runtime::Handle,
    signal,
    sync::{
        broadcast,
        mpsc::{self, Receiver, Sender},
    },
    time,
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // handle to async runtime for use in another thread
    let handle = Handle::current();

    let handle_print = handle.clone();

    let mut game = Game::new_random();

    let bs = web::Data::new(BoardString {
        board: RwLock::new(game.to_string()),
    });

    let dir_buf = web::Data::new(DirBuffer {
        inner: Mutex::new(DirBuf::new()),
    });

    let board_writer = bs.clone();
    let dir_buf_reader = dir_buf.clone();

    let (tx, mut rx): (Sender<String>, Receiver<String>) = mpsc::channel(1);
    let (kill_tx, mut kill_rx) = broadcast::channel(10);
    let mut print_kill_rx = kill_tx.subscribe();

    let interrupt_task = tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                println!("Got ctrlc");
                kill_tx.send(()).unwrap();
                println!("Sended");
            }
            Err(err) => println!("Error handling ctrlc: {}", err),
        }
    });

    // task managing game state
    let game_state_task = tokio::task::spawn_blocking(move || {
        handle.block_on(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            loop {
                tokio::select! {
                    _ = kill_rx.recv() => {
                        println!("Shutting down game_state_task");
                        break ;
                    },
                    _ = interval.tick() => {
                        {
                            let mut game_str = board_writer.board.write();
                            let dir = { dir_buf_reader.inner.lock().drain_and_get_random().clone() };

                            game.progress(dir);
                            *game_str = game.to_string();
                            let _ = tx.send(game_str.clone()).await.unwrap();
                        };
                    }
                }
            }
        });
    });

    let print_task = tokio::task::spawn_blocking(move || {
        handle_print.block_on(async move {
            loop {
                tokio::select! {
                    board = rx.recv() => {
                        if let Some(board) = board {
                            println!("{board}");
                        }
                    },
                    _ = print_kill_rx.recv() => {
                        println!("Shutting down print_task");
                        break;
                    }

                }
            }
        })
    });

    HttpServer::new(move || {
        App::new()
            .app_data(bs.clone())
            .app_data(dir_buf.clone())
            .service(get_board)
            .service(post_dir)
    })
    .bind(("127.0.0.1", 3721))
    .unwrap()
    .run()
    .await
    .unwrap();

    join!(game_state_task, print_task, interrupt_task);
    Ok(())
}

struct BoardString {
    board: RwLock<String>,
}

struct DirBuffer {
    inner: Mutex<DirBuf>,
}

#[get("/snake")]
async fn get_board(board: web::Data<BoardString>) -> String {
    {
        board.board.read().to_owned()
    }
}

#[post("/snake/{dir}")]
async fn post_dir(path: web::Path<(String,)>, buf: web::Data<DirBuffer>) -> HttpResponse {
    let dir = path.into_inner().0;
    match Direction::try_from(dir.as_str()) {
        Ok(dir) => {
            let res = { buf.inner.lock().push(dir) };

            match res {
                Ok(_) => HttpResponse::Ok()
                    .content_type(ContentType::plaintext())
                    .body("OK"),
                Err(err_msg) => HttpResponse::BadRequest()
                    .content_type(ContentType::plaintext())
                    .body(err_msg.to_string()),
            }
        }
        Err(err_msg) => HttpResponse::BadRequest()
            .content_type(ContentType::plaintext())
            .body(err_msg.to_string()),
    }
}
