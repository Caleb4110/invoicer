use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Network timeout")]
    Timeout,
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::Database(format!("Database error: {}", e))
    }
}
