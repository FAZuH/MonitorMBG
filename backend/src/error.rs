//! Error handling for the application.
//!
//! This module defines the central [`AppError`] enum used throughout the backend.

use crate::database::error::DatabaseError;

/// Central error type for the application.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AppError {
    /// An assertion failed.
    #[error("Assertion error: {msg}")]
    AssertionError { msg: String },

    /// A required configuration value is missing.
    #[error("Missing config \"{config}\"")]
    MissingConfig { config: String },

    /// An error occurred during application configuration.
    #[error("Error in app configuration: {msg}")]
    ConfigurationError { msg: String },

    /// An internal server error occurred.
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    /// The user is not authorized to perform the action.
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// The user is forbidden from performing the action.
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// The request was invalid.
    #[error("Bad Request: {0}")]
    BadRequest(String),

    /// The requested resource was not found.
    #[error("Not Found: {0}")]
    NotFound(String),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::Unauthorized(msg) => (axum::http::StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (axum::http::StatusCode::FORBIDDEN, msg),
            AppError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (axum::http::StatusCode::NOT_FOUND, msg),
            AppError::InternalServerError(msg) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            _ => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
            ),
        };

        let body = axum::Json(serde_json::json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<DatabaseError> for AppError {
    fn from(err: DatabaseError) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

pub enum AppErrorKind {
    AppError(AppError),
}
