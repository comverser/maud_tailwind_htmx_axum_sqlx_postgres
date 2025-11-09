//! Database access layer following CQRS pattern.
//!
//! Separates data operations into:
//! - `commands`: Database mutations (INSERT, UPDATE, DELETE)
//! - `queries`: Database reads (SELECT)
//!
//! All operations return `Result<T, DataError>` for error handling.

pub mod commands;
pub mod errors;
pub mod queries;

use errors::DataError;
use sqlx::postgres::PgQueryResult;

/// Helper to check if a query affected any rows, returning an error if not.
///
/// This is commonly used to verify that an UPDATE or DELETE operation
/// actually found and modified a row, ensuring proper authorization.
///
/// # Example
/// ```
/// let result = sqlx::query!("DELETE FROM todos WHERE id = $1 AND user_id = $2", id, user_id)
///     .execute(db)
///     .await?;
/// ensure_rows_affected(result, "Todo not found")?;
/// ```
pub fn ensure_rows_affected(result: PgQueryResult, message: &'static str) -> Result<(), DataError> {
    if result.rows_affected() == 0 {
        Err(DataError::NotFound(message))
    } else {
        Ok(())
    }
}

/// Helper to map sqlx::Error::RowNotFound to DataError::NotFound.
///
/// This standardizes the common pattern of converting sqlx errors
/// when fetching a single row that may not exist.
///
/// # Example
/// ```
/// let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", id)
///     .fetch_one(db)
///     .await
///     .map_err(|e| map_row_not_found(e, "Todo not found"))?;
/// ```
pub fn map_row_not_found(error: sqlx::Error, message: &'static str) -> DataError {
    match error {
        sqlx::Error::RowNotFound => DataError::NotFound(message),
        _ => DataError::Database(error),
    }
}
