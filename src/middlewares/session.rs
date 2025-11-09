use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};
use tower_sessions::Session;

use crate::{auth::{CurrentUser, SESSION_USER_ID_KEY}, flash::FlashMessage};

pub async fn session_context(session: Session, mut req: Request, next: Next) -> axum::response::Response {
    let current_user = match session.get::<i32>(SESSION_USER_ID_KEY).await {
        Ok(Some(user_id)) => CurrentUser::Authenticated { user_id },
        Ok(None) => CurrentUser::Guest,
        Err(e) => {
            tracing::error!("Failed to read user_id from session: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Session error").into_response();
        }
    };

    let flash = match FlashMessage::get(&session).await {
        Ok(flash) => flash,
        Err(e) => {
            tracing::error!("Failed to read flash message from session: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Session error").into_response();
        }
    };

    req.extensions_mut().insert(current_user);
    req.extensions_mut().insert(flash);
    next.run(req).await
}
