//! HTTP request handlers organized by interaction type.
//!
//! Handlers are organized into three categories:
//! - `pages`: GET-only handlers that render full page views
//! - `forms`: POST handlers that process form submissions with validation
//! - `actions`: POST/DELETE/PATCH handlers for state-changing operations
//!
//! All handlers return `Result<T, HandlerError>` for consistent error handling.

pub mod actions;
pub mod errors;
pub mod fallback;
pub mod forms;
pub mod htmx;
pub mod pages;
