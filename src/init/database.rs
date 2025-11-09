use sqlx::{
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
    ConnectOptions,
};
use std::str::FromStr;

pub async fn init_database(database_url: &str) -> PgPool {
    let options = PgConnectOptions::from_str(database_url).unwrap_or_else(|e| {
        eprintln!("Failed to parse DATABASE_URL: {}", e);
        eprintln!("\nThe DATABASE_URL format should be:");
        eprintln!("  postgresql://username:password@localhost:5432/database_name");
        eprintln!("\nPlease check your .env file.");
        std::process::exit(1);
    }).disable_statement_logging();

    let db = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect_with(options)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect to database: {}", e);
            eprintln!("\nTroubleshooting:");
            eprintln!("  1. Is PostgreSQL running?");
            eprintln!("  2. Are the credentials in DATABASE_URL correct?");
            eprintln!("  3. Does the database exist?");
            eprintln!("  4. Is the database accepting connections on the specified host/port?");
            std::process::exit(1);
        });

    sqlx::migrate!()
        .run(&db)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to run database migrations: {}", e);
            eprintln!("\nTroubleshooting:");
            eprintln!("  1. Check if migration files in migrations/ are valid SQL");
            eprintln!("  2. Verify database schema hasn't been manually modified");
            eprintln!("  3. If needed, drop and recreate the database to start fresh");
            std::process::exit(1);
        });

    db
}
