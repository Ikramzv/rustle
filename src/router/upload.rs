use axum::{
    Router,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use axum_extra::{headers::ContentType, typed_header::TypedHeader};
use serde_json::json;

use crate::{
    app_state::SharedAppState, ctx::services::s3::upload_file_to_s3, models::error::HttpError,
};

async fn upload_file(
    State(app_state): State<SharedAppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, HttpError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| HttpError::bad_request(e.to_string()))?
    {
        let name = field.name().unwrap_or_default().to_string();
        let content_type = field.content_type().unwrap_or_default().to_string();
        let file_name = field.file_name();

        if name != "file" {
            continue;
        };

        if let Some(file_name) = file_name {
            let file_name = file_name.to_string();
            let data = field.bytes().await.unwrap_or_default();

            let url = upload_file_to_s3(&app_state.s3, data, file_name, content_type)
                .await
                .map_err(|e| HttpError::server_error(e.to_string()))?;

            return Ok((
                StatusCode::CREATED,
                TypedHeader(ContentType::json()),
                json!({
                    "url": url
                })
                .to_string(),
            )
                .into_response());
        }
    }

    Err(HttpError::bad_request("File field is required".to_string()))
}

pub fn router() -> Router<SharedAppState> {
    let router = Router::new();

    router.nest("/upload", Router::new().route("/", post(upload_file)))
}
