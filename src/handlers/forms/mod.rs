mod sign_in;
mod todo;

pub use sign_in::post_forms_sign_in;
pub use todo::post_forms_todos;

use axum::{http::StatusCode, response::{IntoResponse, Response}};
use maud::Markup;
use std::collections::HashMap;

pub(super) fn parse_validation_errors(
    validation_errors: &validator::ValidationErrors,
) -> HashMap<String, String> {
    validation_errors
        .field_errors()
        .iter()
        .filter_map(|(field, errs)| {
            errs.first()
                .and_then(|e| e.message.as_ref())
                .map(|msg| (field.to_string(), msg.to_string()))
        })
        .collect()
}

/// Helper for rendering validation error responses with BAD_REQUEST status.
///
/// This encapsulates the common pattern of parsing validation errors
/// and returning them with a 400 status code.
pub(super) fn render_validation_error_response<F>(
    validation_errors: &validator::ValidationErrors,
    render_fn: F,
) -> Response
where
    F: FnOnce(HashMap<String, String>) -> Markup,
{
    let errors = parse_validation_errors(validation_errors);
    (StatusCode::BAD_REQUEST, render_fn(errors)).into_response()
}
