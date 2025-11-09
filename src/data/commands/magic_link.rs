use crate::data::errors::DataError;
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

const MAGIC_LINK_EXPIRY_MINUTES: i64 = 15;

/// Create or update a magic link token for an email address
pub async fn create_magic_link(
    db: &PgPool,
    email: &str,
    token: &str,
) -> Result<(), DataError> {
    let expires_at = OffsetDateTime::now_utc() + Duration::minutes(MAGIC_LINK_EXPIRY_MINUTES);

    // Delete any existing tokens for this email first
    sqlx::query!("DELETE FROM magic_links WHERE email = $1", email)
        .execute(db)
        .await
        .map_err(DataError::Database)?;

    // Insert the new token
    sqlx::query!(
        "INSERT INTO magic_links(token, email, expires_at) VALUES($1, $2, $3)",
        token,
        email,
        expires_at
    )
    .execute(db)
    .await
    .map_err(DataError::Database)?;

    Ok(())
}

/// Verify a magic link token and return the associated email if valid
/// Deletes the token after verification (one-time use)
pub async fn verify_and_consume_magic_link(
    db: &PgPool,
    token: &str,
) -> Result<String, DataError> {
    let now = OffsetDateTime::now_utc();

    // Find and delete the token in one query
    let row = sqlx::query!(
        "DELETE FROM magic_links
         WHERE token = $1 AND expires_at > $2
         RETURNING email",
        token,
        now
    )
    .fetch_one(db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => DataError::Unauthorized("Invalid or expired magic link"),
        _ => DataError::Database(e),
    })?;

    Ok(row.email)
}

/// Clean up expired magic links (should be called periodically)
pub async fn cleanup_expired_magic_links(db: &PgPool) -> Result<u64, DataError> {
    let now = OffsetDateTime::now_utc();

    let result = sqlx::query!("DELETE FROM magic_links WHERE expires_at <= $1", now)
        .execute(db)
        .await
        .map_err(DataError::Database)?;

    Ok(result.rows_affected())
}
