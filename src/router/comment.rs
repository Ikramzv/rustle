use axum::{Router, routing::post};

use crate::app_state::SharedAppState;

async fn create_comment() {}

pub fn router() -> Router<SharedAppState> {
    Router::new().nest("/comment", Router::new().route("/", post(create_comment)))
}
