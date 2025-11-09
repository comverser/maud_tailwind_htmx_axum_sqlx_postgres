use axum::{Extension, Form, extract::State, http::StatusCode, response::{IntoResponse, Response}};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    auth::CurrentUser,
    data::commands,
    email::{self, EmailConfig},
    flash::FlashMessage,
    magic_link,
    models::user::{FIELD_EMAIL, MagicLinkRequestForm},
    paths,
    views::pages::sign_in,
};
use tower_sessions::Session;

use super::parse_validation_errors;

/// Handle magic link request - sends an email with a sign-in link
pub async fn post_forms_sign_in(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    Form(form): Form<MagicLinkRequestForm>,
) -> Result<Response, crate::handlers::errors::HandlerError> {
    if let Err(validation_errors) = form.validate() {
        return Ok(render_validation_errors(&current_user, &form, &validation_errors));
    }

    // Generate magic link token
    let token = magic_link::generate_token();

    // Store the magic link in database
    commands::magic_link::create_magic_link(&db, &form.email, &token).await?;

    // Send the magic link email
    let email_config = EmailConfig::from_env();
    if let Err(e) = email::send_magic_link(&email_config, &form.email, &token).await {
        tracing::error!("Failed to send magic link email: {}", e);
        return Ok(FlashMessage::error("Failed to send email. Please try again.")
            .set_and_redirect(&session, paths::pages::SIGN_IN)
            .await?);
    }

    // Show success message
    Ok(FlashMessage::success("Check your email! We sent you a link to sign in.")
        .set_and_redirect(&session, paths::pages::SIGN_IN)
        .await?)
}

fn render_validation_errors(
    current_user: &CurrentUser,
    form: &MagicLinkRequestForm,
    validation_errors: &validator::ValidationErrors,
) -> Response {
    let errors = parse_validation_errors(validation_errors);
    (
        StatusCode::BAD_REQUEST,
        sign_in::sign_in(
            current_user,
            &None,
            Some(&form.email),
            errors.get(FIELD_EMAIL).map(String::as_str),
        ),
    )
        .into_response()
}
