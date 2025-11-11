use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{constants::errors, data::errors::DataError};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,
    Paid,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: Uuid,
    pub user_id: i32,
    pub filename: String,
    pub file_size: i32,
    pub text_content: String,
    pub text_length: i32,
    pub price_amount: i32,
    pub payment_status: PaymentStatus,
    pub payment_key: Option<String>,
    pub order_number: String,
    pub created_at: OffsetDateTime,
    pub paid_at: Option<OffsetDateTime>,
}

impl Order {
    pub fn verify_ownership(&self, user_id: i32) -> Result<(), DataError> {
        if self.user_id != user_id {
            Err(DataError::Unauthorized(errors::NOT_YOUR_ORDER))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSummary {
    pub order_id: Uuid,
    pub filename: String,
    pub file_size: i32,
    pub text_length: i32,
    pub price_amount: i32,
    pub payment_status: PaymentStatus,
    pub order_number: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug)]
pub struct OrderStats {
    pub total_orders: i64,
    pub total_spent: i64,
    pub paid_orders_count: i64,
}
