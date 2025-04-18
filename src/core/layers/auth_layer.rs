use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Method, Request, Response},
    response::IntoResponse,
};
use tower::{Layer, Service};

use crate::core::error::http_error::HttpError;
use crate::{config::CONFIG, core::utils::jwt};

#[derive(Debug, Clone, Default)]
pub struct ExcludedPaths {
    post: matchit::Router<()>,
    get: matchit::Router<()>,
    put: matchit::Router<()>,
    patch: matchit::Router<()>,
    delete: matchit::Router<()>,
}

impl ExcludedPaths {
    pub fn new() -> Self {
        Self {
            post: Self::from(&["/auth/login", "/auth/verify", "/upload"]),
            get: Self::from(&["/posts", "/posts/{post_id}", "/posts/user/{user_id}"]),
            put: Self::from(&[]),
            patch: Self::from(&[]),
            delete: Self::from(&[]),
        }
    }

    fn from(paths: &[&str]) -> matchit::Router<()> {
        let mut router = matchit::Router::new();

        paths.iter().for_each(|value| {
            router.insert(value.to_string(), ()).unwrap();
        });

        router
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
            &Method::GET => self.excluded_paths.get.at(path).is_ok(),
            &Method::POST => self.excluded_paths.post.at(path).is_ok(),
            &Method::PUT => self.excluded_paths.put.at(path).is_ok(),
            &Method::PATCH => self.excluded_paths.patch.at(path).is_ok(),
            &Method::DELETE => self.excluded_paths.delete.at(path).is_ok(),
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
