use sqlx::PgPool;
use uuid::Uuid;

use crate::{constants::errors, data::errors::DataError, models::order::{Order, OrderStats, OrderSummary}};

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

pub async fn get_orders_for_user(
    db: &PgPool,
    user_id: i32,
    limit: i64,
) -> Result<Vec<OrderSummary>, DataError> {
    let orders = sqlx::query_as!(
        OrderSummary,
        r#"
        SELECT
            order_id,
            filename,
            file_size,
            text_length,
            price_amount,
            payment_status as "payment_status: _",
            order_number,
            created_at
        FROM orders
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
        user_id,
        limit
    )
    .fetch_all(db)
    .await?;

    Ok(orders)
}

pub async fn get_order_stats_for_user(
    db: &PgPool,
    user_id: i32,
) -> Result<OrderStats, DataError> {
    let result = sqlx::query!(
        r#"
        SELECT
            COUNT(*) as "total_orders!",
            COALESCE(SUM(CASE WHEN payment_status = 'paid' THEN price_amount ELSE 0 END), 0) as "total_spent!",
            COUNT(CASE WHEN payment_status = 'paid' THEN 1 END) as "paid_orders_count!"
        FROM orders
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(OrderStats {
        total_orders: result.total_orders,
        total_spent: result.total_spent,
        paid_orders_count: result.paid_orders_count,
    })
}

pub async fn get_order_for_user(
    db: &PgPool,
    order_id: Uuid,
    user_id: i32,
) -> Result<Order, DataError> {
    let order = get_order(db, order_id)
        .await?
        .ok_or(DataError::NotFound(errors::ORDER_NOT_FOUND))?;
    order.verify_ownership(user_id)?;
    Ok(order)
}

pub async fn get_order_by_order_number_for_user(
    db: &PgPool,
    order_number: &str,
    user_id: i32,
) -> Result<Order, DataError> {
    let order = get_order_by_order_number(db, order_number)
        .await?
        .ok_or(DataError::NotFound(errors::ORDER_NOT_FOUND))?;
    order.verify_ownership(user_id)?;
    Ok(order)
}
