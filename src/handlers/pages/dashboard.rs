use axum::{Extension, extract::State};
use sqlx::PgPool;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    constants::errors,
    data::{errors::DataError, queries},
    flash::FlashMessage,
    handlers::errors::HandlerError,
    views::pages,
};
use maud::Markup;

pub async fn get_dashboard(
    State(config): State<AppConfig>,
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let user_id = current_user.require_authenticated();

    let email = queries::user::get_user_email(&db, user_id)
        .await?
        .ok_or(DataError::NotFound(errors::ORDER_NOT_FOUND))?;

    let (stats, recent_orders) = tokio::try_join!(
        queries::order::get_order_stats_for_user(&db, user_id),
        queries::order::get_orders_for_user(&db, user_id, 5)
    )?;

    Ok(pages::dashboard::dashboard(
        &current_user,
        flash.as_ref(),
        config.site_name(),
        &email,
        stats,
        recent_orders,
    ))
}
