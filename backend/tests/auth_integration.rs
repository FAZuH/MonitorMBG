use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use backend::auth::handlers::{login_handler, register_handler, AuthState, LoginRequest, RegisterRequest};
use backend::config::Config;
use backend::database::model::UserRole;
use std::sync::Arc;
use tower::util::ServiceExt; // Correct import

mod common;

use backend::service::auth::AuthService;

#[tokio::test]
async fn test_register_and_login() {
    let (db, db_name) = common::setup_db().await;
    let config = Arc::new(Config {
        jwt_secret: "test_secret".to_string(),
        ..Default::default()
    });

    let auth_service = Arc::new(AuthService::new(db.clone(), config.clone()));

    let state = AuthState {
        service: auth_service,
    };

    let app = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(state);

    // 1. Register
    let register_payload = RegisterRequest {
        name: "Test User".to_string(),
        role: UserRole::Kitchen,
        unique_code: "KITCHEN_TEST".to_string(),
        password: "password123".to_string(),
        phone: Some("08123456789".to_string()),
        institution_name: None,
    };

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&register_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // 2. Login
    let login_payload = LoginRequest {
        unique_code: "KITCHEN_TEST".to_string(),
        password: "password123".to_string(),
    };

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&login_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 3. Login with wrong password
    let login_payload_wrong = LoginRequest {
        unique_code: "KITCHEN_TEST".to_string(),
        password: "wrongpassword".to_string(),
    };

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&login_payload_wrong).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    common::teardown_db(db, db_name).await;
}
