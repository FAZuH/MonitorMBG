use std::sync::Arc;
use std::time::Instant;

use axum::Router;
use backend::auth::handlers::{auth_routes, AuthState};
use backend::config::Config;
use backend::database::Database;
use backend::logging::setup_logging;
use backend::middleware::rate_limit::{rate_limit, RateLimitMiddleware};
use backend::service::auth::AuthService;
use dotenv::dotenv;
use log::debug;
use log::info;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let init_start = Instant::now();
    dotenv().ok();

    let mut config = Config::new();
    config.load()?;
    let config = Arc::new(config);

    setup_logging(&config)?;

    info!("Starting MonitorMBG (backend)...");

    // Setup database
    debug!("Setting up Database...");
    let db = Arc::new(Database::new(&config.db_url).await?);
    info!("Running database migrations...");
    db.run_migrations().await?;
    info!(
        "Database setup complete ({:.2}s).",
        init_start.elapsed().as_secs_f64()
    );

    // Setup Services
    let auth_service = Arc::new(AuthService::new(db.clone(), config.clone()));

    // Setup Auth State
    let auth_state = AuthState {
        service: auth_service,
    };

    // Setup Rate Limit
    let rl_state = RateLimitMiddleware::new(100);

    // Setup Router
    let app = Router::new()
        .nest("/auth", auth_routes(auth_state))
        .layer(axum::middleware::from_fn_with_state(rl_state, rate_limit));

    // Start Server
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on {}", listener.local_addr()?);

    let init_done = init_start.elapsed();
    info!(
        "MonitorMBG (backend) is up in {:.2}s.",
        init_done.as_secs_f64()
    );

    axum::serve(listener, app).await?;

    Ok(())
}
