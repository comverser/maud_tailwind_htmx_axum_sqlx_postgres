use axum::{Extension, extract::{Path, State}};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{auth::CurrentUser, config::AppConfig, constants::errors, data::{errors::DataError, queries}, flash::FlashMessage, handlers::errors::HandlerError, models::order::PaymentStatus, views::pages};
use maud::Markup;

pub async fn get_result(
    State(config): State<AppConfig>,
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
    Path(order_id): Path<Uuid>,
) -> Result<Markup, HandlerError> {
    let user_id = current_user.require_authenticated();

    let order = queries::order::get_order(&db, order_id)
        .await?
        .ok_or(DataError::NotFound(errors::ORDER_NOT_FOUND))?;

    order.verify_ownership(user_id)?;

    if !matches!(order.payment_status, PaymentStatus::Paid) {
        return Err(DataError::Unauthorized(errors::PAYMENT_NOT_COMPLETED).into());
    }

    Ok(pages::result::result(&current_user, flash.as_ref(), config.site_name(), &order))
}
