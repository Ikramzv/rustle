mod auth;
mod comment;
mod post;
mod upload;
mod user;

use std::{any::Any, convert::Infallible, sync::Arc};

use axum::{
    Extension, Router,
    extract::{DefaultBodyLimit, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::{AllowOrigin, CorsLayer},
    services::ServeDir,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{
    app_state::SharedAppState,
    config::{CONFIG, Env},
    constants,
    core::{
        error::http_error::HttpError,
        layers::auth_layer,
        services::{mail::MailService, storage::StorageProvider},
    },
};

pub fn api_router() -> Router<SharedAppState> {
    let router = Router::new()
        .route("/", get(healt_check))
        .merge(auth::router())
        .merge(upload::router())
        .merge(user::router())
        .merge(post::router())
        .merge(comment::router())
        .layer(Extension(Arc::new(StorageProvider::new())))
        .layer(Extension(Arc::new(MailService::new())));

    let router = init_layers(router).nest_service(
        "/uploads",
        ServeDir::new(constants::DISK_STORAGE_PATH.to_string()),
    );

    router
}

fn init_layers(router: Router<SharedAppState>) -> Router<SharedAppState> {
    let cors = CorsLayer::new().allow_origin(match CONFIG.env {
        Env::DEV => AllowOrigin::any(),
        Env::RELEASE => AllowOrigin::from(vec![constants::WEBSITE_URL.parse().unwrap()]),
    });

    router
        .layer(
            ServiceBuilder::new()
                .layer(CatchPanicLayer::custom(handle_panic))
                .layer(TraceLayer::new_for_http())
                .layer(cors)
                .layer(TimeoutLayer::new(constants::REQUEST_TIMEOUT))
                .layer(auth_layer::AuthLayer::new().except(auth_layer::ExcludedPaths::new()))
                .layer(DefaultBodyLimit::max(CONFIG.request_body_limit)),
        )
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

fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response {
    let message = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Internal Server Error".to_string()
    };

    return HttpError::server_error(message).into_response();
}
