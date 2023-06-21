use crate::models::{ TodoList, TodoItem };
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    println!("[Server] incoming /todo/ GET...");
    let statement = client.prepare("select * from todo_list").await.unwrap();
    let todos = client.query(&statement, &[])
    .await
    .expect("Error getting todo lists")
    .iter()
    .map(|row| TodoList::from_row_ref(row).unwrap())
    .collect::<Vec<TodoList>>();
    println!("[Server] Successfully fetched all todo lists");
    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error> {
    println!("[Server] incoming /todo/{list_id}/items GET...");
    let statement = client.prepare(&format!("select * from todo_item where list_id = {list_id} order by id")).await.unwrap();
    let items = client.query(&statement, &[])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
    println!("[Server] Successfully fetched todo items for todo list: {list_id}");
    Ok(items)
}

pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, io::Error> {
    println!("[Server] incoming /todo/ POST...");
    let statement = client.prepare(&format!("insert into todo_list (title) values (\'{title}\') returning id, title")).await.unwrap();
    let created_todo_list = client.query(&statement, &[])
        .await
        .expect("Error creating todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating todo list"));
    println!("[Server] Successfully created new todo list");
    return created_todo_list;
}

pub async fn check_item(client: &Client, list_id: i32, item_id: i32) -> Result<(), io::Error> {
    println!("[Server] incoming /todo/{list_id}/item/{item_id} PUT...");
    // Need to add logic to toggle checked true or false!
    let statement = client.prepare(
        &format!(
            "update todo_item set checked = true where list_id = {list_id} and id = {item_id} and checked = false"
        )).await.unwrap();
    let result = client.execute(&statement, &[])
        .await
        .expect("Error checking todo item");
    match result {
        ref updated if *updated == 1 => {
            println!("[Server] Successfully checked todo list: {list_id}'s todo item: {item_id}");
            Ok(())
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to check item"))
    }
}