use std::{pin::Pin, str::FromStr};

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderName, response::Parts},
    response::{IntoResponse, Response},
};
use tower::{Layer, Service};

use crate::models::error::HttpError;

#[derive(Clone)]
pub struct GlobalErrorLayer;

impl GlobalErrorLayer {
    pub fn new() -> Self {
        GlobalErrorLayer
    }
}

impl<S> Layer<S> for GlobalErrorLayer {
    type Service = GlobalErrorService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        GlobalErrorService(inner)
    }
}

#[derive(Clone)]
pub struct GlobalErrorService<S>(pub S);

impl<S> Service<Request> for GlobalErrorService<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Error: std::fmt::Debug + std::fmt::Display + Send + Sync + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let fut = self.0.call(req);

        Box::pin(async move {
            println!("Called GlobalErrorService");

            let res = fut.await;

            match res {
                Ok(res) => Ok(handle_response(res).await),
                Err(e) => {
                    tracing::error!("Error in GlobalErrorService: {:?}", e);
                    Ok(HttpError::server_error(e.to_string()).into_response())
                }
            }
        })
    }
}

async fn handle_response(res: Response) -> Response {
    if res.status().is_client_error() || res.status().is_server_error() {
        let (parts, body) = res.into_parts();

        let bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .unwrap_or_default();

        match serde_json::from_slice::<serde_json::Value>(&bytes) {
            // If JSON, print the error message
            Ok(json) => return handle_json_error(parts, json),
            // If not JSON, try to read as text
            Err(_) => return handle_text_error(parts, &bytes),
        }
    }

    res
}

fn handle_json_error(parts: Parts, json: serde_json::Value) -> Response {
    let default_message = serde_json::Value::String("Unknown server error".to_string());

    let message = json.get("message").unwrap_or(&default_message);

    let http_error = HttpError::new(parts.status, message.to_string());

    return http_error.into_response();
}

fn handle_text_error(parts: Parts, bytes: &[u8]) -> Response {
    let text = String::from_utf8_lossy(&bytes);
    tracing::error!("(GlobalErrorService): {}", text);

    let http_error = HttpError::new(parts.status, text.to_string());
    let response = http_error.into_response();

    return response;
}
