use sqlx::PgPool;
use uuid::Uuid;

use crate::{data::errors::DataError, models::order::Order};

pub async fn get_order(db: &PgPool, order_id: Uuid) -> Result<Option<Order>, DataError> {
    let order = sqlx::query_as!(
        Order,
        r#"
        SELECT
            order_id,
            user_id,
            filename,
            file_size,
            text_content,
            text_length,
            price_amount,
            payment_status as "payment_status: _",
            payment_key,
            order_number,
            created_at,
            paid_at
        FROM orders
        WHERE order_id = $1
        "#,
        order_id
    )
    .fetch_optional(db)
    .await?;

    Ok(order)
}

pub async fn get_order_by_order_number(db: &PgPool, order_number: &str) -> Result<Option<Order>, DataError> {
    let order = sqlx::query_as!(
        Order,
        r#"
        SELECT
            order_id,
            user_id,
            filename,
            file_size,
            text_content,
            text_length,
            price_amount,
            payment_status as "payment_status: _",
            payment_key,
            order_number,
            created_at,
            paid_at
        FROM orders
        WHERE order_number = $1
        "#,
        order_number
    )
    .fetch_optional(db)
    .await?;

    Ok(order)
}
