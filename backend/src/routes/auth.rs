//! Authentication routes.

use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::routing::post;
use serde::Deserialize;
use serde::Serialize;

use crate::database::model::User;
use crate::database::model::UserRole;
use crate::error::AppError;
use crate::service::auth::AuthService;

/// State for authentication routes.
#[derive(Clone)]
pub struct AuthState {
    /// The authentication service.
    pub service: Arc<AuthService>,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterRequest {
    pub name: String,
    pub role: UserRole,
    pub unique_code: String,
    pub password: String,
    pub phone: Option<String>,
    pub institution_name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub unique_code: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

/// Handler for user registration.
pub async fn register_handler(

    State(state): State<AuthState>,
    Json(mut payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Sanitize Input
    payload.name = payload.name.trim().to_string();
    payload.unique_code = payload.unique_code.trim().to_string();

    // Input Validation
    if payload.password.len() < 8 {
        return Err(AppError::BadRequest(
            "Password must be at least 8 characters long".to_string(),
        ));
    }
    if payload.password.len() > 32 {
        return Err(AppError::BadRequest(
            "Password must be less than 32 characters long".to_string(),
        ));
    }
    if payload.unique_code.is_empty() || payload.unique_code.len() > 50 {
        return Err(AppError::BadRequest(
            "Unique code must be between 1 and 50 characters".to_string(),
        ));
    }
    if payload.name.is_empty() || payload.name.len() > 255 {
        return Err(AppError::BadRequest(
            "Name must be between 1 and 255 characters".to_string(),
        ));
    }

    let (token, user) = state
        .service
        .register_user(
            payload.name,
            payload.role,
            payload.unique_code,
            payload.password,
            payload.phone,
            payload.institution_name,
        )
        .await?;

    Ok((StatusCode::CREATED, Json(AuthResponse { token, user })))
}

pub async fn login_handler(
    State(state): State<AuthState>,
    Json(mut payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Sanitize Input
    payload.unique_code = payload.unique_code.trim().to_string();

    // Input Validation
    if payload.password.len() > 32 {
        return Err(AppError::BadRequest("Password too long".to_string()));
    }

    let (token, user) = state
        .service
        .login_user(payload.unique_code, payload.password)
        .await?;

    Ok(Json(AuthResponse { token, user }))
}

pub fn auth_routes(state: AuthState) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(state)
}
