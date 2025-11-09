use axum::response::Response;
use tower_sessions::Session;

use crate::{flash::FlashMessage, paths};

pub async fn post_actions_sign_out(
    session: Session,
) -> Result<Response, crate::handlers::errors::HandlerError> {
    session.flush().await?;
    Ok(FlashMessage::info("You have been signed out.")
        .set_and_redirect(&session, paths::pages::ROOT)
        .await?)
}
