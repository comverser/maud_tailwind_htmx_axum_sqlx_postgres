use axum::{
    extract::{Query, State},
    response::Response,
};
use serde::Deserialize;
use sqlx::PgPool;
use tower_sessions::Session;

use crate::{
    auth::SESSION_USER_ID_KEY,
    constants::messages,
    data::commands,
    flash::FlashMessage,
    handlers::errors::HandlerError,
    paths,
};

#[derive(Deserialize)]
pub struct VerifyQuery {
    token: String,
}

/// Verify a magic link token and sign the user in
pub async fn get_actions_auth_verify(
    State(db): State<PgPool>,
    session: Session,
    Query(query): Query<VerifyQuery>,
) -> Result<Response, HandlerError> {
    // Verify and consume the magic link token
    let email = match commands::magic_link::verify_and_consume_magic_link(&db, &query.token).await
    {
        Ok(email) => email,
        Err(_) => {
            return Ok(FlashMessage::error(messages::MAGIC_LINK_INVALID)
                .set_and_redirect(&session, paths::pages::SIGN_IN)
                .await?);
        }
    };

    // Get or create the user
    let user_id = commands::user::get_or_create_user(&db, &email).await?;

    // Create session
    session.insert(SESSION_USER_ID_KEY, user_id).await?;

    Ok(FlashMessage::success(messages::SIGNED_IN)
        .set_and_redirect(&session, paths::pages::ROOT)
        .await?)
}
