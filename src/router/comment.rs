use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{app_state::SharedAppState, controllers};

pub fn router() -> Router<SharedAppState> {
    Router::new().nest(
        "/comments",
        Router::new()
            .route(
                "/post/{post_id}",
                get(controllers::comment::get_posts_comments),
            )
            .route(
                "/user/{user_id}",
                get(controllers::comment::get_user_comments),
            )
            .route("/", post(controllers::comment::create_comment))
            .route("/{comment_id}", patch(controllers::comment::update_comment))
            .route(
                "/{comment_id}",
                delete(controllers::comment::delete_comment),
            ),
    )
}
