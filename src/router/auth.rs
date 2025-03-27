use axum::{Router, routing::post};

async fn signup() {}

async fn login() {}

async fn verify_email() {}

pub fn router() -> Router {
    let base = Router::new();

    base.nest(
        "/auth",
        Router::new()
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify", post(verify_email)),
    )
}
