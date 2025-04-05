use std::sync::Arc;

use lettre::{SmtpTransport, transport::smtp::authentication::Credentials};
use sqlx::PgPool;

use crate::config::CONFIG;

pub struct AppState {
    pub db: PgPool,
    pub smtp: SmtpTransport,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        let smtp = SmtpTransport::relay(&CONFIG.mail_config.host)
            .expect("SMTP Host not specified")
            .credentials(Credentials::new(
                CONFIG.mail_config.username.clone(),
                CONFIG.mail_config.password.clone(),
            ))
            .build();

        Self { db, smtp }
    }
}

pub type AppStateRef = Arc<AppState>;

// impl FromRef<AppState> for PgPool {
//     fn from_ref(state: &AppState) -> Self {
//         state.db.clone()
//     }
// }
