//! Application routing configuration.
//!
//! Routes are organized by interaction type (pages, forms, actions) and protection level.
//! Authentication enforcement is applied at the route group level via middleware.

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
        .layer(middleware::from_fn(middlewares::security_headers))
        .layer(middlewares::create_http_trace_layer())
}

fn app_routes(state: AppState, session_layer: SessionManagerLayer<PostgresStore>) -> Router {
    Router::new()
        .merge(public_routes())
        .merge(protected_routes())
        .fallback(handlers::fallback::handle_404)
        .with_state(state)
        // CRITICAL: Middleware ordering matters! Layers are applied bottom-to-top (last to first).
        // Execution order during a request is: session_layer → session_context → handler
        //
        // 1. session_layer: Tower-sessions provides the Session extractor
        // 2. session_context: Loads CurrentUser from session and injects via Extension
        //
        // This ordering ensures CurrentUser is available to all handlers.
        // See also: protected_routes() for authentication enforcement.
        .layer(middleware::from_fn(middlewares::session_context))
        .layer(session_layer)
}

/// Public routes accessible to all users (authenticated and guests)
fn public_routes() -> Router<AppState> {
    Router::new()
        .merge(pages::public_page_routes())
        .nest(paths::forms::BASE, forms::public_form_routes())
        .nest(paths::actions::BASE, actions::public_action_routes())
}

/// Protected routes requiring authentication
///
/// The `require_authentication` middleware intercepts requests and:
/// - Redirects guests to the sign-in page
/// - Allows authenticated users to proceed to handlers
///
/// This middleware MUST be applied last (outermost) so it runs first during request processing.
/// It relies on `CurrentUser` being loaded by the `session_context` middleware.
fn protected_routes() -> Router<AppState> {
    Router::new()
        .merge(pages::protected_page_routes())
        .nest(paths::forms::BASE, forms::protected_form_routes())
        .nest(paths::actions::BASE, actions::protected_action_routes())
        // This layer runs BEFORE the handler, ensuring only authenticated users proceed
        .layer(middleware::from_fn(middlewares::require_authentication))
}
