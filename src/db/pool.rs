use sqlx::sqlite::SqlitePoolOptions;
use std::env;

pub async fn init_pool() -> anyhow::Result<sqlx::SqlitePool> {
    let database_url = env::var("DATABASE_URL")?;
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)

}