use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

#[derive(Debug, Type)]
#[sqlx(type_name = "MediaType")]
pub enum MediaType {
    Image,
    Video,
    Document,
    Audio,
    Other,
}

impl MediaType {
    pub fn to_str(&self) -> &str {
        match self {
            MediaType::Image => "image",
            MediaType::Video => "video",
            MediaType::Document => "document",
            MediaType::Audio => "audio",
            MediaType::Other => "other",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub profile_image_url: Option<String>,
    pub is_verified: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct VerificationPin {
    pub id: String,
    pub email: String,
    pub pin: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
