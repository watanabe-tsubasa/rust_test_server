use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub done: bool,
}

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub done: bool,
}
