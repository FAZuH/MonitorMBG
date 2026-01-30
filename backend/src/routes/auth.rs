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
use crate::service::otp::OtpService;

/// State for authentication routes.
#[derive(Clone)]
pub struct AuthState {
    /// The authentication service.
    pub service: Arc<AuthService>,
    /// The OTP service for phone verification.
    pub otp_service: Arc<OtpService>,
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

/// Request to send OTP to a phone number.
#[derive(Deserialize)]
pub struct SendOtpRequest {
    pub phone: String,
}

/// Response after sending OTP.
#[derive(Serialize)]
pub struct SendOtpResponse {
    pub success: bool,
    pub message: String,
    #[serde(rename = "referenceId")]
    pub reference_id: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: u64,
}

/// Request to verify an OTP code.
#[derive(Deserialize)]
pub struct VerifyOtpRequest {
    pub phone: String,
    pub code: String,
    #[serde(rename = "referenceId")]
    pub reference_id: String,
}

/// Response after verifying OTP.
#[derive(Serialize)]
pub struct VerifyOtpResponse {
    pub success: bool,
    pub message: String,
    pub verified: bool,
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

/// Handler for sending OTP to a phone number.
pub async fn send_otp_handler(
    State(state): State<AuthState>,
    Json(payload): Json<SendOtpRequest>,
) -> Result<impl IntoResponse, AppError> {
    let phone = payload.phone.trim().to_string();

    let (reference_id, expires_in) = state.otp_service.send_otp(phone).await?;

    Ok(Json(SendOtpResponse {
        success: true,
        message: "OTP sent via WhatsApp".to_string(),
        reference_id,
        expires_in,
    }))
}

/// Handler for verifying an OTP code.
pub async fn verify_otp_handler(
    State(state): State<AuthState>,
    Json(payload): Json<VerifyOtpRequest>,
) -> Result<impl IntoResponse, AppError> {
    let phone = payload.phone.trim().to_string();
    let code = payload.code.trim().to_string();
    let reference_id = payload.reference_id.trim().to_string();

    let verified = state
        .otp_service
        .verify_otp(&reference_id, &phone, &code)
        .await?;

    if verified {
        Ok(Json(VerifyOtpResponse {
            success: true,
            message: "Phone verified successfully".to_string(),
            verified: true,
        }))
    } else {
        Err(AppError::BadRequest("Invalid OTP code".to_string()))
    }
}

pub fn auth_routes(state: AuthState) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/otp/send", post(send_otp_handler))
        .route("/otp/verify", post(verify_otp_handler))
        .with_state(state)
}
