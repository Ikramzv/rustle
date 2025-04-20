use axum::{
    Extension,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use validator::Validate;

use crate::{
    app_state::SharedAppState,
    core::{error::http_error::HttpError, extractors::json::Json, layers::auth_layer::AuthUser},
    dtos::post::{CreatePostDto, CreatePostResponseDto, UpdatePostDto},
    service,
    types::PaginationQuery,
};

pub async fn create_post(
    Extension(AuthUser(user_id)): Extension<AuthUser>,
    State(app_state): State<SharedAppState>,
    Json(body): Json<CreatePostDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(HttpError::validation_error)?;

    let (post, post_media_list) = service::post::create_post(&app_state.db, &user_id, body)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(CreatePostResponseDto {
        post,
        media: post_media_list,
    }))
}

pub async fn find_posts(
    Query(query): Query<PaginationQuery>,
    State(app_state): State<SharedAppState>,
) -> Result<impl IntoResponse, HttpError> {
    let posts = service::post::find_posts(&app_state.db, query.offset, query.limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(posts))
}

pub async fn find_user_posts(
    Path(user_id): Path<String>,
    Query(query): Query<PaginationQuery>,
    State(app_state): State<SharedAppState>,
) -> Result<impl IntoResponse, HttpError> {
    let posts = service::post::find_user_posts(&app_state.db, &user_id, query.offset, query.limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(posts))
}

pub async fn find_post_by_id(
    Path(post_id): Path<String>,
    State(app_state): State<SharedAppState>,
) -> Result<impl IntoResponse, HttpError> {
    let post = service::post::find_post_by_id(&app_state.db, &post_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match post {
        Some(post) => Ok(Json(post)),
        None => Err(HttpError::not_found("Post not found".into())),
    }
}

pub async fn update_post(
    Path(post_id): Path<String>,
    Extension(AuthUser(user_id)): Extension<AuthUser>,
    State(app_state): State<SharedAppState>,
    Json(body): Json<UpdatePostDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(HttpError::validation_error)?;

    let post = service::post::update_post(&app_state.db, &user_id, &post_id, body)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match post {
        Some(post) => Ok(Json(post)),
        None => Err(HttpError::not_found("Post not found".into())),
    }
}

pub async fn delete_post(
    Path(post_id): Path<String>,
    Extension(AuthUser(user_id)): Extension<AuthUser>,
    State(app_state): State<SharedAppState>,
) -> Result<impl IntoResponse, HttpError> {
    let post = service::post::delete_post(&app_state.db, &user_id, &post_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    match post {
        Some(_) => Ok((
            StatusCode::ACCEPTED,
            Json(json!({
                "success": true,
                "message": "Post deleted successfully"
            })),
        )),
        None => Err(HttpError::not_found("Post not found".into())),
    }
}

pub async fn like_post(
    State(app_state): State<SharedAppState>,
    Path(post_id): Path<String>,
    Extension(AuthUser(user_id)): Extension<AuthUser>,
) -> Result<impl IntoResponse, HttpError> {
    let is_liked = service::post::like_post(&app_state.db, &user_id, &post_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => HttpError::not_found("Post not found".into()),
            _ => HttpError::server_error(e.to_string()),
        })?;

    Ok(Json(json!({
        "success": true,
        "liked": is_liked
    })))
}
