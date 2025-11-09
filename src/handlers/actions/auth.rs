use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use sqlx::PgPool;
use tower_sessions::Session;

use crate::{
    auth::SESSION_USER_ID_KEY,
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
            FlashMessage::error("Invalid or expired magic link. Please request a new one.")
                .set(&session)
                .await?;
            return Ok(Redirect::to(paths::pages::SIGN_IN).into_response());
        }
    };

    // Get or create the user
    let user_id = commands::user::get_or_create_user(&db, &email).await?;

    // Create session
    session.insert(SESSION_USER_ID_KEY, user_id).await?;
    FlashMessage::success("Successfully signed in!")
        .set(&session)
        .await?;

    Ok(Redirect::to(paths::pages::ROOT).into_response())
}
