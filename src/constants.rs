use std::{ops::Range, sync::LazyLock, time::Duration};

use crate::config::CONFIG;

pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

pub const WEBSITE_URL: LazyLock<String> =
    LazyLock::new(|| std::env::var("WEBSITE_URL").unwrap_or("https://example.com".to_string()));

pub const PIN_RANGE: Range<u32> = 1000000..9999999;

pub const VERIFICATION_PIN_EXPIRATION_TIME: Duration = Duration::from_secs(60 * 5); // 5 minutes

pub const DEFAULT_POSTS_PAGINATION_LIMIT: i32 = 20;

pub const SERVER_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("SERVER_URL").unwrap_or(format!("http://localhost:{}", CONFIG.port).to_string())
});

pub const DISK_STORAGE_PATH: LazyLock<String> =
    LazyLock::new(|| std::env::var("DISK_STORAGE_PATH").unwrap_or("uploads".to_string()));

// pub const TEST_PIN: u32 = 1234567;
