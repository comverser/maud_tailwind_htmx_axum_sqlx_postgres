use sqlx::PgPool;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;

use crate::constants;

pub async fn init_session(db: PgPool) -> SessionManagerLayer<PostgresStore> {
    let session_store = PostgresStore::new(db);

    session_store
        .migrate()
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to initialize session storage: {}", e);
            eprintln!("\nThis usually means the sessions table couldn't be created.");
            eprintln!("The database connection is working, but there may be a permissions issue.");
            std::process::exit(1);
        });

    SessionManagerLayer::new(session_store)
        .with_expiry(tower_sessions::Expiry::OnInactivity(time::Duration::days(constants::auth::SESSION_EXPIRY_DAYS)))
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
}
