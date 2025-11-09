use crate::data::errors::DataError;
use sqlx::PgPool;

/// Get or create a user by email address
/// Returns the user_id
pub async fn get_or_create_user(db: &PgPool, email: &str) -> Result<i32, DataError> {
    // Try to find existing user
    let existing = sqlx::query!("SELECT user_id FROM users WHERE email = $1", email)
        .fetch_optional(db)
        .await
        .map_err(DataError::Database)?;

    if let Some(row) = existing {
        return Ok(row.user_id);
    }

    // Create new user if not exists
    let row = sqlx::query!(
        "INSERT INTO users(email) VALUES($1) RETURNING user_id",
        email
    )
    .fetch_one(db)
    .await
    .map_err(DataError::Database)?;

    Ok(row.user_id)
}
