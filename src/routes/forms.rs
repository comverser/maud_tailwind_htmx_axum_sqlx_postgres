use crate::{config::AppState, handlers::forms, paths::forms::relative};
use axum::{Router, routing::post};

pub fn public_form_routes() -> Router<AppState> {
    Router::new()
        .route(relative::SIGN_IN, post(forms::post_forms_sign_in))
}

pub fn protected_form_routes() -> Router<AppState> {
    Router::new()
        .route(relative::TODOS, post(forms::post_forms_todos))
        .route(relative::TODOS_TODO_ID_TOGGLE, post(forms::post_forms_todos_todo_id_toggle))
}
