use std::{str::FromStr, sync::LazyLock};

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::build);

pub struct Config {
    pub env: Env,
}

impl Config {
    pub fn build() -> Self {
        let env = std::env::var("CARGO_PROFILE")
            .map(|s| Env::from_str(&s).unwrap())
            .expect("CARGO_PROFILE is not set");

        Self { env }
    }
}

pub enum Env {
    DEV,
    RELEASE,
}

impl FromStr for Env {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Env::DEV),
            "release" => Ok(Env::RELEASE),
            _ => Err("Error parsing Config::env : Unknown environment".to_string()),
        }
    }
}
