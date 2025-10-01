use axum::{
    extract::{Path, State}, http::StatusCode, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Error, SqlitePool};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let pool = SqlitePool::connect("sqlite://todos.db").await?;
    // let todos = Arc::new(Mutex::new(Vec::<Todo>::new()));

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route(
            "/todos",
            post(create_todo)
            .get(list_todo)
        )
        .route(
            "/todo/{id}",
            get(get_todo)
            .put(update_todo)
            .delete(delete_todo)
        )
        .with_state(pool);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>,  StatusCode> {
    let todo = sqlx::query_as::<_, Todo>("
        INSERT INTO todos (title, done) VALUES (?, 0) RETURNING id, title, done
    ")
    .bind(payload.title)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todo))
}

async fn get_todo (
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Todo>, StatusCode> {
    let result = sqlx::query_as::<_, Todo>("
        SELECT * FROM todos WHERE id = ?
    ")
    .bind(id)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(todo) => Ok(Json(todo)),
        Err(Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_todo (
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let result = sqlx::query_as::<_, Todo>("
        UPDATE todos SET done = ? WHERE id = ? RETURNING id, title, done
    ")
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

async fn delete_todo (
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("
        DELETE FROM todos WHERE id = ?
    ")
    .bind(id)
    .execute(&pool)
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


async fn list_todo(
    State(pool): State<SqlitePool>
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = sqlx::query_as::<_, Todo>("
        SELECT * FROM todos
    ")
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: i64,
    username: String,
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTodo {
    done: bool,
}

#[derive(Serialize, Deserialize, Clone, FromRow)]
struct Todo {
    id: i64,
    title: String,
    done: bool,
}
