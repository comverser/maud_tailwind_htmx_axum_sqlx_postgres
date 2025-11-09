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
