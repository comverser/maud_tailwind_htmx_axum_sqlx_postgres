use sqlx::PgPool;
use uuid::Uuid;

use crate::{data::errors::DataError, models::order::{Order, PaymentStatus}};

pub async fn create_order(
    db: &PgPool,
    user_id: i32,
    filename: &str,
    file_size: i32,
    text_content: &str,
    text_length: i32,
    price_amount: i32,
    order_number: &str,
) -> Result<Order, DataError> {
    let order = sqlx::query_as!(
        Order,
        r#"
        INSERT INTO orders(user_id, filename, file_size, text_content, text_length, price_amount, payment_status, order_number)
        VALUES($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING
            order_id,
            user_id,
            filename,
            file_size,
            text_content,
            text_length,
            price_amount,
            payment_status as "payment_status: PaymentStatus",
            payment_key,
            order_number,
            created_at,
            paid_at
        "#,
        user_id,
        filename,
        file_size,
        text_content,
        text_length,
        price_amount,
        PaymentStatus::Pending as PaymentStatus,
        order_number
    )
    .fetch_one(db)
    .await?;

    Ok(order)
}

pub async fn update_order_payment(
    db: &PgPool,
    order_id: Uuid,
    payment_key: &str,
    payment_status: PaymentStatus,
) -> Result<Order, DataError> {
    let order = sqlx::query_as!(
        Order,
        r#"
        UPDATE orders
        SET payment_key = $2, payment_status = $3, paid_at = CASE WHEN $3 = 'paid' THEN NOW() ELSE paid_at END
        WHERE order_id = $1
        RETURNING
            order_id,
            user_id,
            filename,
            file_size,
            text_content,
            text_length,
            price_amount,
            payment_status as "payment_status: PaymentStatus",
            payment_key,
            order_number,
            created_at,
            paid_at
        "#,
        order_id,
        payment_key,
        payment_status as PaymentStatus
    )
    .fetch_one(db)
    .await?;

    Ok(order)
}
