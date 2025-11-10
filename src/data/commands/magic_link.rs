use crate::constants::auth::MAGIC_LINK_EXPIRY_MINUTES;
use crate::data::{errors::DataError, map_row_unauthorized};
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

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
    .map_err(|e| map_row_unauthorized(e, "Invalid or expired magic link"))?;

    Ok(row.email)
}
