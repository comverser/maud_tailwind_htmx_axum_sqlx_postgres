use axum::Extension;
use crate::{auth::CurrentUser, flash::FlashMessage, handlers::errors::HandlerError, views::pages};
use maud::Markup;

pub async fn get_root(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    Ok(pages::root::root(&current_user, &flash))
}
