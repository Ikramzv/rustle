use serde::Deserialize;

use crate::constants::DEFAULT_POSTS_PAGINATION_LIMIT;

#[derive(Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_offset")]
    pub offset: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

fn default_offset() -> i32 {
    0
}

fn default_limit() -> i32 {
    DEFAULT_POSTS_PAGINATION_LIMIT
}
