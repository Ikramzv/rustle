use std::{ops::Range, path::PathBuf, sync::LazyLock, time::Duration};

pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
pub const WEBSITE_URL: LazyLock<String> =
    LazyLock::new(|| std::env::var("WEBSITE_URL").unwrap_or("https://example.com".to_string()));

pub const PIN_RANGE: Range<u32> = 1000000..9999999;

pub const VERIFICATION_PIN_EXPIRATION_TIME: Duration = Duration::from_secs(60 * 5); // 5 minutes

pub const REQUEST_TIMEOUT_DURATION: Duration = Duration::from_secs(30);

pub const UPLOAD_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut current_dir = std::env::current_dir().unwrap();
    current_dir.push("uploads");
    current_dir
});

pub const REQUEST_BODY_LIMIT: LazyLock<u64> = LazyLock::new(|| {
    let limit = std::env::var("REQUEST_BODY_LIMIT").unwrap_or("5".to_string());
    limit.parse::<u64>().unwrap_or(5) * 1024 * 1024
}); // 5 Mb
