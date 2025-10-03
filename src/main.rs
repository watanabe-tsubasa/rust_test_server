use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use tracing_subscriber;

mod db;
mod models;
mod handlers;

use db::{init_pool, run_migrations};
use handlers::{root, create_user, create_todo, get_todo, list_todo, update_todo, delete_todo, db_health};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    // Log backend (postgres-only)
    println!("starting server with backend: postgres");

    if let Some(url) = std::env::var("DATABASE_URL").ok() {
        println!("DATABASE_URL detected: {}", url);
    } else {
        println!("DATABASE_URL not set; relying on PG* env vars");
    }

    let pool = match init_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("failed to initialize database pool: {}", e);
            return;
        }
    };

    if let Err(e) = run_migrations(&pool).await {
        eprintln!("failed to run migrations: {}", e);
        return;
    }

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/todos", post(create_todo).get(list_todo))
        .route("/todos/:id", get(get_todo).put(update_todo).delete(delete_todo))
        .route("/healthz/db", get(db_health))
        .with_state(pool);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            println!("listening on {}", addr);
            if let Err(e) = axum::serve(listener, app).await {
                eprintln!("server error: {}", e);
            }
        }
        Err(e) => eprintln!("failed to bind listener: {}", e),
    }
}
