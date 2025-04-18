use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{app_state::SharedAppState, controllers};

pub fn router() -> Router<SharedAppState> {
    Router::new().nest(
        "/posts",
        Router::new()
            .route("/", post(controllers::post::create_post))
            .route("/", get(controllers::post::find_posts))
            .route("/user/{user_id}", get(controllers::post::find_user_posts))
            .route("/{post_id}", get(controllers::post::find_post_by_id))
            .route("/{post_id}", patch(controllers::post::update_post))
            .route("/{post_id}", delete(controllers::post::delete_post)),
    )
}
