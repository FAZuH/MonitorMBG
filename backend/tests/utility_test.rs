use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::routing::get;
use backend::config::StorageConfig;
use backend::routes::utility::UtilityState;
use backend::routes::utility::health_check_handler;
use backend::service::storage::StorageService;
use backend::service::utility::UtilityService;
use tower::util::ServiceExt;
use uuid::Uuid;

mod common;

/// Creates a temporary storage service for testing.
fn create_test_storage_service() -> Arc<StorageService> {
    let temp_dir = std::env::temp_dir().join(format!("test_utility_{}", Uuid::new_v4()));
    let config = StorageConfig {
        storage_type: "local".to_string(),
        local_path: Some(temp_dir.to_string_lossy().to_string()),
        bucket: None,
        region: None,
        access_key: None,
        secret_key: None,
        endpoint: None,
        base_url: Some("http://localhost:3000/uploads".to_string()),
    };
    Arc::new(StorageService::new(&config).expect("Failed to create test storage service"))
}

#[tokio::test]
async fn test_health_check() {
    let (db, db_name) = common::setup_db().await;
    let storage_service = create_test_storage_service();
    let utility_service = Arc::new(UtilityService::new(db.clone(), storage_service));

    let state = UtilityState {
        service: utility_service,
    };

    let app = Router::new()
        .route("/health", get(health_check_handler))
        .with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}
