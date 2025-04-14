use axum::extract::{Extension, State};
use axum::response::{IntoResponse, Json};
use chrono::Utc;
use validator::Validate;

use crate::core::error::http_error::HttpError;
use crate::dtos::auth::VerifyEmailDto;
use crate::extensions::MailServiceExt;
use crate::service::user::{create_user_if_not_exists, get_user_by_email};
use crate::service::verification_pin::{create_verification_pin, get_verification_pin};
use crate::{
    app_state::SharedAppState,
    config::CONFIG,
    constants::VERIFICATION_PIN_EXPIRATION_TIME,
    core::utils::{jwt, pin::generate_pin},
    dtos::auth::{LoginUserDto, VerifyResponseDto},
};

pub async fn verify_email(
    State(app_state): State<SharedAppState>,
    Json(body): Json<VerifyEmailDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(HttpError::validation_error)?;

    let verification_pin = match get_verification_pin(&app_state.db, &body.email, &body.pin)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
    {
        Some(pin) => pin,
        None => {
            return Err(HttpError::bad_request(
                "Invalid verification pin".to_owned(),
            ));
        }
    };

    if verification_pin.expires_at < Utc::now() {
        return Err(HttpError::bad_request(
            "Verification pin expired".to_owned(),
        ));
    }

    let user = match get_user_by_email(&app_state.db, &body.email)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
    {
        Some(user) => user,
        None => return Err(HttpError::bad_request("User not found".into())),
    };

    let token = jwt::generate_token(&user.id, &CONFIG.jwt_secret)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(VerifyResponseDto { token, user }))
}

#[tracing::instrument(
    name = "Login",
    skip(app_state, mail_service, body),
    fields(
        email = %body.email,
    ),
    err
)]
pub async fn login(
    State(app_state): State<SharedAppState>,
    Extension(mail_service): MailServiceExt,
    Json(body): Json<LoginUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(HttpError::validation_error)?;

    let pin = generate_pin();

    let expires_at = Utc::now() + VERIFICATION_PIN_EXPIRATION_TIME;

    let (verification_pin, user) = tokio::try_join!(
        create_verification_pin(&app_state.db, body.email.clone(), pin, expires_at),
        create_user_if_not_exists(
            &app_state.db,
            &body.email,
            &body.username,
            body.profile_image_url
        )
    )
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    mail_service
        .send_verification_mail(body.email, verification_pin.pin)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(user))
}
