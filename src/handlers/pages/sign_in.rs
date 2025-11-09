use axum::Extension;
use crate::{auth::CurrentUser, flash::FlashMessage, handlers::errors::HandlerError, views::pages};
use maud::Markup;

pub async fn get_sign_in(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    Ok(pages::sign_in::sign_in(&current_user, &flash, None, None))
}
