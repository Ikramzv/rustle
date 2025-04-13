use axum::{
    Router,
    routing::{get, post},
};

use crate::app_state::SharedAppState;

async fn create_post() {}

async fn get_post_detail() {}

async fn like_post() {}

pub fn router() -> Router<SharedAppState> {
    Router::new().nest(
        "/post",
        Router::new()
            .route("/", post(create_post))
            .route("/{post_id}/like", post(like_post))
            .route("/{post_id}", get(get_post_detail)),
    )
}
