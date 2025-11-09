use sqlx::{
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
    ConnectOptions,
};
use std::str::FromStr;

pub async fn init_database(database_url: &str) -> PgPool {
    let options = PgConnectOptions::from_str(database_url)
        .expect("Failed to parse database URL")
        .disable_statement_logging();

    let db = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run migrations");

    db
}
