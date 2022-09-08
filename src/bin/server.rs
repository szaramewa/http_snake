use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use http_snake::game::direction::Direction;
use parking_lot::{Mutex, RwLock};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // start server with some web framework
    // crate two threads, one with interval ticker sender
    // and the second one with reciver waiting to update game state
    // or maybe third one to print the board in console
    let bs = web::Data::new(BoardState {
        board: RwLock::new("SHNAKE".to_owned()),
    });

    let dir_buf = web::Data::new(DirBuffer {
        buffer: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(bs.clone())
            .app_data(dir_buf.clone())
            .service(get_board)
            .service(post_up)
    })
    .bind(("127.0.0.1", 3721))?
    .run()
    .await
}

struct BoardState {
    board: RwLock<String>,
}

struct DirBuffer {
    buffer: Mutex<Vec<Direction>>,
}

#[get("/snake")]
async fn get_board(board: web::Data<BoardState>) -> String {
    board.board.read().to_owned()
}

// #[post("/snake/{dir}")]
// async fn post_dir(path: web::Path<(String)>, buf: web::Data<DirBuffer>) -> Result<String>{
//    let dir = Direction::try_from(path.into_inner());
// }

#[post("/snake/up")]
async fn post_up(buf: web::Data<DirBuffer>) -> HttpResponse {
    let dir = Direction::Up;

    buf.buffer.lock().push(dir);
    HttpResponse::Ok().finish()
}
