use crate::{app_state::SharedAppState, controllers};
use axum::{Router, routing::post};

pub fn router() -> Router<SharedAppState> {
    let router = Router::new();

    router.nest(
        "/upload",
        Router::new().route("/", post(controllers::upload::upload_file)),
    )
}
