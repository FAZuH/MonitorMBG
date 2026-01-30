//! Logging initialization and configuration.
//!
//! This module sets up `tracing` with both stdout and rolling file appenders.

use tracing_appender::rolling::RollingFileAppender;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::error::AppError;

/// Initializes the logging system.
///
/// Sets up a rolling file appender and a stdout appender using `tracing-subscriber`.
///
/// # Errors
///
/// Returns [`AppError::ConfigurationError`] if the logs directory cannot be created
/// or if the file appender fails to initialize.
pub fn setup_logging(config: &Config) -> Result<(), AppError> {
    // Create logs directory if it doesn't exist
    std::fs::create_dir_all(&config.logs_path).map_err(|e| AppError::ConfigurationError {
        msg: format!(
            "Failed to create logs directory at '{}': {}",
            config.logs_path, e
        ),
    })?;

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("monitor_mbg")
        .filename_suffix("log")
        .max_log_files(7)
        .build(&config.logs_path)
        .map_err(|e| AppError::ConfigurationError {
            msg: format!(
                "Failed to initialize rolling file appender at '{}': {}",
                config.logs_path, e
            ),
        })?;

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Leak the guard to prevent it from being dropped
    std::mem::forget(_guard);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("monitor_mbg=info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_writer(std::io::stdout).with_ansi(true))
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();

    Ok(())
}
