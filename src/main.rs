//! Web application template using Axum, HTMX, Tailwind CSS, and PostgreSQL.
//!
//! This template provides a clean starting point for building web applications with:
//! - Type-first routing organized by interaction type (pages, forms, actions)
//! - RESTful API design within each route type
//! - Magic link authentication
//! - Session management with PostgreSQL storage
//! - Compile-time HTML templates using Maud

mod auth;
mod config;
mod data;
mod email;
mod flash;
mod handlers;
mod init;
mod magic_link;
mod middlewares;
mod paths;
mod routes;
mod views;

use config::{AppConfig, AppState};

#[tokio::main]
async fn main() {
    init::init_logging();

    let config = AppConfig::from_env();
    let db = init::init_database(&config.database_url).await;
    let session_layer = init::init_session(db.clone()).await;
    let state = AppState { db };

    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .unwrap();

    tracing::info!("Server listening on {}", config.server_addr);

    let app = routes::create_routes(state, session_layer)
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    axum::serve(listener, app).await.unwrap();
}
