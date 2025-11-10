use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::{auth::CurrentUser, constants::error_pages, data::errors::DataError, views::pages::server_error};

/// Handler-level errors that can occur during request processing.
///
/// This wraps lower-level errors (database, session) and provides a consistent
/// error response via `IntoResponse`. All handler errors are converted to appropriate
/// HTTP responses with user-friendly error pages.
#[derive(Error, Debug)]
pub enum HandlerError {
    /// Database or data access errors (not found, unauthorized, database failures)
    #[error("{0}")]
    Data(#[from] DataError),

    /// Session management errors (read/write failures)
    #[error("{0}")]
    Session(#[from] tower_sessions::session::Error),
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::Data(DataError::NotFound(msg)) => (StatusCode::NOT_FOUND, *msg),
            Self::Data(DataError::Unauthorized(msg)) => (StatusCode::UNAUTHORIZED, *msg),
            Self::Data(DataError::Database(e)) => {
                tracing::error!(error = %e, "Database error in handler");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Self::Session(e) => {
                tracing::error!(error = %e, "Session error in handler");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        (status, server_error::server_error(&CurrentUser::Guest, None, error_pages::FALLBACK_SITE_NAME, message)).into_response()
    }
}
