mod auth;
mod config;
mod data;
mod flash;
mod handlers;
mod init;
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

    let app = routes::create_routes(state, session_layer);

    axum::serve(listener, app).await.unwrap();
}
