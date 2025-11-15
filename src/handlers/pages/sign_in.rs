use axum::{Extension, extract::State};
use crate::{auth::CurrentUser, config::AppConfig, flash::FlashMessage, handlers::errors::HandlerError, views::pages};
use maud::Markup;

pub async fn get_sign_in(
    State(config): State<AppConfig>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    Ok(pages::sign_in(&current_user, flash.as_ref(), config.site_name(), None, None))
}
