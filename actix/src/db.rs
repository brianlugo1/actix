use crate::models::{ TodoList, TodoItem };
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    println!("[Server] incoming /todo/ GET...");
    let statement = client.prepare(
        "select * from todo_list"
    ).await.unwrap();
    let todos = client.query(&statement, &[])
    .await
    .expect("Error getting todo lists")
    .iter()
    .map(|row| TodoList::from_row_ref(row).unwrap())
    .collect::<Vec<TodoList>>();
    println!("[Server]      Successfully retrieved all todo lists");
    Ok(todos)
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
    println!("[Server]      Successfully created new todo list");
    return created_todo_list;
}

pub async fn get_todo(client: &Client, list_id: i32) -> Result<TodoList, io::Error> {
    println!("[Server] incoming /todo/{list_id}/ GET...");
    let statement = client.prepare(
        &format!(
            "select * from todo_list where id = {list_id}"
        )
    ).await.unwrap();
    let todo = client.query(&statement, &[])
    .await
    .expect("Error getting todo list")
    .iter()
    .map(|row| TodoList::from_row_ref(row).unwrap())
    .collect::<Vec<TodoList>>();
    println!("[Server]      Successfully retrieved todo list: {list_id}");
    Ok(todo[0].clone())
}

pub async fn update_todo(client: &Client, list_id: i32, title: String) -> Result<Vec<TodoList>, io::Error> {
    println!("[Server] incoming /todo/{list_id} PUT...");
    let statement = client.prepare(
        &format!(
            "update todo_list set title = '{title}' where id = {list_id}"
        )
    ).await.unwrap();
    let todo = client.query(&statement, &[])
        .await
        .expect("Error updating todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    println!("[Server]      Successfully updated todo list: {list_id}");
    // Currently returns an empty array because the sql statement does not retrieve any data,
    //  it only updates an existing todo list!
    Ok(todo)
}

pub async fn delete_todo(client: &Client, list_id: i32) -> Result<Vec<TodoList>, io::Error> {
    println!("[Server] incoming /todo/{list_id}/ DELETE...");
    let statement = client. prepare(
        &format!(
            "delete from todo_list where id={list_id}"
        )
    ).await.unwrap();
    let todo = client.query(&statement, &[])
        .await
        .expect("Error deleting todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    println!("[Server]      Successfully deleted todo list: {list_id}");
    Ok(todo)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error> {
    println!("[Server] incoming /todo/{list_id}/items/ GET...");
    let statement = client.prepare(
        &format!(
            "select * from todo_item where list_id = {list_id} order by id"
        )
    ).await.unwrap();
    let items = client.query(&statement, &[])
        .await
        .expect("Error getting todo items")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
    println!("[Server]      Successfully retrieved todo items for todo list: {list_id}");
    Ok(items)
}

pub async fn get_item(client: &Client, list_id: i32, item_id: i32) -> Result<Vec<TodoItem>, io::Error> {
    println!("[Server] incoming /todo/{list_id}/item/{item_id}/ GET...");
    let statement = client.prepare(
        &format!(
            "select * from todo_item where list_id = {list_id} and id = {item_id}"
        )
    ).await.unwrap();
    let item = client.query(&statement, &[])
        .await
        .expect("Error getting todo item")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
    println!("[Server]      Successfully retrieved todo item: {item_id}");
    Ok(item)
}

pub async fn create_item(client: &Client, title: String, checked: bool, list_id: i32) -> Result<TodoItem, io::Error> {
    println!("[Server] incoming /todo/{list_id}/item/ POST...");
    let statement = client.prepare(&format!(
        "insert into todo_item (title, checked, list_id) values (\'{title}\', {checked}, {list_id}) returning id, title, checked, list_id"
    )).await.unwrap();
    let create_todo_item = client.query(&statement, &[])
        .await
        .expect("Error creating todo item")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating todo item"));
    println!("[Server]      Successfully created a new todo item");
    return create_todo_item;
}

pub async fn update_item(client: &Client, title: String, list_id: i32, item_id: i32) -> Result<Vec<TodoItem>, io::Error> {
    println!("[Server] incoming /todo/{list_id}/item/{item_id}/ PUT...");
    let statement = client.prepare(
        &format!(
            "update todo_item set title = '{title}' where id = {item_id}"
        )
    ).await.unwrap();
    let todo = client.query(&statement, &[])
        .await
        .expect("Error updating todo item")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
    println!("[Server]      Successfully updated todo item: {item_id}");
    Ok(todo)
}

pub async fn delete_item(client: &Client, list_id: i32, item_id: i32) -> Result<Vec<TodoItem>, io::Error> {
    println!("[Server] incoming /todo/{list_id}/items/{item_id}");
    let statement = client.prepare(
        &format!(
            "delete from todo_item where id={item_id}"
        )
    ).await.unwrap();
    let todo = client.query(&statement, &[])
        .await
        .expect("Error deleting todo item")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
    println!("[Server]      Successfully deleted todo item: {item_id}");
    Ok(todo)
}

pub async fn check_item(client: &Client, list_id: i32, item_id: i32) -> Result<(), io::Error> {
    println!("[Server] incoming /todo/{list_id}/item/{item_id} PUT...");
    let statement = client.prepare(
        &format!(
            "update todo_item set checked = not checked where list_id = {list_id} and id = {item_id}"
        )).await.unwrap();
    let result = client.execute(&statement, &[])
        .await
        .expect("Error checking todo item");
    match result {
        ref updated if *updated == 1 => {
            println!("[Server]      Successfully checked todo list: {list_id}'s todo item: {item_id}");
            Ok(())
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to check item"))
    }
}