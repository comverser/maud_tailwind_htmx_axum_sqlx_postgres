use axum::{Extension, extract::State};
use crate::{auth::CurrentUser, data::queries, flash::FlashMessage, handlers::errors::HandlerError, views::pages};
use maud::Markup;
use sqlx::PgPool;

pub async fn get_todos(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let user_id = current_user.require_authenticated();

    let todos = queries::todo::get_todos_for_user(&db, user_id).await?;

    Ok(pages::todos::todos(&current_user, flash.as_ref(), todos, None, None))
}
