use std::sync::Arc;

use axum::Router;
use axum::extract::Multipart;
use axum::extract::State;
use axum::middleware;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::routing::get;
use axum::routing::post;

use crate::auth::middleware::AuthState as MiddlewareAuthState;
use crate::auth::middleware::auth_middleware;
use crate::error::AppError;
use crate::service::utility::UtilityService;

#[derive(Clone)]
pub struct UtilityState {
    pub service: Arc<UtilityService>,
}

pub async fn health_check_handler(
    State(state): State<UtilityState>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.service.health_check().await?;
    Ok(Json(response))
}

pub async fn upload_image_handler(
    State(state): State<UtilityState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    // Simple single file upload handling
    while let Some(field) =
        multipart
            .next_field()
            .await
            .map_err(|e: axum::extract::multipart::MultipartError| {
                AppError::BadRequest(e.to_string())
            })?
    {
        let name = field.name().unwrap_or("file").to_string();
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        if name == "file" {
            let data =
                field
                    .bytes()
                    .await
                    .map_err(|e: axum::extract::multipart::MultipartError| {
                        AppError::BadRequest(e.to_string())
                    })?;

            if data.len() > 5 * 1024 * 1024 {
                return Err(AppError::BadRequest("File too large".into()));
            }

            let response = state
                .service
                .upload_image(file_name, data.to_vec(), content_type)
                .await?;
            return Ok((axum::http::StatusCode::CREATED, Json(response)));
        }
    }

    Err(AppError::BadRequest("No file uploaded".into()))
}

pub async fn upload_multiple_images_handler(
    State(state): State<UtilityState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut files = Vec::new();
    let mut file_count = 0;

    while let Some(field) =
        multipart
            .next_field()
            .await
            .map_err(|e: axum::extract::multipart::MultipartError| {
                AppError::BadRequest(e.to_string())
            })?
    {
        let name = field.name().unwrap_or("files").to_string();
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        // Handle both "files" and "files[]" field names
        if name == "files" || name.starts_with("files[") {
            if file_count >= 5 {
                return Err(AppError::BadRequest("Maximum 5 files allowed".into()));
            }

            let data =
                field
                    .bytes()
                    .await
                    .map_err(|e: axum::extract::multipart::MultipartError| {
                        AppError::BadRequest(e.to_string())
                    })?;

            if data.len() > 5 * 1024 * 1024 {
                return Err(AppError::BadRequest(format!(
                    "File '{}' exceeds 5MB limit",
                    file_name
                )));
            }

            files.push((file_name, data.to_vec(), content_type));
            file_count += 1;
        }
    }

    if files.is_empty() {
        return Err(AppError::BadRequest("No files uploaded".into()));
    }

    let response = state.service.upload_multiple_images(files).await?;
    Ok((axum::http::StatusCode::CREATED, Json(response)))
}

pub fn utility_routes(state: UtilityState, auth_middleware_state: MiddlewareAuthState) -> Router {
    let protected_routes = Router::new()
        .route("/upload/image", post(upload_image_handler))
        .route("/upload/images", post(upload_multiple_images_handler))
        .layer(middleware::from_fn_with_state(
            auth_middleware_state,
            auth_middleware,
        ));

    let public_routes = Router::new().route("/health", get(health_check_handler));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
