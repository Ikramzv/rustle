use sqlx::{PgPool, Result};

use crate::models::VerificationPin;

pub async fn get_verification_pin(
    pool: &PgPool,
    email: &str,
    pin: &str,
) -> Result<Option<VerificationPin>> {
    sqlx::query_as(r#"SELECT * FROM verification_pins WHERE email = $1 AND pin = $2"#)
        .bind(email)
        .bind(pin)
        .fetch_optional(pool)
        .await
}

pub async fn create_verification_pin(
    pool: &PgPool,
    email: String,
    pin: String,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> Result<VerificationPin> {
    let verification_pin: VerificationPin = sqlx::query_as(
        r#"INSERT INTO verification_pins
            (email, pin, expires_at)    
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
    )
    .bind(email)
    .bind(pin)
    .bind(expires_at)
    .fetch_one(pool)
    .await?;

    Ok(verification_pin)
}
