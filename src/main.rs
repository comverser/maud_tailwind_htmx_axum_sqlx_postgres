//! Web application template with type-first routing architecture.
//!
//! Routes are organized by interaction type rather than resource,
//! enabling clear separation between views, form submissions, and state changes.

mod auth;
mod config;
mod constants;
mod data;
mod email;
mod email_templates;
mod flash;
mod formatting;
mod handlers;
mod init;
mod magic_link;
mod middlewares;
mod models;
mod paths;
mod routes;
mod validation;
mod views;

use config::{AppConfig, AppState};

#[tokio::main]
async fn main() {
    init::init_logging();

    dotenvy::dotenv().ok();

    let config = AppConfig::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {}", e);
        eprintln!("\nPlease check your .env file and ensure all required variables are set.");
        eprintln!("Required: DATABASE_URL, SERVER_ADDR, SITE_NAME");
        std::process::exit(1);
    });

    let db = init::init_database(config.database_url()).await;
    let session_layer = init::init_session(db.clone()).await;

    let server_addr = config.server_addr().to_string();
    let state = AppState::new(db, config);

    let listener = tokio::net::TcpListener::bind(&server_addr)
        .await
        .unwrap_or_else(|e| {
            eprintln!(
                "Failed to bind to address {}: {}",
                server_addr,
                e
            );
            eprintln!("\nIs another process already using this port?");
            std::process::exit(1);
        });

    tracing::info!("Server listening on {}", server_addr);

    let app = routes::create_routes(state, session_layer)
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {}", e);
        std::process::exit(1);
    }
}
