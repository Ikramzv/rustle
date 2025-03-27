mod auth;

use std::convert::Infallible;

use axum::{
    Router,
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{
    config::{CONFIG, Env},
    constants,
    ctx::layers::auth_layer,
    models::error::HttpError,
};

pub fn api_router() -> Router {
    let cors = CorsLayer::new().allow_origin(match CONFIG.env {
        Env::DEV => AllowOrigin::any(),
        Env::RELEASE => AllowOrigin::from(vec![constants::WEBSITE_URL.parse().unwrap()]),
    });

    let layers = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(TimeoutLayer::new(constants::REQUEST_TIMEOUT));

    Router::new()
        .route("/", get(healt_check))
        .merge(auth::router())
        .layer(layers)
        .layer(auth_layer::AuthLayer::new().except(auth_layer::ExcludedPaths::new()))
        .fallback(handle_404)
        .method_not_allowed_fallback(handle_405)
}

async fn healt_check() -> &'static str {
    "OK"
}

async fn handle_404(req: Request) -> Result<Response, Infallible> {
    tracing::error!("Unhandled request: {:?}", req);

    let error = HttpError::new(StatusCode::NOT_FOUND, "Not Found".to_string());

    let response = error.into_response();

    Ok(response)
}

async fn handle_405(req: Request) -> Result<Response, Infallible> {
    tracing::error!("Method not allowed: {:?}", req);

    let error = HttpError::new(
        StatusCode::METHOD_NOT_ALLOWED,
        "Method not allowed".to_string(),
    );

    let response = error.into_response();

    Ok(response)
}
