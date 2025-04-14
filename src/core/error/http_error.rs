use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
struct ValidationError {
    field: String,
    message: String,
}

#[derive(Debug, Serialize)]
pub struct HttpError {
    status: u16,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<ValidationError>>,
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
            errors: None,
        }
    }

    fn set_errors(mut self, errors: Vec<ValidationError>) -> Self {
        self.errors = Some(errors);
        self
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

    pub fn not_found(message: String) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn forbidden(message: String) -> Self {
        Self::new(StatusCode::FORBIDDEN, message)
    }

    pub fn conflict(message: String) -> Self {
        Self::new(StatusCode::CONFLICT, message)
    }

    pub fn too_many_requests(message: String) -> Self {
        Self::new(StatusCode::TOO_MANY_REQUESTS, message)
    }

    pub fn validation_error(errors: ValidationErrors) -> Self {
        let errors = errors
            .field_errors()
            .iter()
            .map(|(field, errors)| ValidationError {
                field: field.to_string(),
                message: errors
                    .iter()
                    .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                    .collect::<Vec<String>>()
                    .join(", "),
            })
            .collect::<Vec<ValidationError>>();

        Self::bad_request("Request body validation failed".to_string()).set_errors(errors)
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
