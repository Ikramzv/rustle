mod auth;
mod comment;
mod post;
mod upload;
mod user;

use std::convert::Infallible;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Request},
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
    app_state::SharedAppState,
    config::{CONFIG, Env},
    constants,
    ctx::layers::{auth_layer, error_layer::GlobalErrorLayer},
    models::error::HttpError,
};

pub fn api_router() -> Router<SharedAppState> {
    let cors = CorsLayer::new().allow_origin(match CONFIG.env {
        Env::DEV => AllowOrigin::any(),
        Env::RELEASE => AllowOrigin::from(vec![constants::WEBSITE_URL.parse().unwrap()]),
    });

    let layers = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(TimeoutLayer::new(constants::REQUEST_TIMEOUT))
        .layer(auth_layer::AuthLayer::new().except(auth_layer::ExcludedPaths::new()))
        .layer(DefaultBodyLimit::max(CONFIG.request_body_limit));

    Router::new()
        .route("/", get(healt_check))
        .merge(auth::router())
        .merge(upload::router())
        .merge(user::router())
        .merge(post::router())
        .merge(comment::router())
        .layer(layers)
        .layer(GlobalErrorLayer::new())
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
// async fn handle_error(error: BoxError) -> impl IntoResponse {
//     if error.is::<HttpError>() {
//         return error.downcast::<HttpError>().unwrap().into_response();
//     }

//     tracing::error!("Unhandled error: {:?}", error);

//     let error = HttpError::server_error("Internal server error".to_owned());

//     error.into_response()
// }
