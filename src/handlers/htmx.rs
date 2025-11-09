//! HTMX response helpers for consistent interaction patterns.
//!
//! This module provides helper functions for common HTMX response patterns,
//! making handler code more expressive and reducing duplication.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::Markup;

/// Returns an empty 200 OK response.
///
/// Used when HTMX should swap the target with empty content,
/// effectively removing the element from the DOM.
///
/// # Example
/// ```
/// // Delete handler that removes element from page
/// pub async fn delete_item() -> Response {
///     // ... delete logic ...
///     htmx::empty_ok()
/// }
/// ```
pub fn empty_ok() -> Response {
    StatusCode::OK.into_response()
}

/// Returns HTML content for HTMX to swap into the DOM.
///
/// Use with `hx-swap` attributes to update page content dynamically.
///
/// # Example
/// ```
/// // Toggle handler that returns updated item HTML
/// pub async fn toggle_item() -> Response {
///     let updated_item = // ... fetch updated item
///     htmx::swap_html(views::item(&updated_item))
/// }
/// ```
pub fn swap_html(markup: Markup) -> Response {
    markup.into_response()
}

/// Returns a 204 No Content response.
///
/// Tells HTMX to do nothing - useful for operations that don't need
/// to update the UI (like analytics tracking).
pub fn no_content() -> Response {
    StatusCode::NO_CONTENT.into_response()
}
