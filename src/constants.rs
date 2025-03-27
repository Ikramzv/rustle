use std::{sync::LazyLock, time::Duration};

pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
pub const WEBSITE_URL: LazyLock<String> =
    LazyLock::new(|| std::env::var("WEBSITE_URL").unwrap_or("https://example.com".to_string()));
