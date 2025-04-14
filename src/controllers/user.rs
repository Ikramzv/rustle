use axum::{Extension, Json, extract::State, response::IntoResponse};
use validator::Validate;

use crate::{
    app_state::SharedAppState,
    core::error::http_error::HttpError,
    core::layers::auth_layer::AuthUser,
    dtos::user::UpdateProfileDto,
    service::user::{get_user_by_id, get_user_by_username, update_user},
};

pub async fn whoami(
    State(app_state): State<SharedAppState>,
    Extension(AuthUser(user_id)): Extension<AuthUser>,
) -> Result<impl IntoResponse, HttpError> {
    let user = get_user_by_id(&app_state.db, &user_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(HttpError::not_found("User not found".into())),
    }
}

pub async fn update_profile(
    State(app_state): State<SharedAppState>,
    Extension(AuthUser(user_id)): Extension<AuthUser>,
    Json(body): Json<UpdateProfileDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(HttpError::validation_error)?;

    if let Some(ref username) = body.username {
        let maybe_user = get_user_by_username(&app_state.db, username)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?;

        match maybe_user {
            Some(_) => {
                return Err(HttpError::conflict("Username already exists".to_string()));
            }
            None => (),
        }
    }

    let user = update_user(&app_state.db, &user_id, body)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(user))
}
