mod actions;
mod forms;
mod pages;

use axum::{Router, middleware};
use tower_http::services::ServeDir;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;

use crate::{config::AppState, handlers, middlewares, paths};

pub fn create_routes(state: AppState, session_layer: SessionManagerLayer<PostgresStore>) -> Router {
    Router::new()
        .nest_service(paths::static_files::BASE, ServeDir::new("static"))
        .merge(app_routes(state, session_layer))
        .layer(middlewares::create_http_trace_layer())
}

fn app_routes(state: AppState, session_layer: SessionManagerLayer<PostgresStore>) -> Router {
    Router::new()
        .merge(public_routes())
        .merge(protected_routes())
        .fallback(handlers::fallback::handle_404)
        .with_state(state)
        .layer(middleware::from_fn(middlewares::session_context))
        .layer(session_layer)
}

fn public_routes() -> Router<AppState> {
    Router::new()
        .merge(pages::public_page_routes())
        .nest(paths::forms::BASE, forms::public_form_routes())
}

fn protected_routes() -> Router<AppState> {
    Router::new()
        .merge(pages::protected_page_routes())
        .nest(paths::forms::BASE, forms::protected_form_routes())
        .nest(paths::actions::BASE, actions::protected_action_routes())
        .layer(middleware::from_fn(middlewares::require_authentication))
}
