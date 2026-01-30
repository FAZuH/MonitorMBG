use std::sync::Arc;

use axum::Router;
use axum::extract::Query;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::routing::get;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::service::stats::StatsService;

#[derive(Clone)]
pub struct StatsState {
    pub service: Arc<StatsService>,
}

#[derive(Deserialize)]
pub struct NationalStatsQuery {
    pub year: Option<i32>,
    pub month: Option<i32>,
}

#[derive(Deserialize)]
pub struct RegionalStatsQuery {
    pub province: Option<String>,
    pub kabupaten: Option<String>,
    pub year: Option<i32>,
    pub month: Option<i32>,
}

#[derive(Deserialize)]
pub struct ComplianceTrendsQuery {
    pub province: Option<String>,
    pub kabupaten: Option<String>,
    #[serde(rename = "kitchenId")]
    pub kitchen_id: Option<Uuid>,
    pub months: Option<i32>,
}

#[derive(Deserialize)]
pub struct IncidentTrendsQuery {
    pub province: Option<String>,
    pub months: Option<i32>,
    #[serde(rename = "groupBy")]
    pub group_by: Option<String>,
}

pub async fn get_national_stats_handler(
    State(state): State<StatsState>,
    Query(query): Query<NationalStatsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .get_national_stats(query.year, query.month)
        .await?;
    Ok(Json(response))
}

pub async fn get_regional_stats_handler(
    State(state): State<StatsState>,
    Query(query): Query<RegionalStatsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .get_regional_stats(query.province, query.kabupaten, query.year, query.month)
        .await?;
    Ok(Json(response))
}

pub async fn get_compliance_trends_handler(
    State(state): State<StatsState>,
    Query(query): Query<ComplianceTrendsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .get_compliance_trends(
            query.province,
            query.kabupaten,
            query.kitchen_id,
            query.months,
        )
        .await?;
    Ok(Json(response))
}

pub async fn get_incident_trends_handler(
    State(state): State<StatsState>,
    Query(query): Query<IncidentTrendsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .get_incident_trends(query.province, query.months, query.group_by)
        .await?;
    Ok(Json(response))
}

pub fn stats_routes(state: StatsState) -> Router {
    Router::new()
        .route("/national", get(get_national_stats_handler))
        .route("/regional", get(get_regional_stats_handler))
        .route("/trends/compliance", get(get_compliance_trends_handler))
        .route("/trends/incidents", get(get_incident_trends_handler))
        .with_state(state)
}
