use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::routing::post;
use backend::config::Config;
use backend::database::model::UserRole;
use backend::routes::auth::AuthState;
use backend::routes::auth::LoginRequest;
use backend::routes::auth::RegisterRequest;
use backend::routes::auth::login_handler;
use backend::routes::auth::register_handler;
use tower::util::ServiceExt;

mod common;

use backend::service::auth::AuthService;

#[tokio::test]
async fn test_auth_validation_and_security() {
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

    // 1. Test Weak Password Registration
    let weak_password_payload = RegisterRequest {
        name: "Weak User".to_string(),
        role: UserRole::Kitchen,
        unique_code: "WEAK_USER".to_string(),
        password: "short".to_string(), // Too short
        phone: None,
        institution_name: None,
    };

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/register")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&weak_password_payload).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // 2. Test Long Password Registration (DoS prevention)
    let long_password = "a".repeat(33);
    let long_password_payload = RegisterRequest {
        name: "Long User".to_string(),
        role: UserRole::Kitchen,
        unique_code: "LONG_USER".to_string(),
        password: long_password,
        phone: None,
        institution_name: None,
    };

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/register")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&long_password_payload).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // 3. Test User Enumeration / Timing Attack Mitigation
    // We can't easily test timing in unit tests reliably, but we can ensure it returns 401 Unauthorized
    // instead of 404 Not Found or similar for non-existent users.
    let non_existent_login = LoginRequest {
        unique_code: "NON_EXISTENT".to_string(),
        password: "password123".to_string(),
    };

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&non_existent_login).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    common::teardown_db(db, db_name).await;
}
