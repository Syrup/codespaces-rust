use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct TodoQuery {
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
}
