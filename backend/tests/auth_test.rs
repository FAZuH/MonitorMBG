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
use backend::service::otp::OtpService;

#[tokio::test]
async fn test_register_and_login_success() {
    let (db, db_name) = common::setup_db().await;
    let config = Arc::new(Config {
        jwt_secret: "test_secret".to_string(),
        ..Default::default()
    });

    let auth_service = Arc::new(AuthService::new(db.clone(), config.clone()));

    let otp_service = Arc::new(OtpService::new(config.clone()));
    let state = AuthState {
        service: auth_service,
        otp_service,
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
                .body(Body::from(
                    serde_json::to_string(&register_payload).unwrap(),
                ))
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
                .body(Body::from(
                    serde_json::to_string(&login_payload_wrong).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_register_duplicate_unique_code() {
    let (db, db_name) = common::setup_db().await;
    let config = Arc::new(Config {
        jwt_secret: "test_secret".to_string(),
        ..Default::default()
    });

    let auth_service = Arc::new(AuthService::new(db.clone(), config.clone()));

    let otp_service = Arc::new(OtpService::new(config.clone()));
    let state = AuthState {
        service: auth_service,
        otp_service,
    };

    let app = Router::new()
        .route("/register", post(register_handler))
        .with_state(state);

    // First registration should succeed
    let register_payload = RegisterRequest {
        name: "Test User".to_string(),
        role: UserRole::Kitchen,
        unique_code: "DUPLICATE_CODE".to_string(),
        password: "password123".to_string(),
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
                    serde_json::to_string(&register_payload).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // Second registration with same unique code should fail
    let register_payload2 = RegisterRequest {
        name: "Another User".to_string(),
        role: UserRole::School,
        unique_code: "DUPLICATE_CODE".to_string(),
        password: "password456".to_string(),
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
                    serde_json::to_string(&register_payload2).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_register_validation_errors() {
    let (db, db_name) = common::setup_db().await;
    let config = Arc::new(Config {
        jwt_secret: "test_secret".to_string(),
        ..Default::default()
    });

    let auth_service = Arc::new(AuthService::new(db.clone(), config.clone()));

    let otp_service = Arc::new(OtpService::new(config.clone()));
    let state = AuthState {
        service: auth_service,
        otp_service,
    };

    let app = Router::new()
        .route("/register", post(register_handler))
        .with_state(state);

    // Test password too short
    let register_payload = RegisterRequest {
        name: "Test User".to_string(),
        role: UserRole::Kitchen,
        unique_code: "TEST001".to_string(),
        password: "short".to_string(),
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
                    serde_json::to_string(&register_payload).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Test password too long
    let register_payload = RegisterRequest {
        name: "Test User".to_string(),
        role: UserRole::Kitchen,
        unique_code: "TEST002".to_string(),
        password: "a".repeat(33),
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
                    serde_json::to_string(&register_payload).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Test empty unique code
    let register_payload = RegisterRequest {
        name: "Test User".to_string(),
        role: UserRole::Kitchen,
        unique_code: "".to_string(),
        password: "password123".to_string(),
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
                    serde_json::to_string(&register_payload).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Test empty name
    let register_payload = RegisterRequest {
        name: "".to_string(),
        role: UserRole::Kitchen,
        unique_code: "TEST003".to_string(),
        password: "password123".to_string(),
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
                    serde_json::to_string(&register_payload).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_login_nonexistent_user() {
    let (db, db_name) = common::setup_db().await;
    let config = Arc::new(Config {
        jwt_secret: "test_secret".to_string(),
        ..Default::default()
    });

    let auth_service = Arc::new(AuthService::new(db.clone(), config.clone()));

    let otp_service = Arc::new(OtpService::new(config.clone()));
    let state = AuthState {
        service: auth_service,
        otp_service,
    };

    let app = Router::new()
        .route("/login", post(login_handler))
        .with_state(state);

    // Login with non-existent user
    let login_payload = LoginRequest {
        unique_code: "NONEXISTENT".to_string(),
        password: "password123".to_string(),
    };

    let response = app
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

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_register_different_roles() {
    let (db, db_name) = common::setup_db().await;
    let config = Arc::new(Config {
        jwt_secret: "test_secret".to_string(),
        ..Default::default()
    });

    let auth_service = Arc::new(AuthService::new(db.clone(), config.clone()));

    let otp_service = Arc::new(OtpService::new(config.clone()));
    let state = AuthState {
        service: auth_service,
        otp_service,
    };

    let app = Router::new()
        .route("/register", post(register_handler))
        .with_state(state);

    let roles = vec![
        (UserRole::Admin, "ADMIN001"),
        (UserRole::Kitchen, "KITCHEN001"),
        (UserRole::School, "REVIEWER001"),
    ];

    for (role, code) in roles {
        let register_payload = RegisterRequest {
            name: format!("{:?} User", role),
            role,
            unique_code: code.to_string(),
            password: "password123".to_string(),
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
                        serde_json::to_string(&register_payload).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::CREATED,
            "Failed to register user with role {:?}",
            role
        );
    }

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_register_input_sanitization() {
    let (db, db_name) = common::setup_db().await;
    let config = Arc::new(Config {
        jwt_secret: "test_secret".to_string(),
        ..Default::default()
    });

    let auth_service = Arc::new(AuthService::new(db.clone(), config.clone()));

    let otp_service = Arc::new(OtpService::new(config.clone()));
    let state = AuthState {
        service: auth_service,
        otp_service,
    };

    let app = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(state);

    // Register with whitespace in name and unique_code
    let register_payload = RegisterRequest {
        name: "  Test User  ".to_string(),
        role: UserRole::Kitchen,
        unique_code: "  SANITIZE_TEST  ".to_string(),
        password: "password123".to_string(),
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
                    serde_json::to_string(&register_payload).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // Login with whitespace (should be sanitized and work)
    let login_payload = LoginRequest {
        unique_code: "  SANITIZE_TEST  ".to_string(),
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

    common::teardown_db(db, db_name).await;
}
