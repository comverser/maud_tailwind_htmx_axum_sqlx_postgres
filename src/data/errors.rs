use thiserror::Error;

/// Errors that occur at the data access layer.
///
/// This separates database errors from application-level errors (not found, unauthorized).
/// These errors bubble up to handlers and are converted to appropriate HTTP responses.
///
/// # Error Types
/// - `Database`: Low-level SQLx errors (connection failures, SQL syntax errors, etc.)
/// - `NotFound`: Resource exists check failed (e.g., todo not found)
/// - `Unauthorized`: Permission check failed (e.g., user doesn't own the resource)
#[derive(Error, Debug)]
pub enum DataError {
    /// Low-level database error (SQLx)
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    /// Resource not found (maps to 404 NOT FOUND)
    #[error("{0}")]
    NotFound(&'static str),

    /// Permission denied (maps to 401 UNAUTHORIZED)
    #[error("{0}")]
    Unauthorized(&'static str),
}
