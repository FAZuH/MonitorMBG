use std::num::NonZeroU32;
use std::sync::Arc;

use axum::extract::Request;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use governor::Quota;
use governor::RateLimiter;
use governor::clock::DefaultClock;
use governor::state::InMemoryState;
use governor::state::NotKeyed;

#[derive(Clone)]
pub struct RateLimitMiddleware {
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl RateLimitMiddleware {
    pub fn new(requests_per_second: u32) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(requests_per_second).unwrap());
        let limiter = Arc::new(RateLimiter::direct(quota));
        Self { limiter }
    }
}

pub async fn rate_limit(
    State(state): State<RateLimitMiddleware>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if state.limiter.check().is_err() {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    Ok(next.run(req).await)
}
