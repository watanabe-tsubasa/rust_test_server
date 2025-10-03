mod todo;

use axum::{http::StatusCode, Json};
use sqlx::PgPool;
use crate::models::{CreateUser, User};
use axum::extract::State;

// root handler
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

// re-export todo handlers
pub use todo::*;

// Simple DB health check
pub async fn db_health(State(pool): State<PgPool>) -> StatusCode {
    let query = "SELECT 1";
    match sqlx::query_scalar::<_, i32>(query).fetch_one(&pool).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
