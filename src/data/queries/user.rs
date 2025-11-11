use sqlx::PgPool;
use crate::data::errors::DataError;

pub async fn get_user_email(db: &PgPool, user_id: i32) -> Result<Option<String>, DataError> {
    let result = sqlx::query!("SELECT email FROM users WHERE user_id = $1", user_id)
        .fetch_optional(db)
        .await?;

    Ok(result.map(|row| row.email))
}
