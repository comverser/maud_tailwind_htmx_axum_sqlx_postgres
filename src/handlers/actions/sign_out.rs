use tower_sessions::Session;

use crate::{constants::messages, flash::FlashMessage, handlers::errors::HandlerResult, paths};

pub async fn post_actions_sign_out(session: Session) -> HandlerResult {
    session.flush().await?;
    Ok(FlashMessage::info(messages::SIGNED_OUT)
        .set_and_redirect(&session, paths::pages::ROOT)
        .await?)
}
