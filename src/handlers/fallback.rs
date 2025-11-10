use axum::{Extension, http::StatusCode};
use maud::Markup;

use crate::{auth::CurrentUser, flash::FlashMessage, views::pages};

pub async fn handle_404(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> (StatusCode, Markup) {
    (StatusCode::NOT_FOUND, pages::not_found::not_found(&current_user, flash.as_ref()))
}
