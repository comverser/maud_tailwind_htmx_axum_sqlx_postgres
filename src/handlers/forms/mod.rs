mod sign_in;
mod sign_up;
mod todo;

pub use sign_in::post_forms_sign_in;
pub use sign_up::post_forms_sign_up;
pub use todo::{post_forms_todos, post_forms_todos_todo_id_toggle};

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
