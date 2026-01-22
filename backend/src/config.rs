use crate::error::AppError;

#[derive(Clone, Default)]
pub struct Config {
    pub db_url: String,
    pub logs_path: String,
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
        Ok(())
    }
}
