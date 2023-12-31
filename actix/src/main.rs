// Part 1
// https://youtu.be/gQwA0g0NNSI
// Part 2
// https://youtu.be/e37NbhSm56o
// Part 3
// https://youtu.be/3vMxuM7ezEk

mod config;
mod models;
mod handlers;
mod db;

use actix_web::{ HttpServer, App, web };
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("[Server] starting...");
    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();
    println!("[Server] running on: http://{}:{}/", config.server.host, config.server.port);
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/todos/{list_id}/items{_:/?}", web::get().to(get_items))
            .route("/todos/{list_id}/items/{item_id}{_:/?}", web::put().to(check_item))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
