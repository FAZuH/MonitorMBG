use crate::error::AppError;

#[derive(Clone, Default)]
pub struct Config {
    pub db_url: String,
    pub logs_path: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

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

        Ok(())
    }
}
