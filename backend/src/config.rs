//! Configuration management for the application.
//!
//! This module handles loading configuration from environment variables.

use crate::error::AppError;

/// Storage configuration for file uploads.
#[derive(Clone, Default)]
pub struct StorageConfig {
    /// Type of storage backend: "local" or "s3".
    pub storage_type: String,
    /// For local storage: path to store files.
    pub local_path: Option<String>,
    /// For S3 storage: bucket name.
    pub bucket: Option<String>,
    /// For S3 storage: AWS region.
    pub region: Option<String>,
    /// For S3 storage: access key ID.
    pub access_key: Option<String>,
    /// For S3 storage: secret access key.
    pub secret_key: Option<String>,
    /// For S3 storage: custom endpoint URL (for MinIO, etc.).
    pub endpoint: Option<String>,
    /// Base URL for generating file URLs.
    pub base_url: Option<String>,
}

/// Application configuration structure.
#[derive(Clone, Default)]
pub struct Config {
    /// Database connection URL.
    pub db_url: String,
    /// Path to the directory where logs will be stored.
    pub logs_path: String,
    /// Secret key used for JWT signing and verification.
    pub jwt_secret: String,
    /// Host address to bind the server to.
    pub host: String,
    /// Port number to bind the server to.
    pub port: u16,
    /// Storage configuration for file uploads.
    pub storage: StorageConfig,
}

impl Config {
    /// Creates a new default configuration.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Loads configuration from environment variables.
    ///
    /// # Errors
    ///
    /// Returns [`AppError::MissingConfig`] if `JWT_SECRET` is not set.
    /// Returns [`AppError::ConfigurationError`] if `PORT` is not a valid number.
    pub fn load(&mut self) -> Result<(), AppError> {
        self.db_url = std::env::var("DATABASE_URL")
            .unwrap_or("postgres://postgres:password@localhost:5432/my_database".to_string());
        self.logs_path = std::env::var("LOGS_PATH").unwrap_or("./logs".to_string());

        // JWT Secret must be provided in production-like environments
        // For dev convenience, we can fallback, but let's make it explicit or warn
        self.jwt_secret = std::env::var("JWT_SECRET").map_err(|_| AppError::MissingConfig {
            config: "JWT_SECRET".to_string(),
        })?;

        self.host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
        self.port = std::env::var("PORT")
            .unwrap_or("3000".to_string())
            .parse()
            .map_err(|_| AppError::ConfigurationError {
                msg: "PORT must be a number".to_string(),
            })?;

        // Load storage configuration
        self.storage = StorageConfig {
            storage_type: std::env::var("STORAGE_TYPE").unwrap_or("local".to_string()),
            local_path: std::env::var("STORAGE_LOCAL_PATH").ok(),
            bucket: std::env::var("STORAGE_S3_BUCKET").ok(),
            region: std::env::var("STORAGE_S3_REGION").ok(),
            access_key: std::env::var("STORAGE_S3_ACCESS_KEY").ok(),
            secret_key: std::env::var("STORAGE_S3_SECRET_KEY").ok(),
            endpoint: std::env::var("STORAGE_S3_ENDPOINT").ok(),
            base_url: std::env::var("STORAGE_BASE_URL").ok(),
        };

        Ok(())
    }
}
