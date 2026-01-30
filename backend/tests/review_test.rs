use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::routing::{get, post, patch, delete};
use axum::middleware;
use backend::auth::middleware::{AuthState as MiddlewareAuthState, auth_middleware};
use backend::auth::utils::{generate_token};
use backend::config::Config;
use backend::database::model::{Kitchen, User, UserRole, Review};
use backend::database::table::Table;
use backend::routes::review::ReviewState;
use backend::routes::review::{submit_review_handler, get_kitchen_reviews_handler, get_public_reviews_handler, submit_batch_reviews_handler, update_review_handler, delete_review_handler};
use backend::service::auth::AuthService;
use backend::service::review::{ReviewService, CreateReviewRequest, HaccpRatingDto, UpdateReviewRequest};
use tower::util::ServiceExt;
use uuid::Uuid;

mod common;

async fn setup_auth_state(db: Arc<backend::database::Database>) -> (Arc<AuthService>, MiddlewareAuthState, String) {
    let config = Arc::new(Config {
        jwt_secret: "test_secret".to_string(),
        ..Default::default()
    });

    let auth_service = Arc::new(AuthService::new(db.clone(), config.clone()));
    
    let middleware_state = MiddlewareAuthState {
        config: config.clone(),
    };

    (auth_service, middleware_state, config.jwt_secret.clone())
}

#[tokio::test]
async fn test_submit_review_success() {
    let (db, db_name) = common::setup_db().await;

    let user = User {
        name: "Test User".to_string(),
        unique_code: "TEST001".to_string(),
        role: UserRole::School,
        ..Default::default()
    };
    let user_id = db.user_table.insert(&user).await.unwrap();

    let kitchen = Kitchen {
        name: "Test Kitchen".to_string(),
        city: Some("Jakarta".to_string()),
        province: Some("DKI Jakarta".to_string()),
        ..Default::default()
    };
    let kitchen_id = db.kitchen_table.insert(&kitchen).await.unwrap();

    let review_service = Arc::new(ReviewService::new(db.clone()));
    let (_, middleware_state, jwt_secret) = setup_auth_state(db.clone()).await;

    let state = ReviewState {
        service: review_service,
    };

    let token = generate_token(user_id, UserRole::School, &jwt_secret).unwrap();

    let app = Router::new()
        .route("/", post(submit_review_handler))
        .layer(middleware::from_fn_with_state(
            middleware_state,
            auth_middleware,
        ))
        .with_state(state);

    let review_request = CreateReviewRequest {
        kitchen_id,
        reviewer_name: "Test Reviewer".to_string(),
        reviewer_type: UserRole::School,
        ratings: HaccpRatingDto {
            taste: 4.5,
            hygiene: 4.0,
            freshness: 5.0,
            temperature: 4.5,
            packaging: 4.0,
            handling: 4.5,
        },
        comment: "Great food!".to_string(),
        photos: None,
        delivery_date: None,
        meal_type: None,
    };

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::from(serde_json::to_string(&review_request).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_submit_review_unauthorized() {
    let (db, db_name) = common::setup_db().await;

    let kitchen = Kitchen {
        name: "Test Kitchen".to_string(),
        city: Some("Jakarta".to_string()),
        province: Some("DKI Jakarta".to_string()),
        ..Default::default()
    };
    let kitchen_id = db.kitchen_table.insert(&kitchen).await.unwrap();

    let review_service = Arc::new(ReviewService::new(db.clone()));
    let (_, middleware_state, _) = setup_auth_state(db.clone()).await;

    let state = ReviewState {
        service: review_service,
    };

    let app = Router::new()
        .route("/", post(submit_review_handler))
        .layer(middleware::from_fn_with_state(
            middleware_state,
            auth_middleware,
        ))
        .with_state(state);

    let review_request = CreateReviewRequest {
        kitchen_id,
        reviewer_name: "Test Reviewer".to_string(),
        reviewer_type: UserRole::School,
        ratings: HaccpRatingDto {
            taste: 4.5,
            hygiene: 4.0,
            freshness: 5.0,
            temperature: 4.5,
            packaging: 4.0,
            handling: 4.5,
        },
        comment: "Great food!".to_string(),
        photos: None,
        delivery_date: None,
        meal_type: None,
    };

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&review_request).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/")
                .header("Content-Type", "application/json")
                .header("Authorization", "Bearer invalid_token")
                .body(Body::from(serde_json::to_string(&review_request).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_get_kitchen_reviews() {
    let (db, db_name) = common::setup_db().await;

    let user = User {
        name: "Test User".to_string(),
        unique_code: "TEST001".to_string(),
        role: UserRole::School,
        ..Default::default()
    };
    let user_id = db.user_table.insert(&user).await.unwrap();

    let kitchen = Kitchen {
        name: "Test Kitchen".to_string(),
        city: Some("Jakarta".to_string()),
        province: Some("DKI Jakarta".to_string()),
        ..Default::default()
    };
    let kitchen_id = db.kitchen_table.insert(&kitchen).await.unwrap();

    for i in 0..5 {
        let review = Review {
            kitchen_id,
            reviewer_id: user_id,
            reviewer_name: format!("Reviewer {}", i),
            comment: format!("Comment {}", i),
            ..Default::default()
        };
        db.review_table.insert(&review).await.unwrap();
    }

    let review_service = Arc::new(ReviewService::new(db.clone()));

    let state = ReviewState {
        service: review_service,
    };

    let app = Router::new()
        .route("/kitchen/:kitchenId", get(get_kitchen_reviews_handler))
        .with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/kitchen/{}", kitchen_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_get_public_reviews() {
    let (db, db_name) = common::setup_db().await;

    let user = User {
        name: "Test User".to_string(),
        unique_code: "TEST001".to_string(),
        role: UserRole::School,
        ..Default::default()
    };
    let user_id = db.user_table.insert(&user).await.unwrap();

    let kitchen = Kitchen {
        name: "Test Kitchen".to_string(),
        city: Some("Jakarta".to_string()),
        province: Some("DKI Jakarta".to_string()),
        ..Default::default()
    };
    let kitchen_id = db.kitchen_table.insert(&kitchen).await.unwrap();

    for i in 0..3 {
        let review = Review {
            kitchen_id,
            reviewer_id: user_id,
            reviewer_name: format!("Reviewer {}", i),
            comment: format!("Comment {}", i),
            verified: Some(true),
            ..Default::default()
        };
        db.review_table.insert(&review).await.unwrap();
    }

    let review_service = Arc::new(ReviewService::new(db.clone()));

    let state = ReviewState {
        service: review_service,
    };

    let app = Router::new()
        .route("/public", get(get_public_reviews_handler))
        .with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/public")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_update_review_success() {
    let (db, db_name) = common::setup_db().await;

    let user = User {
        name: "Test User".to_string(),
        unique_code: "TEST001".to_string(),
        role: UserRole::School,
        ..Default::default()
    };
    let user_id = db.user_table.insert(&user).await.unwrap();

    let kitchen = Kitchen {
        name: "Test Kitchen".to_string(),
        city: Some("Jakarta".to_string()),
        province: Some("DKI Jakarta".to_string()),
        ..Default::default()
    };
    let kitchen_id = db.kitchen_table.insert(&kitchen).await.unwrap();

    let review = Review {
        kitchen_id,
        reviewer_id: user_id,
        reviewer_name: "Test Reviewer".to_string(),
        comment: "Original comment".to_string(),
        verified: Some(false),
        ..Default::default()
    };
    let review_id = db.review_table.insert(&review).await.unwrap();

    let review_service = Arc::new(ReviewService::new(db.clone()));
    let (_, middleware_state, jwt_secret) = setup_auth_state(db.clone()).await;

    let state = ReviewState {
        service: review_service,
    };

    let token = generate_token(user_id, UserRole::School, &jwt_secret).unwrap();

    let app = Router::new()
        .route("/:id", patch(update_review_handler))
        .layer(middleware::from_fn_with_state(
            middleware_state,
            auth_middleware,
        ))
        .with_state(state);

    let update_request = UpdateReviewRequest {
        ratings: Some(HaccpRatingDto {
            taste: 5.0,
            hygiene: 5.0,
            freshness: 5.0,
            temperature: 5.0,
            packaging: 5.0,
            handling: 5.0,
        }),
        comment: Some("Updated comment".to_string()),
        photos: None,
    };

    let response = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/{}", review_id))
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::from(serde_json::to_string(&update_request).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_delete_review_success() {
    let (db, db_name) = common::setup_db().await;

    let user = User {
        name: "Test User".to_string(),
        unique_code: "TEST001".to_string(),
        role: UserRole::School,
        ..Default::default()
    };
    let user_id = db.user_table.insert(&user).await.unwrap();

    let kitchen = Kitchen {
        name: "Test Kitchen".to_string(),
        city: Some("Jakarta".to_string()),
        province: Some("DKI Jakarta".to_string()),
        ..Default::default()
    };
    let kitchen_id = db.kitchen_table.insert(&kitchen).await.unwrap();

    let review = Review {
        kitchen_id,
        reviewer_id: user_id,
        reviewer_name: "Test Reviewer".to_string(),
        comment: "Test comment".to_string(),
        verified: Some(false),
        ..Default::default()
    };
    let review_id = db.review_table.insert(&review).await.unwrap();

    let review_service = Arc::new(ReviewService::new(db.clone()));
    let (_, middleware_state, jwt_secret) = setup_auth_state(db.clone()).await;

    let state = ReviewState {
        service: review_service,
    };

    let token = generate_token(user_id, UserRole::School, &jwt_secret).unwrap();

    let app = Router::new()
        .route("/:id", delete(delete_review_handler))
        .layer(middleware::from_fn_with_state(
            middleware_state,
            auth_middleware,
        ))
        .with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/{}", review_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}
