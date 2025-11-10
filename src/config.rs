use axum::extract::FromRef;
use sqlx::PgPool;
use std::env::VarError;

/// Site name used in page titles and branding.
pub fn site_name() -> String {
    dotenvy::var("SITE_NAME").expect("SITE_NAME must be set")
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingVar(String),
    #[error("Invalid value for {0}: {1}")]
    InvalidValue(String, String),
}

impl From<VarError> for ConfigError {
    fn from(e: VarError) -> Self {
        match e {
            VarError::NotPresent => ConfigError::MissingVar("unknown".to_string()),
            VarError::NotUnicode(_) => ConfigError::InvalidValue("unknown".to_string(), "not valid unicode".to_string()),
        }
    }
}

pub struct AppConfig {
    server_addr: String,
    database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url = dotenvy::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingVar("DATABASE_URL".to_string()))?;

        let server_addr = dotenvy::var("SERVER_ADDR")
            .map_err(|_| ConfigError::MissingVar("SERVER_ADDR".to_string()))?;

        Ok(Self {
            server_addr,
            database_url,
        })
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
