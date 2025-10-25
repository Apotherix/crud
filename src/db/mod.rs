pub mod pool;

pub use pool::init_pool;

pub async fn init_db() -> anyhow::Result<sqlx::SqlitePool> {
    let pool = init_pool().await?;
    let migrator = sqlx::migrate::Migrator::new(std::path::Path::new("./migrations"))
    .await
    .expect("Failed to read migration folder");
migrator.run(&pool).await?;

Ok(pool)
}