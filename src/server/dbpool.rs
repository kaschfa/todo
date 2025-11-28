#[cfg(feature = "server")]
use once_cell::sync::Lazy;

#[cfg(feature = "server")]
use sqlx::postgres::PgPoolOptions;

#[cfg(feature = "server")]
use tokio::sync::OnceCell;

#[cfg(feature = "server")]
pub type DbPool = sqlx::PgPool;

#[cfg(feature = "server")]
static POOL: OnceCell<DbPool> = OnceCell::const_new();

#[cfg(feature = "server")]
pub async fn get_pool() -> &'static DbPool {
    POOL.get_or_init(|| async {
        // You can hard-code this for now:
        let database_url = "postgres://app_user:supersecret@localhost:5432/app_db";
        // or use: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .expect("Failed to connect to Postgres")
    })
    .await
}
