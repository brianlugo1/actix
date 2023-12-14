use crate::models::{
    Status,
    CreateTodoList,
    CreateTodoItem,
    ResultResponse
};
use crate::db;
use std::io::ErrorKind::Other;
use deadpool_postgres::{ Pool, Client };
use actix_web::{ web, Responder, HttpResponse };

pub async fn status() -> impl Responder {
    println!("[Server] incoming / GET...");
    web::HttpResponse::Ok()
    .json(Status {
        status: "Welcome to Actix API!".to_string(),
        message: "Available routes are: (/todos (GET), /todos/{list_id}/items (GET))".to_string(),
    })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::get_todos(&client).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::create_todo(&client, json.title.clone()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn get_todo(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::get_todo(&client, path.0).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn update_todo(db_pool: web::Data<Pool>, path: web::Path<(i32,)>, json: web::Json<CreateTodoList>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::update_todo(&client, path.0, json.title.clone()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn delete_todo(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::delete_todo(&client, path.0).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::get_items(&client, path.0).await;
    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn get_item(db_pool: web::Data<Pool>, path: web::Path<(i32,i32)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::get_item(&client, path.0, path.1).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn create_item(db_pool: web::Data<Pool>, path: web::Path<(i32,)>, json: web::Json<CreateTodoItem>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::create_item(
        &client,
        json.title.clone(),
        false,
        path.0
    ).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn update_item(db_pool: web::Data<Pool>, path: web::Path<(i32,i32)>, json: web::Json<CreateTodoItem>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::update_item(
        &client,
        json.title.clone(),
        path.0,
        path.1
    ).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn delete_item(db_pool: web::Data<Pool>, path: web::Path<(i32,i32)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::delete_item(&client, path.0, path.1).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn check_item(db_pool: web::Data<Pool>, path: web::Path<(i32,i32)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::check_item(&client, path.0, path.1).await;
    match result {
        Ok(()) => HttpResponse::Ok().json(ResultResponse { success: true }),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(ResultResponse { success: false }),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}