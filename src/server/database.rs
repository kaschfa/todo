#![cfg(feature = "server")]
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let db_url = "postgres://app_user:supersecret@localhost:5432/app_db";

    Database::connect(db_url).await
}
