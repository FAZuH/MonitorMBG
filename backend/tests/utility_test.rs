use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::routing::get;
use backend::routes::utility::UtilityState;
use backend::routes::utility::health_check_handler;
use backend::service::utility::UtilityService;
use tower::util::ServiceExt;

mod common;

#[tokio::test]
async fn test_health_check() {
    let (db, db_name) = common::setup_db().await;
    let utility_service = Arc::new(UtilityService::new(db.clone()));

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
