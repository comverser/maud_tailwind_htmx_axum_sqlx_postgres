//! Application initialization functions.
//!
//! Provides setup for:
//! - `init_logging`: Configures tracing subscriber with environment-based filtering
//! - `init_database`: Establishes PostgreSQL connection pool and runs migrations
//! - `init_session`: Sets up session management with PostgreSQL storage

mod database;
mod logging;
mod session;

pub use database::init_database;
pub use logging::init_logging;
pub use session::init_session;
