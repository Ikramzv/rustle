use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HttpError {
    status: u16,
    message: String,
}

impl HttpError {
    pub fn new(status: StatusCode, message: String) -> Self {
        Self {
            status: status.into(),
            message,
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        )
            .into_response()
    }
}
