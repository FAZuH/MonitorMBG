use std::sync::Arc;
use std::time::Instant;

use axum::Router;
use backend::auth::middleware::AuthState as MiddlewareAuthState;
use backend::config::Config;
use backend::database::Database;
use backend::logging::setup_logging;
use backend::middleware::rate_limit::RateLimitMiddleware;
use backend::middleware::rate_limit::rate_limit;
use backend::routes::auth::AuthState;
use backend::routes::auth::auth_routes;
use backend::routes::incident::IncidentState;
use backend::routes::incident::incident_routes;
use backend::routes::kitchen::KitchenState;
use backend::routes::kitchen::kitchen_routes;
use backend::routes::review::ReviewState;
use backend::routes::review::review_routes;
use backend::routes::stats::StatsState;
use backend::routes::stats::stats_routes;
use backend::routes::utility::UtilityState;
use backend::routes::utility::utility_routes;
use backend::service::auth::AuthService;
use backend::service::incident::IncidentService;
use backend::service::kitchen::KitchenService;
use backend::service::review::ReviewService;
use backend::service::stats::StatsService;
use backend::service::utility::UtilityService;
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

    // Setup routes
    let api_routes = setup_routes(config.clone(), db.clone());

    // Setup Rate Limit
    let rl_state = RateLimitMiddleware::new(100);

    let app = Router::new()
        .nest("/api", api_routes)
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

fn setup_routes(config: Arc<Config>, db: Arc<Database>) -> Router {
    // Setup Services
    let auth_service = Arc::new(AuthService::new(db.clone(), config.clone()));
    let kitchen_service = Arc::new(KitchenService::new(db.pool.clone()));
    let review_service = Arc::new(ReviewService::new(db.pool.clone()));
    let incident_service = Arc::new(IncidentService::new(db.pool.clone()));
    let stats_service = Arc::new(StatsService::new(db.pool.clone()));
    let utility_service = Arc::new(UtilityService::new(db.pool.clone()));

    // Setup States
    let auth_state = AuthState {
        service: auth_service,
    };
    let kitchen_state = KitchenState {
        service: kitchen_service,
    };
    let review_state = ReviewState {
        service: review_service,
    };
    let incident_state = IncidentState {
        service: incident_service,
    };
    let stats_state = StatsState {
        service: stats_service,
    };
    let utility_state = UtilityState {
        service: utility_service,
    };

    let middleware_auth_state = MiddlewareAuthState {
        config: config.clone(),
    };

    // Setup Router
    Router::new()
        .nest("/auth", auth_routes(auth_state))
        .nest("/kitchens", kitchen_routes(kitchen_state))
        .nest(
            "/reviews",
            review_routes(review_state, middleware_auth_state.clone()),
        )
        .nest("/incidents", incident_routes(incident_state))
        .nest("/stats", stats_routes(stats_state))
        .nest("/", utility_routes(utility_state, middleware_auth_state))
}
