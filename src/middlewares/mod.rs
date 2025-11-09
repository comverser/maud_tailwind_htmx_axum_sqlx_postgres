//! Axum middleware for request/response processing.
//!
//! Middleware layers (applied in order):
//! 1. `security_headers`: Adds security headers (CSP, HSTS, etc.)
//! 2. HTTP tracing: Logs request/response details
//! 3. `session_context`: Extracts user from session into CurrentUser extension
//! 4. Session management: Tower sessions layer
//! 5. `require_authentication`: Protects routes requiring authentication

mod auth;
mod http_tracing;
mod security_headers;
mod session;

pub use auth::require_authentication;
pub use http_tracing::create_http_trace_layer;
pub use security_headers::security_headers;
pub use session::session_context;
