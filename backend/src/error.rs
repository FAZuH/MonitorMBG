#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AppError {
    #[error("Assertion error: {msg}")]
    AssertionError { msg: String },

    #[error("Missing config \"{config}\"")]
    MissingConfig { config: String },

    #[error("Error in app configuration: {msg}")]
    ConfigurationError { msg: String },
}

pub enum AppErrorKind {
    AppError(AppError),
}
