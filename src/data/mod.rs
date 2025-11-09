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
/// ensure_rows_affected(result, "Todo")?;
/// ```
pub fn ensure_rows_affected(result: PgQueryResult, entity: &'static str) -> Result<(), DataError> {
    if result.rows_affected() == 0 {
        Err(DataError::NotFound(entity))
    } else {
        Ok(())
    }
}
