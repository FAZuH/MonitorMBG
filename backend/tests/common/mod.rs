use std::sync::Arc;
use dotenv::dotenv;
use uuid::Uuid;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::Postgres;

use backend::database::Database;

pub async fn setup_db() -> (Arc<Database>, String) {
    dotenv().ok();
    let uuid = Uuid::new_v4();
    let db_name = format!("monitor_mbg_test_{}", uuid);
    let db_url = format!("postgres://postgres:password@localhost:5432/{}", db_name);
    
    // Create database
    Postgres::create_database(&db_url).await.expect("Failed to create database");

    let db = Database::new(&db_url)
        .await
        .expect("Failed to create database connection");

    // Run migrations to ensure tables exist
    db.run_migrations().await.expect("Failed to run migrations");

    (Arc::new(db), db_name)
}

pub async fn teardown_db(db: Arc<Database>, db_name: String) {
    db.pool.close().await;
    let db_url = format!("postgres://postgres:password@localhost:5432/{}", db_name);
    Postgres::drop_database(&db_url).await.expect("Failed to drop database");
}

