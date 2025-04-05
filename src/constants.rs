use std::{ops::Range, sync::LazyLock, time::Duration};

pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
pub const WEBSITE_URL: LazyLock<String> =
    LazyLock::new(|| std::env::var("WEBSITE_URL").unwrap_or("https://example.com".to_string()));

pub const PIN_RANGE: Range<u32> = 1000000..9999999;

pub const VERIFICATION_PIN_EXPIRATION_TIME: Duration = Duration::from_secs(60 * 5); // 5 minutes
