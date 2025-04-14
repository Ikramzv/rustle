use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

pub type SharedAppState = Arc<AppState>;

// impl FromRef<AppState> for PgPool {
//     fn from_ref(state: &AppState) -> Self {
//         state.db.clone()
//     }
// }
