//! Incident reporting routes.

use std::sync::Arc;

use axum::Router;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::routing::get;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::service::incident::IncidentService;

/// State for incident routes.
#[derive(Clone)]
pub struct IncidentState {
    /// The incident service.
    pub service: Arc<IncidentService>,
}

#[derive(Deserialize)]
pub struct ListIncidentsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub status: Option<String>,
    pub province: Option<String>,
    #[serde(rename = "dateFrom")]
    pub date_from: Option<String>,
    #[serde(rename = "dateTo")]
    pub date_to: Option<String>,
    #[serde(rename = "minVictims")]
    pub min_victims: Option<i32>,
}

/// Handler for listing incidents.
pub async fn list_incidents_handler(

    State(state): State<IncidentState>,
    Query(query): Query<ListIncidentsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let limit = query.limit.unwrap_or(100).min(500);
    let offset = query.offset.unwrap_or(0);

    let response = state
        .service
        .list_incidents(
            limit,
            offset,
            query.status,
            query.province,
            query.date_from,
            query.date_to,
            query.min_victims,
        )
        .await?;

    Ok(Json(response))
}

pub async fn get_incident_detail_handler(
    State(state): State<IncidentState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.service.get_incident_detail(id).await?;
    Ok(Json(response))
}

pub fn incident_routes(state: IncidentState) -> Router {
    Router::new()
        .route("/", get(list_incidents_handler))
        .route("/:id", get(get_incident_detail_handler))
        .with_state(state)
}
