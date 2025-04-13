use axum::{
    Extension, Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, patch},
};
use validator::Validate;

use crate::{
    app_state::SharedAppState,
    ctx::layers::auth_layer::AuthUser,
    dtos::user::UpdateProfileDto,
    models::{error::HttpError, user::User},
    repositories::user::{get_user_by_username, update_user},
};

async fn get_me(
    State(app_state): State<SharedAppState>,
    Extension(AuthUser(user_id)): Extension<AuthUser>,
) -> Result<impl IntoResponse, HttpError> {
    let user: User = sqlx::query_as(r#"SELECT * FROM users WHERE id = $1"#)
        .bind(user_id)
        .fetch_one(&app_state.db)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(user))
}

async fn update_profile(
    State(app_state): State<SharedAppState>,
    Extension(AuthUser(user_id)): Extension<AuthUser>,
    Json(body): Json<UpdateProfileDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    if let Some(ref username) = body.username {
        let maybe_user: Option<User> = get_user_by_username(&app_state.db, username)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?;

        if maybe_user.is_some() {
            return Err(HttpError::bad_request(
                "Username already exists".to_string(),
            ));
        }
    }

    let user: User = update_user(&app_state.db, &user_id, body)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(user))
}

pub fn router() -> Router<SharedAppState> {
    Router::new().nest(
        "/user",
        Router::new()
            .route("/me", get(get_me))
            .route("/update_profile", patch(update_profile)),
    )
}
