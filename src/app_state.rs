use std::sync::Arc;

use lettre::{SmtpTransport, transport::smtp::authentication::Credentials};
use s3::Bucket;
use sqlx::PgPool;

use crate::{config::CONFIG, ctx::services::s3::get_bucket};

pub struct AppState {
    pub db: PgPool,
    pub smtp: SmtpTransport,
    pub s3: Bucket,
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

        let s3 = get_bucket();

        Self { db, smtp, s3 }
    }
}

pub type SharedAppState = Arc<AppState>;

// impl FromRef<AppState> for PgPool {
//     fn from_ref(state: &AppState) -> Self {
//         state.db.clone()
//     }
// }
