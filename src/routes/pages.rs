use crate::{config::AppState, handlers::pages, paths};
use axum::{Router, routing::get};

pub fn public_page_routes() -> Router<AppState> {
    Router::new()
        .route(paths::pages::ROOT, get(pages::get_root))
        .route(paths::pages::ABOUT, get(pages::get_about))
        .route(paths::pages::SIGN_UP, get(pages::get_sign_up))
        .route(paths::pages::SIGN_IN, get(pages::get_sign_in))
}

pub fn protected_page_routes() -> Router<AppState> {
    Router::new()
        .route(paths::pages::TODOS, get(pages::get_todos))
}
