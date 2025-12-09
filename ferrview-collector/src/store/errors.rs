use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database not initialized")]
    NotInitialized,

    #[error("Invalid query: {0}")]
    InvalidQuery(String),
}
