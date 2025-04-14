use axum::{
    Router,
    routing::{get, patch},
};

use crate::{app_state::SharedAppState, controllers};

pub fn router() -> Router<SharedAppState> {
    Router::new().nest(
        "/user",
        Router::new()
            .route("/whoami", get(controllers::user::whoami))
            .route("/update_profile", patch(controllers::user::update_profile)),
    )
}
