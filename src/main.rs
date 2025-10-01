use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State}, http::StatusCode, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let todos = Arc::new(Mutex::new(Vec::<Todo>::new()));

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
        .with_state(todos);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
    State(todos): State<Arc<Mutex<Vec<Todo>>>>,
    Json(payload): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let mut todos = todos.lock().unwrap();
    match !payload.title.is_empty() {
        true => {
            let id = (todos.len() + 1) as u64;
            let todo = Todo {
                id,
                title: payload.title,
                done: false
            };
            todos.push(todo.clone());
            (StatusCode::CREATED, Json(todo))
        },
        false => (StatusCode::BAD_REQUEST, Json(Todo { id: 0, title: "does not exist".to_string(), done: false })),
    }
}

async fn get_todo (
    Path(id): Path<u64>,
    State(todos): State<Arc<Mutex<Vec<Todo>>>>,
) -> Result<Json<Todo>, StatusCode> {
    let todos = todos.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(todo) = todos.iter().find(|&todo| todo.id == id) {
        Ok(Json(todo.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn update_todo (
    Path(id): Path<u64>,
    State(todos): State<Arc<Mutex<Vec<Todo>>>>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = todos.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.done = payload.done;
        Ok(Json(todo.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_todo (
    Path(id): Path<u64>,
    State(todos): State<Arc<Mutex<Vec<Todo>>>>
) -> Result<String, StatusCode> {
    let mut todos = todos.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let before = todos.len();
    todos.retain(|t| t.id != id);
    let after = todos.len();
    if before == after {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(format!("id: {} is deleted.", id))
    }
}


async fn list_todo(
    State(todos): State<Arc<Mutex<Vec<Todo>>>>
) -> (StatusCode, Json<Vec<Todo>>) {
    let todos = todos.lock().unwrap();
    (StatusCode::OK, Json(todos.clone()))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
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

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u64,
    title: String,
    done: bool,
}
