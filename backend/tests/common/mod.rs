use std::sync::Arc;

use backend::database::Database;
use dotenv::dotenv;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::Postgres;
use uuid::Uuid;

pub async fn setup_db() -> (Arc<Database>, String) {
    dotenv().ok();
    let uuid = Uuid::new_v4();
    let db_name = format!("monitor_mbg_test_{}", uuid);
    let db_url = format!("postgres://postgres:password@localhost:5432/{}", db_name);

    // Create database
    Postgres::create_database(&db_url)
        .await
        .expect("Failed to create database");

    let db = Database::new(&db_url)
        .await
        .expect("Failed to create database connection");

    // Run migrations to ensure tables exist
    db.run_migrations().await.expect("Failed to run migrations");

    (Arc::new(db), db_name)
}

pub async fn teardown_db(db: Arc<Database>, db_name: String) {
    // Close the database connection pool
    db.close().await;

    // Give a moment for connections to fully close
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Drop the Arc to ensure the Database is dropped before we try to drop the database
    drop(db);

    // Additional small delay to ensure cleanup
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let db_url = format!("postgres://postgres:password@localhost:5432/{}", db_name);

    // Try to drop the database with retry logic
    let mut retries = 3;
    while retries > 0 {
        match Postgres::drop_database(&db_url).await {
            Ok(_) => break,
            Err(e) => {
                retries -= 1;
                if retries == 0 {
                    panic!("Failed to drop database after 3 retries: {}", e);
                }
                // Wait a bit longer before retrying
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }
    }
}
