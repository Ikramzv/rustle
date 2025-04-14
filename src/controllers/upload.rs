use axum::{Extension, extract::Multipart, http::StatusCode, response::IntoResponse};
use axum_extra::{TypedHeader, headers::ContentType};
use serde_json::json;

use crate::{core::error::http_error::HttpError, extensions::S3ServiceExt};

pub async fn upload_file(
    Extension(s3_service): S3ServiceExt,
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

            let url = s3_service
                .upload_file(data, file_name, content_type)
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
