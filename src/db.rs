use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

pub type DbPool = sqlx::PgPool;

pub async fn init_pool() -> Result<DbPool, sqlx::Error> {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        println!("Using DATABASE_URL: {}", url);
        // Use the full URL as-is to preserve options like channel_binding
        DbPool::connect(&url).await
    } else {
        println!("DATABASE_URL not set, falling back to PG* environment variables");

        // Convert missing env vars into sqlx::Error::Configuration for consistent error type
        let host = std::env::var("PGHOST").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
        let user = std::env::var("PGUSER").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
        let password = std::env::var("PGPASSWORD").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
        let database = std::env::var("PGDATABASE").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;

        let options = PgConnectOptions::new()
            .host(&host)
            .username(&user)
            .password(&password)
            .database(&database)
            .ssl_mode(PgSslMode::Require);

        PgPoolOptions::new().connect_with(options).await
    }
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::Error> {
    // Apply SQL migrations from the migrations/ directory at startup
    let migrator = sqlx::migrate!("./migrations");
    migrator.run(pool).await.map_err(Into::into)
}
