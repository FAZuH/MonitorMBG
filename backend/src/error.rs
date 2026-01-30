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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[test]
    fn test_error_display_messages() {
        let err = AppError::Unauthorized("Invalid credentials".to_string());
        assert!(err.to_string().contains("Invalid credentials"));

        let err = AppError::BadRequest("Missing field".to_string());
        assert!(err.to_string().contains("Missing field"));

        let err = AppError::NotFound("User not found".to_string());
        assert!(err.to_string().contains("User not found"));

        let err = AppError::Forbidden("Access denied".to_string());
        assert!(err.to_string().contains("Access denied"));

        let err = AppError::InternalServerError("Database error".to_string());
        assert!(err.to_string().contains("Database error"));
    }

    #[test]
    fn test_error_into_response_unauthorized() {
        let err = AppError::Unauthorized("Invalid token".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_error_into_response_bad_request() {
        let err = AppError::BadRequest("Invalid input".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_error_into_response_not_found() {
        let err = AppError::NotFound("Resource missing".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_error_into_response_forbidden() {
        let err = AppError::Forbidden("No permission".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_error_into_response_internal() {
        let err = AppError::InternalServerError("Server error".to_string());
        let response = err.into_response();
        assert_eq!(
            response.status(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_error_from_database_error() {
        let db_err = DatabaseError::InternalError {
            message: "Connection failed".to_string(),
        };
        let app_err: AppError = db_err.into();
        match app_err {
            AppError::InternalServerError(msg) => assert!(msg.contains("Connection failed")),
            _ => panic!("Expected InternalServerError"),
        }
    }

    #[test]
    fn test_assertion_error_display() {
        let err = AppError::AssertionError {
            msg: "Test failed".to_string(),
        };
        assert!(err.to_string().contains("Test failed"));
    }

    #[test]
    fn test_missing_config_error_display() {
        let err = AppError::MissingConfig {
            config: "DATABASE_URL".to_string(),
        };
        assert!(err.to_string().contains("DATABASE_URL"));
    }

    #[test]
    fn test_configuration_error_display() {
        let err = AppError::ConfigurationError {
            msg: "Invalid config".to_string(),
        };
        assert!(err.to_string().contains("Invalid config"));
    }
}
