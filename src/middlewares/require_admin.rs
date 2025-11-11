use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};
use crate::{auth::CurrentUser, constants::errors};

pub async fn require_admin(req: Request, next: Next) -> axum::response::Response {
    match req.extensions().get::<CurrentUser>() {
        Some(current_user) if current_user.is_admin() => next.run(req).await,
        _ => (StatusCode::FORBIDDEN, errors::FORBIDDEN).into_response(),
    }
}
