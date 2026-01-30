//! Kitchen management routes.

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
use crate::service::kitchen::KitchenService;

/// State for kitchen routes.
#[derive(Clone)]
pub struct KitchenState {
    /// The kitchen service.
    pub service: Arc<KitchenService>,
}

#[derive(Deserialize)]
pub struct ListKitchensQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub q: Option<String>,
    pub loc: Option<String>,
    pub r#type: Option<String>,
    #[serde(rename = "minRating")]
    pub min_rating: Option<f64>,
}

#[derive(Deserialize)]
pub struct BatchKitchensQuery {
    pub ids: String,
}

/// Handler for listing kitchens.
pub async fn list_kitchens_handler(
    State(state): State<KitchenState>,
    Query(query): Query<ListKitchensQuery>,
) -> Result<impl IntoResponse, AppError> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    let response = state
        .service
        .list_kitchens(
            limit,
            offset,
            query.q,
            query.loc,
            query.r#type,
            query.min_rating,
        )
        .await?;

    Ok(Json(response))
}

pub async fn get_kitchen_detail_handler(
    State(state): State<KitchenState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.service.get_kitchen_detail(id).await?;
    Ok(Json(response))
}

pub async fn get_kitchen_stats_handler(
    State(state): State<KitchenState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.service.get_kitchen_stats(id).await?;
    Ok(Json(response))
}

pub async fn get_multiple_kitchens_handler(
    State(state): State<KitchenState>,
    Query(query): Query<BatchKitchensQuery>,
) -> Result<impl IntoResponse, AppError> {
    let ids: Vec<Uuid> = query
        .ids
        .split(',')
        .filter_map(|s: &str| Uuid::parse_str(s.trim()).ok())
        .take(50)
        .collect();

    if ids.is_empty() {
        return Err(AppError::BadRequest("No valid IDs provided".into()));
    }

    let kitchens = state.service.get_multiple_kitchens(ids).await?;

    // The API schema expects { data: [...], notFound: [...] }
    // For now, just returning the list as per service implementation
    // We can enhance this later to match exact schema if needed

    Ok(Json(serde_json::json!({ "data": kitchens })))
}

pub fn kitchen_routes(state: KitchenState) -> Router {
    Router::new()
        .route("/", get(list_kitchens_handler))
        .route("/batch", get(get_multiple_kitchens_handler))
        .route("/:id", get(get_kitchen_detail_handler))
        .route("/:id/stats", get(get_kitchen_stats_handler))
        .with_state(state)
}
