//! Authentication middleware.

use std::sync::Arc;

use axum::extract::Request;
use axum::extract::State;
use axum::http::StatusCode;
use axum::http::header;
use axum::middleware::Next;
use axum::response::Response;

use crate::auth::utils::validate_token;
use crate::config::Config;

/// State for the authentication middleware.
#[derive(Clone)]
pub struct AuthState {
    /// Application configuration, containing the JWT secret.
    pub config: Arc<Config>,
}

/// Middleware that validates the JWT in the `Authorization` header.
///
/// If the token is valid, the [`Claims`] are inserted into the request extensions.
/// Otherwise, it returns `401 Unauthorized`.
pub async fn auth_middleware(
    State(state): State<AuthState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    match validate_token(token, &state.config.jwt_secret) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
