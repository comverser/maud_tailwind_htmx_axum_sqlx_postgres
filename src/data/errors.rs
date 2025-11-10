use thiserror::Error;

/// Errors at the data access layer.
///
/// Separates low-level database errors from application-level semantic errors
/// (not found, unauthorized) for clearer error handling in handlers.
#[derive(Error, Debug)]
pub enum DataError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("{0}")]
    NotFound(&'static str),

    #[error("{0}")]
    Unauthorized(&'static str),
}
