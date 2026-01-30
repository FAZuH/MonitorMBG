use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::routing::get;
use backend::database::model::{Incident, IncidentType, IncidentSource, IncidentSeverity, IncidentStatus, Kitchen};
use backend::database::table::Table;
use backend::routes::incident::IncidentState;
use backend::routes::incident::{list_incidents_handler, get_incident_detail_handler};
use backend::service::incident::IncidentService;
use tower::util::ServiceExt;
use uuid::Uuid;

mod common;

#[tokio::test]
async fn test_list_incidents_empty() {
    let (db, db_name) = common::setup_db().await;
    let incident_service = Arc::new(IncidentService::new(db.clone()));

    let state = IncidentState {
        service: incident_service,
    };

    let app = Router::new()
        .route("/", get(list_incidents_handler))
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
async fn test_list_incidents_with_data() {
    let (db, db_name) = common::setup_db().await;

    let kitchen = Kitchen {
        name: "Test Kitchen".to_string(),
        ..Default::default()
    };
    let kitchen_id = db.kitchen_table.insert(&kitchen).await.unwrap();

    let incident1 = Incident {
        kitchen_id,
        r#type: IncidentType::Poisoning,
        source: IncidentSource::Consumer,
        severity: IncidentSeverity::Critical,
        description: Some("Food poisoning reported".to_string()),
        affected_count: Some(10),
        deaths: Some(0),
        status: Some(IncidentStatus::Investigating),
        ..Default::default()
    };

    let incident2 = Incident {
        kitchen_id,
        r#type: IncidentType::Sanitation,
        source: IncidentSource::Inspector,
        severity: IncidentSeverity::Major,
        description: Some("Sanitation issue found".to_string()),
        affected_count: Some(5),
        deaths: Some(0),
        status: Some(IncidentStatus::Resolved),
        ..Default::default()
    };

    db.incident_table.insert(&incident1).await.unwrap();
    db.incident_table.insert(&incident2).await.unwrap();

    let incident_service = Arc::new(IncidentService::new(db.clone()));

    let state = IncidentState {
        service: incident_service,
    };

    let app = Router::new()
        .route("/", get(list_incidents_handler))
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
async fn test_get_incident_detail_success() {
    let (db, db_name) = common::setup_db().await;

    let kitchen = Kitchen {
        name: "Test Kitchen".to_string(),
        ..Default::default()
    };
    let kitchen_id = db.kitchen_table.insert(&kitchen).await.unwrap();

    let incident = Incident {
        kitchen_id,
        r#type: IncidentType::Poisoning,
        source: IncidentSource::Consumer,
        severity: IncidentSeverity::Critical,
        description: Some("Critical incident".to_string()),
        affected_count: Some(15),
        deaths: Some(0),
        status: Some(IncidentStatus::Investigating),
        ..Default::default()
    };

    let incident_id = db.incident_table.insert(&incident).await.unwrap();

    let incident_service = Arc::new(IncidentService::new(db.clone()));

    let state = IncidentState {
        service: incident_service,
    };

    let app = Router::new()
        .route("/{id}", get(get_incident_detail_handler))
        .with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/{}", incident_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    common::teardown_db(db, db_name).await;
}

#[tokio::test]
async fn test_get_incident_detail_not_found() {
    let (db, db_name) = common::setup_db().await;

    let incident_service = Arc::new(IncidentService::new(db.clone()));

    let state = IncidentState {
        service: incident_service,
    };

    let app = Router::new()
        .route("/{id}", get(get_incident_detail_handler))
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
