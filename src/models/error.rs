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

impl std::error::Error for HttpError {}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HttpError: status={}, message={}",
            self.status, self.message
        )
    }
}

impl HttpError {
    pub fn new(status: StatusCode, message: String) -> Self {
        Self {
            status: status.into(),
            message,
        }
    }

    pub fn bad_request(message: String) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn server_error(message: String) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn unauthorized(message: String) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
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

impl From<anyhow::Error> for HttpError {
    fn from(value: anyhow::Error) -> Self {
        Self::server_error(value.to_string())
    }
}
