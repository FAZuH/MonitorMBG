//! Review and rating routes.

use std::sync::Arc;

use axum::Extension;
use axum::Router;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::middleware;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::routing::get;
use axum::routing::patch;
use axum::routing::post;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::middleware::AuthState as MiddlewareAuthState;
use crate::auth::middleware::auth_middleware;
use crate::auth::utils::Claims;
use crate::error::AppError;
use crate::service::review::CreateReviewRequest;
use crate::service::review::ReviewService;
use crate::service::review::UpdateReviewRequest;

/// State for review routes.
#[derive(Clone)]
pub struct ReviewState {
    /// The review service.
    pub service: Arc<ReviewService>,
}

#[derive(Deserialize)]
pub struct ListReviewsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub verified: Option<bool>,
    #[serde(rename = "minRating")]
    pub min_rating: Option<f64>,
    #[serde(rename = "reviewerType")]
    pub reviewer_type: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

#[derive(Deserialize)]
pub struct PublicReviewsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub province: Option<String>,
    #[serde(rename = "minRating")]
    pub min_rating: Option<f64>,
}

#[derive(Deserialize)]
pub struct BatchReviewsRequest {
    pub reviews: Vec<CreateReviewRequest>,
}

/// Handler for submitting a review.
pub async fn submit_review_handler(
    State(state): State<ReviewState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateReviewRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.service.submit_review(claims.sub, payload).await?;
    Ok((axum::http::StatusCode::CREATED, Json(response)))
}

pub async fn get_kitchen_reviews_handler(
    State(state): State<ReviewState>,
    Path(kitchen_id): Path<Uuid>,
    Query(query): Query<ListReviewsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    let response = state
        .service
        .get_kitchen_reviews(kitchen_id, limit, offset)
        .await?;
    Ok(Json(response))
}

pub async fn get_public_reviews_handler(
    State(state): State<ReviewState>,
    Query(query): Query<PublicReviewsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    let response = state.service.get_public_reviews(limit, offset).await?;
    Ok(Json(response))
}

pub async fn submit_batch_reviews_handler(
    State(state): State<ReviewState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<BatchReviewsRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .submit_batch_reviews(claims.sub, payload.reviews)
        .await?;
    Ok((axum::http::StatusCode::CREATED, Json(response)))
}

pub async fn update_review_handler(
    State(state): State<ReviewState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateReviewRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.service.update_review(id, claims.sub, payload).await?;
    Ok(Json(serde_json::json!({
        "id": response.id,
        "message": "Review updated successfully"
    })))
}

pub async fn delete_review_handler(
    State(state): State<ReviewState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.service.delete_review(id, claims.sub).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Review deleted successfully"
    })))
}

pub fn review_routes(state: ReviewState, auth_middleware_state: MiddlewareAuthState) -> Router {
    let protected_routes = Router::new()
        .route("/", post(submit_review_handler))
        .route("/batch", post(submit_batch_reviews_handler))
        .route(
            "/:id",
            patch(update_review_handler).delete(delete_review_handler),
        )
        .layer(middleware::from_fn_with_state(
            auth_middleware_state,
            auth_middleware,
        ));

    let public_routes = Router::new()
        .route("/kitchen/:kitchenId", get(get_kitchen_reviews_handler))
        .route("/public", get(get_public_reviews_handler));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
