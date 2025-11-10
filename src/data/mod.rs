//! Database access layer following CQRS pattern.
//!
//! Separates data operations into commands (mutations) and queries (reads)
//! for clearer intent and better organization.

pub mod commands;
pub mod errors;
pub mod queries;

use errors::DataError;
use sqlx::postgres::PgQueryResult;

/// Verifies that a database operation affected rows.
///
/// Used to ensure mutations (UPDATE/DELETE) with WHERE clauses
/// actually found matching rows, which validates authorization when user_id is in the clause.
pub fn ensure_rows_affected(result: PgQueryResult, message: &'static str) -> Result<(), DataError> {
    if result.rows_affected() == 0 {
        Err(DataError::NotFound(message))
    } else {
        Ok(())
    }
}

/// Maps sqlx::Error::RowNotFound to DataError::NotFound.
pub fn map_row_not_found(error: sqlx::Error, message: &'static str) -> DataError {
    match error {
        sqlx::Error::RowNotFound => DataError::NotFound(message),
        _ => DataError::Database(error),
    }
}

/// Maps sqlx::Error::RowNotFound to DataError::Unauthorized.
///
/// Use when a missing row indicates authorization failure rather than simple not-found.
pub fn map_row_unauthorized(error: sqlx::Error, message: &'static str) -> DataError {
    match error {
        sqlx::Error::RowNotFound => DataError::Unauthorized(message),
        _ => DataError::Database(error),
    }
}
