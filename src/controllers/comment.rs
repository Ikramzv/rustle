use axum::{
    Extension,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    app_state::SharedAppState,
    core::{error::http_error::HttpError, extractors::json::Json, layers::auth_layer::AuthUser},
    dtos::comment::{CreateCommentDto, UpdateCommentDto},
    service,
    types::PaginationQuery,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPostsCommentQuery {
    #[serde(flatten)]
    pagination: PaginationQuery,
    parent_id: Option<String>,
}

pub async fn get_posts_comments(
    State(app_state): State<SharedAppState>,
    Path(post_id): Path<String>,
    Query(mut query): Query<GetPostsCommentQuery>,
) -> Result<impl IntoResponse, HttpError> {
    if let Some(parent_id) = query.parent_id.as_ref() {
        if parent_id.is_empty() {
            query.parent_id = None
        }
    }

    let comments = service::comment::get_posts_comments(
        &app_state.db,
        &post_id,
        query.pagination.offset,
        query.pagination.limit,
        query.parent_id,
    )
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(comments))
}

pub async fn get_user_comments(
    State(app_state): State<SharedAppState>,
    Path(user_id): Path<String>,
    Query(query): Query<PaginationQuery>,
) -> Result<impl IntoResponse, HttpError> {
    let comments =
        service::comment::get_user_comments(&app_state.db, &user_id, query.offset, query.limit)
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(comments))
}

pub async fn create_comment(
    State(app_state): State<SharedAppState>,
    Extension(AuthUser(user_id)): Extension<AuthUser>,
    Json(payload): Json<CreateCommentDto>,
) -> Result<impl IntoResponse, HttpError> {
    let comment = service::comment::create_comment(&app_state.db, &user_id, payload)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(comment))
}

pub async fn update_comment(
    State(app_state): State<SharedAppState>,
    Extension(AuthUser(user_id)): Extension<AuthUser>,
    Path(comment_id): Path<String>,
    Json(payload): Json<UpdateCommentDto>,
) -> Result<impl IntoResponse, HttpError> {
    let comment = service::comment::update_comment(&app_state.db, &user_id, &comment_id, payload)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => HttpError::not_found("Comment not found".into()),
            _ => HttpError::server_error(e.to_string()),
        })?;

    Ok(Json(comment))
}

pub async fn delete_comment(
    State(app_state): State<SharedAppState>,
    Extension(AuthUser(user_id)): Extension<AuthUser>,
    Path(comment_id): Path<String>,
) -> Result<impl IntoResponse, HttpError> {
    service::comment::delete_comment(&app_state.db, &user_id, &comment_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => HttpError::not_found("Comment not found".into()),
            _ => HttpError::server_error(e.to_string()),
        })?;

    Ok((
        StatusCode::ACCEPTED,
        Json(json!({
            "success": true,
            "message": "Comment deleted successfully"
        })),
    ))
}
