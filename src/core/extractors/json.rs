use axum::{
    extract::{FromRequest, rejection::JsonRejection},
    response::IntoResponse,
};
use serde::{Serialize, de::DeserializeOwned};

use crate::core::error::http_error::HttpError;

pub struct Json<T>(pub T);

impl<S, T> FromRequest<S> for Json<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = HttpError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::from_request(req, state).await {
            Ok(json) => Ok(Json(json.0)),
            Err(err) => {
                tracing::error!("Error parsing json: {:?}", err);
                let message = match err {
                    JsonRejection::JsonDataError(e) => e.to_string(),
                    JsonRejection::JsonSyntaxError(e) => e.to_string(),
                    JsonRejection::MissingJsonContentType(e) => e.to_string(),
                    JsonRejection::BytesRejection(err) => err.to_string(),
                    _ => "Invalid json".to_string(),
                };

                Err(HttpError::bad_request(message))
            }
        }
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}
