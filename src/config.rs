use axum::extract::FromRef;
use sqlx::PgPool;

pub struct AppConfig {
    server_addr: String,
    database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url = dotenvy::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let server_addr = dotenvy::var("SERVER_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:8000".to_string());

        Self {
            server_addr,
            database_url,
        }
    }

    pub fn server_addr(&self) -> &str {
        &self.server_addr
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}

#[derive(Clone, FromRef)]
pub struct AppState {
    db: PgPool,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
