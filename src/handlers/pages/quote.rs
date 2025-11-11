use axum::{Extension, extract::{Path, State}};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{auth::CurrentUser, config::AppConfig, data::{errors::DataError, queries}, flash::FlashMessage, handlers::errors::HandlerError, views::pages};
use maud::Markup;

pub async fn get_quote(
    State(config): State<AppConfig>,
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
    Path(order_id): Path<Uuid>,
) -> Result<Markup, HandlerError> {
    let user_id = current_user.require_authenticated();

    let order = queries::order::get_order(&db, order_id)
        .await?
        .ok_or(DataError::NotFound("Order not found"))?;

    if order.user_id != user_id {
        return Err(DataError::Unauthorized("Not your order").into());
    }

    Ok(pages::quote::quote(&current_user, flash.as_ref(), config.site_name(), &order))
}
