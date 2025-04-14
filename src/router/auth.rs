use crate::{app_state::SharedAppState, controllers};
use axum::{Router, routing::post};

pub fn router() -> Router<SharedAppState> {
    let base = Router::new();

    base.nest(
        "/auth",
        Router::new()
            .route("/login", post(controllers::auth::login))
            .route("/verify", post(controllers::auth::verify_email)),
    )
}
