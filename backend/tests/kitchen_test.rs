use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::routing::get;
use backend::database::model::Kitchen;
use backend::database::model::KitchenType;
use backend::database::table::Table;
use backend::routes::kitchen::KitchenState;
use backend::routes::kitchen::get_kitchen_detail_handler;
use backend::routes::kitchen::get_kitchen_stats_handler;
use backend::routes::kitchen::get_multiple_kitchens_handler;
use backend::routes::kitchen::list_kitchens_handler;
use backend::service::kitchen::KitchenService;
use tower::util::ServiceExt;
use uuid::Uuid;

mod common;

#[tokio::test]
async fn test_list_kitchens_empty() {
    let (db, db_name) = common::setup_db().await;
    let kitchen_service = Arc::new(KitchenService::new(db.clone()));

    let state = KitchenState {
        service: kitchen_service,
    };

    let app = Router::new()
        .route("/", get(list_kitchens_handler))
        .with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_list_kitchens_with_data() {
    let (db, db_name) = common::setup_db().await;

    // Insert test kitchens
    let kitchen1 = Kitchen {
        name: "Test Kitchen 1".to_string(),
        city: Some("Jakarta".to_string()),
        province: Some("DKI Jakarta".to_string()),
        r#type: Some(KitchenType::CentralKitchen),
        meals_served: Some(1000),
        ..Default::default()
    };

    let kitchen2 = Kitchen {
        name: "Test Kitchen 2".to_string(),
        city: Some("Bandung".to_string()),
        province: Some("Jawa Barat".to_string()),
        r#type: Some(KitchenType::SatelliteKitchen),
        meals_served: Some(500),
        ..Default::default()
    };

    db.kitchen_table.insert(&kitchen1).await.unwrap();
    db.kitchen_table.insert(&kitchen2).await.unwrap();

    let kitchen_service = Arc::new(KitchenService::new(db.clone()));

    let state = KitchenState {
        service: kitchen_service,
    };

    let app = Router::new()
        .route("/", get(list_kitchens_handler))
        .with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_list_kitchens_with_filters() {
    let (db, db_name) = common::setup_db().await;

    // Insert test kitchens
    let kitchen1 = Kitchen {
        name: "Jakarta Central Kitchen".to_string(),
        city: Some("Jakarta".to_string()),
        province: Some("DKI Jakarta".to_string()),
        r#type: Some(KitchenType::CentralKitchen),
        meals_served: Some(1000),
        ..Default::default()
    };

    let kitchen2 = Kitchen {
        name: "Bandung School Kitchen".to_string(),
        city: Some("Bandung".to_string()),
        province: Some("Jawa Barat".to_string()),
        r#type: Some(KitchenType::SatelliteKitchen),
        meals_served: Some(500),
        ..Default::default()
    };

    db.kitchen_table.insert(&kitchen1).await.unwrap();
    db.kitchen_table.insert(&kitchen2).await.unwrap();

    let kitchen_service = Arc::new(KitchenService::new(db.clone()));

    let state = KitchenState {
        service: kitchen_service,
    };

    let app = Router::new()
        .route("/", get(list_kitchens_handler))
        .with_state(state);

    // Test location filter
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/?loc=Jakarta")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Test search query
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/?q=Central")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Test type filter
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/?type=central_kitchen")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_list_kitchens_pagination() {
    let (db, db_name) = common::setup_db().await;

    // Insert multiple test kitchens
    for i in 0..25 {
        let kitchen = Kitchen {
            name: format!("Test Kitchen {}", i),
            city: Some("Jakarta".to_string()),
            province: Some("DKI Jakarta".to_string()),
            r#type: Some(KitchenType::CentralKitchen),
            meals_served: Some(1000),
            ..Default::default()
        };
        db.kitchen_table.insert(&kitchen).await.unwrap();
    }

    let kitchen_service = Arc::new(KitchenService::new(db.clone()));

    let state = KitchenState {
        service: kitchen_service,
    };

    let app = Router::new()
        .route("/", get(list_kitchens_handler))
        .with_state(state);

    // Test with limit
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/?limit=10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Test with offset
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/?limit=10&offset=10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Test limit cap (should not exceed 100)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/?limit=200")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_get_kitchen_detail_success() {
    let (db, db_name) = common::setup_db().await;

    // Insert test kitchen
    let kitchen = Kitchen {
        name: "Test Kitchen Detail".to_string(),
        city: Some("Jakarta".to_string()),
        province: Some("DKI Jakarta".to_string()),
        r#type: Some(KitchenType::CentralKitchen),
        meals_served: Some(1000),
        address: Some("123 Test Street".to_string()),
        ..Default::default()
    };

    let kitchen_id = db.kitchen_table.insert(&kitchen).await.unwrap();

    let kitchen_service = Arc::new(KitchenService::new(db.clone()));

    let state = KitchenState {
        service: kitchen_service,
    };

    let app = Router::new()
        .route("/{id}", get(get_kitchen_detail_handler))
        .with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/{}", kitchen_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_get_kitchen_detail_not_found() {
    let (db, db_name) = common::setup_db().await;

    let kitchen_service = Arc::new(KitchenService::new(db.clone()));

    let state = KitchenState {
        service: kitchen_service,
    };

    let app = Router::new()
        .route("/{id}", get(get_kitchen_detail_handler))
        .with_state(state);

    let non_existent_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/{}", non_existent_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_get_kitchen_stats_success() {
    let (db, db_name) = common::setup_db().await;

    // Insert test kitchen
    let kitchen = Kitchen {
        name: "Test Kitchen Stats".to_string(),
        city: Some("Jakarta".to_string()),
        province: Some("DKI Jakarta".to_string()),
        r#type: Some(KitchenType::CentralKitchen),
        meals_served: Some(1000),
        ..Default::default()
    };

    let kitchen_id = db.kitchen_table.insert(&kitchen).await.unwrap();

    let kitchen_service = Arc::new(KitchenService::new(db.clone()));

    let state = KitchenState {
        service: kitchen_service,
    };

    let app = Router::new()
        .route("/{id}/stats", get(get_kitchen_stats_handler))
        .with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/{}/stats", kitchen_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_get_kitchen_stats_not_found() {
    let (db, db_name) = common::setup_db().await;

    let kitchen_service = Arc::new(KitchenService::new(db.clone()));

    let state = KitchenState {
        service: kitchen_service,
    };

    let app = Router::new()
        .route("/{id}/stats", get(get_kitchen_stats_handler))
        .with_state(state);

    let non_existent_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/{}/stats", non_existent_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_get_multiple_kitchens_success() {
    let (db, db_name) = common::setup_db().await;

    // Insert test kitchens
    let kitchen1 = Kitchen {
        name: "Kitchen 1".to_string(),
        city: Some("Jakarta".to_string()),
        province: Some("DKI Jakarta".to_string()),
        r#type: Some(KitchenType::CentralKitchen),
        meals_served: Some(1000),
        ..Default::default()
    };

    let kitchen2 = Kitchen {
        name: "Kitchen 2".to_string(),
        city: Some("Bandung".to_string()),
        province: Some("Jawa Barat".to_string()),
        r#type: Some(KitchenType::SatelliteKitchen),
        meals_served: Some(500),
        ..Default::default()
    };

    let id1 = db.kitchen_table.insert(&kitchen1).await.unwrap();
    let id2 = db.kitchen_table.insert(&kitchen2).await.unwrap();

    let kitchen_service = Arc::new(KitchenService::new(db.clone()));

    let state = KitchenState {
        service: kitchen_service,
    };

    let app = Router::new()
        .route("/batch", get(get_multiple_kitchens_handler))
        .with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/batch?ids={},{}", id1, id2))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_get_multiple_kitchens_invalid_ids() {
    let (db, db_name) = common::setup_db().await;

    let kitchen_service = Arc::new(KitchenService::new(db.clone()));

    let state = KitchenState {
        service: kitchen_service,
    };

    let app = Router::new()
        .route("/batch", get(get_multiple_kitchens_handler))
        .with_state(state);

    // Test with no IDs
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/batch?ids=")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Test with invalid UUIDs
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/batch?ids=invalid,not-a-uuid")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    common::teardown_db(db, db_name).await;
}
