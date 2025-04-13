use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use chrono::Utc;
use sqlx::PgPool;
use validator::Validate;

use crate::{
    app_state::SharedAppState,
    config::CONFIG,
    constants::VERIFICATION_PIN_EXPIRATION_TIME,
    ctx::{
        services::mail::send_verification_mail,
        utils::{jwt, pin::generate_pin},
    },
    dtos::auth::{LoginUserDto, VerifyEmailDto, VerifyResponseDto},
    models::{
        error::HttpError,
        user::{User, VerificationPin},
    },
};

async fn verify_email(
    State(app_state): State<SharedAppState>,
    Json(body): Json<VerifyEmailDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let verification_pin: Option<VerificationPin> =
        sqlx::query_as(r#"SELECT * FROM verification_pins WHERE email = $1 AND pin = $2"#)
            .bind(body.email.clone())
            .bind(body.pin)
            .fetch_optional(&app_state.db)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?;

    if verification_pin.is_none() {
        return Err(HttpError::bad_request(
            "Invalid verification pin".to_owned(),
        ));
    }

    let pin = verification_pin.unwrap();

    if pin.expires_at < Utc::now() {
        return Err(HttpError::bad_request(
            "Verification pin expired".to_owned(),
        ));
    }

    let user: User =
        sqlx::query_as("UPDATE users SET is_verified = true WHERE email = $1 RETURNING *")
            .bind(body.email)
            .fetch_one(&app_state.db)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?;

    let token = jwt::generate_token(&user.id, &CONFIG.jwt_secret)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(VerifyResponseDto { token, user }))
}

async fn login(
    State(app_state): State<SharedAppState>,
    Json(body): Json<LoginUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let pin = generate_pin();

    let expires_at = Utc::now() + VERIFICATION_PIN_EXPIRATION_TIME;

    let (verification_pin, user) = tokio::try_join!(
        create_verification_pin(&app_state.db, body.email.clone(), pin, expires_at),
        create_user_if_not_exists(&app_state.db, body.clone())
    )?;

    send_verification_mail(&app_state.smtp, body.email, verification_pin.pin)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(user))
}

async fn create_verification_pin(
    db: &PgPool,
    email: String,
    pin: String,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> Result<VerificationPin, HttpError> {
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
    .fetch_one(db)
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(verification_pin)
}

async fn create_user_if_not_exists(db: &PgPool, body: LoginUserDto) -> Result<User, HttpError> {
    let user: Option<User> = sqlx::query_as(r#"SELECT * FROM users WHERE email = $1"#)
        .bind(body.email.clone())
        .fetch_optional(db)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    if let Some(user) = user {
        return Ok(user);
    }

    let user: User = sqlx::query_as(
        r#"INSERT INTO users 
            (email, username, profile_image_url)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
    )
    .bind(body.email)
    .bind(body.username)
    .bind(body.profile_image_url)
    .fetch_one(db)
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(user)
}

pub fn router() -> Router<SharedAppState> {
    let base = Router::new();

    base.nest(
        "/auth",
        Router::new()
            .route("/login", post(login))
            .route("/verify", post(verify_email)),
    )
}
