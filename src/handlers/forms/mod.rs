mod sign_in;
mod todo;

pub use sign_in::post_forms_sign_in;
pub use todo::post_forms_todos;

use std::collections::HashMap;

/// Parses validator validation errors into a simple field → message map.
///
/// Takes the first error message for each field and returns a HashMap
/// mapping field names to their error messages. This is used when
/// re-rendering forms with validation errors.
///
/// # Example
/// ```text
/// ValidationErrors { email: ["Invalid email format", "Email too long"] }
/// → HashMap { "email": "Invalid email format" }
/// ```
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
