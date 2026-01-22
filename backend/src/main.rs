pub mod config;
pub mod error;
pub mod logging;

use std::sync::Arc;
use std::time::Instant;

use backend::db::Database;
use dotenv::dotenv;
use log::debug;
use log::info;

use crate::config::Config;
use crate::logging::setup_logging;

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

    // Listen for exit signal
    let init_done = init_start.elapsed();
    info!(
        "MonitorMBG (backend) is up in {:.2}s. Press Ctrl+C to stop.",
        init_done.as_secs_f64()
    );
    tokio::signal::ctrl_c().await?;
    info!("Ctrl+C received, shutting down.");
    // Stop publishers

    Ok(())
}
