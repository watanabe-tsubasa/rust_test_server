use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::Error;

use crate::{db::DbPool, models::{CreateTodo, UpdateTodo, Todo}};

pub async fn create_todo(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let query = "INSERT INTO todos (title, done) VALUES ($1, false) RETURNING id, title, done";

    let todo = sqlx::query_as::<_, Todo>(query)
        .bind(payload.title)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todo))
}

pub async fn get_todo(
    Path(id): Path<i64>,
    State(pool): State<DbPool>,
) -> Result<Json<Todo>, StatusCode> {
    let query = "SELECT * FROM todos WHERE id = $1";

    let result = sqlx::query_as::<_, Todo>(query)
        .bind(id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(todo) => Ok(Json(todo)),
        Err(Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_todo(
    Path(id): Path<i64>,
    State(pool): State<DbPool>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let query = "UPDATE todos SET done = $1 WHERE id = $2 RETURNING id, title, done";

    let result = sqlx::query_as::<_, Todo>(query)
        .bind(payload.done)
        .bind(id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(todo) => Ok(Json(todo)),
        Err(Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_todo(
    Path(id): Path<i64>,
    State(pool): State<DbPool>,
) -> Result<StatusCode, StatusCode> {
    let query = "DELETE FROM todos WHERE id = $1";

    let result = sqlx::query(query)
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_todo(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}
