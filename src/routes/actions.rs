use crate::{config::AppState, handlers::actions, paths::actions::relative};
use axum::{Router, routing::{delete, post}};

pub fn protected_action_routes() -> Router<AppState> {
    Router::new()
        .route(relative::SIGN_OUT, post(actions::post_actions_sign_out))
        .route(relative::TODOS_TODO_ID, delete(actions::delete_actions_todos_todo_id))
}
