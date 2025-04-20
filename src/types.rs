use std::fmt;

use serde::{Deserialize, de::Visitor};

use crate::constants::DEFAULT_POSTS_PAGINATION_LIMIT;

pub struct PaginationQuery {
    pub offset: i64,
    pub limit: i64,
}

// both offset and limit can be missing or invalid type (string that cannot be parsed to i64)
// in those cases we use default values

struct PaginationQueryVisitor;

impl<'de> Visitor<'de> for PaginationQueryVisitor {
    type Value = PaginationQuery;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map with offset and limit")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut offset = None;
        let mut limit = None;

        while let Some((key, value)) = map.next_entry::<&str, String>()? {
            match key {
                "offset" => {
                    if offset.is_some() {
                        return Err(serde::de::Error::duplicate_field("offset"));
                    }
                    offset = Some(value.parse().unwrap_or(default_offset()));
                }
                "limit" => {
                    if limit.is_some() {
                        return Err(serde::de::Error::duplicate_field("limit"));
                    }
                    limit = Some(value.parse().unwrap_or(default_limit()));
                }
                _ => (),
            }
        }

        let mut offset = offset.unwrap_or(default_offset());
        let mut limit = limit.unwrap_or(default_limit());

        if limit < 0 {
            limit = default_limit();
        }

        if offset < 0 {
            offset = default_offset();
        }

        Ok(PaginationQuery { offset, limit })
    }
}

impl<'de> Deserialize<'de> for PaginationQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(PaginationQueryVisitor)
    }
}

fn default_offset() -> i64 {
    0
}

fn default_limit() -> i64 {
    DEFAULT_POSTS_PAGINATION_LIMIT as i64
}
