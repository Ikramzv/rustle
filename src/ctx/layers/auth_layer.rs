use std::{
    collections::HashSet,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Method, Request, Response},
    response::IntoResponse,
};
use tower::{Layer, Service};

use crate::{config::CONFIG, ctx::utils::jwt, models::error::HttpError};

#[derive(Debug, Clone, Default)]
pub struct ExcludedPaths {
    post: HashSet<&'static str>,
    get: HashSet<&'static str>,
    put: HashSet<&'static str>,
    patch: HashSet<&'static str>,
    delete: HashSet<&'static str>,
}

impl ExcludedPaths {
    pub fn new() -> Self {
        Self {
            post: HashSet::from_iter(["/auth/login", "/auth/verify", "/upload"]),
            get: HashSet::from_iter([]),
            put: HashSet::from_iter([]),
            patch: HashSet::from_iter([]),
            delete: HashSet::from_iter([]),
        }
    }
}

#[derive(Clone)]
pub struct AuthUser(pub String);

#[derive(Debug, Clone)]
pub struct AuthLayer {
    excluded_paths: ExcludedPaths,
}

impl AuthLayer {
    pub fn new() -> Self {
        Self {
            excluded_paths: ExcludedPaths::default(),
        }
    }

    pub fn except(mut self, excluded_paths: ExcludedPaths) -> Self {
        self.excluded_paths = excluded_paths;
        self
    }
}

#[derive(Debug, Clone)]
pub struct AuthLayerService<S> {
    inner: S,
    excluded_paths: ExcludedPaths,
}

impl<S, ReqBody> Service<Request<ReqBody>> for AuthLayerService<S>
where
    S: Service<Request<ReqBody>, Response = Response<Body>> + Clone + Send + Sync + 'static,
    S::Future: Send,
    ReqBody: Send + 'static,
{
    type Error = S::Error;
    type Response = S::Response;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let clone = self.inner.clone();

        let mut inner = std::mem::replace(&mut self.inner, clone);

        let method = req.method();
        let path = req.uri().path();

        let excluded = match method {
            &Method::GET => self.excluded_paths.get.contains(path),
            &Method::POST => self.excluded_paths.post.contains(path),
            &Method::PUT => self.excluded_paths.put.contains(path),
            &Method::PATCH => self.excluded_paths.patch.contains(path),
            &Method::DELETE => self.excluded_paths.delete.contains(path),
            _ => false,
        };

        Box::pin(async move {
            if excluded {
                return inner.call(req).await;
            }

            let headers = req.headers().clone();

            let user_id = authorize_user(headers).await;

            if let Err(e) = user_id {
                return Ok(HttpError::unauthorized(e).into_response());
            }

            req.extensions_mut()
                .insert::<AuthUser>(AuthUser(user_id.unwrap()));

            let fut = inner.call(req).await;

            println!("fut: {:?}", fut.is_ok());

            fut
        })
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthLayerService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthLayerService {
            inner,
            excluded_paths: self.excluded_paths.clone(),
        }
    }
}

async fn authorize_user(headers: HeaderMap<HeaderValue>) -> Result<String, String> {
    let token = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|value| value.split(" ").nth(1));

    if token.is_none() {
        return Err("Unauthorized".to_string());
    }

    let token = token.unwrap();

    let user_id = jwt::validate_token(token, &CONFIG.jwt_secret).await?;

    Ok(user_id)
}
