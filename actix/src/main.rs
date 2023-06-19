// Part 1
// https://youtu.be/gQwA0g0NNSI
mod models;

use crate::models::Status;
use actix_web::{ HttpServer, App, web, Responder };

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "UP".to_string()})
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("[Server] starting...");
    println!("[Server] running on: http://127.0.0.1:8080/");
    HttpServer::new(||
        App::new()
            .route("/", web::get().to(status))
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
