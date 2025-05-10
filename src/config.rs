use std::{str::FromStr, sync::LazyLock};

use std::time::Duration;

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::build);

pub struct MailConfig {
    pub from: String,
    pub host: String,
    pub username: String,
    pub password: String,
}

impl MailConfig {
    pub fn new() -> Self {
        let from = std::env::var("SMTP_FROM").expect("SMTP_FROM is not set");
        let host = std::env::var("SMTP_HOST").expect("SMTP_HOST is not set");
        let username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME is not set");
        let password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not set");

        Self {
            from,
            host,
            username,
            password,
        }
    }
}

pub struct Config {
    pub env: Env,
    pub db_url: String,
    pub mail_config: MailConfig,
    pub jwt_secret: String,
    pub jwt_expiration_duration: Duration,
    pub request_body_limit: usize,
    pub port: u16,
}

impl Config {
    pub fn build() -> Self {
        let env = Env::new();

        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

        let mail_config = MailConfig::new();

        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is not set");

        let jwt_expiration_duration = std::env::var("JWT_EXPIRATION_DURATION")
            .map(|s| Duration::from_secs(s.parse::<u64>().unwrap()))
            .expect("JWT_EXPIRATION_DURATION is not set");

        let request_body_limit = std::env::var("REQUEST_BODY_LIMIT")
            .map(|s| s.parse::<u64>().unwrap())
            .unwrap_or(5 * 1024 * 1024); // 5 Mb

        let port = std::env::var("PORT")
            .map(|s| s.parse::<u16>().unwrap())
            .unwrap_or(3001);

        Self {
            env,
            db_url,
            mail_config,
            jwt_secret,
            jwt_expiration_duration,
            request_body_limit: request_body_limit as usize,
            port,
        }
    }
}

pub enum Env {
    DEV,
    RELEASE,
}

impl Env {
    fn new() -> Self {
        match std::env::var("CARGO_PROFILE") {
            Ok(s) => Env::from_str(&s).unwrap(),
            Err(_) => {
                Env::warn_unknown_env("CARGO_PROFILE is not set");
                Env::DEV
            }
        }
    }

    fn warn_unknown_env(s: &str) {
        tracing::warn!(
            "Unknown environment: {}\nUsing DEV environment by default",
            s
        );
    }
}

impl FromStr for Env {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Env::DEV),
            "release" => Ok(Env::RELEASE),
            _ => {
                Env::warn_unknown_env(s);
                Ok(Env::DEV)
            }
        }
    }
}
