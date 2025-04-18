use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{MediaType, Post, PostMedia};

#[derive(Deserialize)]
pub struct PostMediaDto {
    pub url: String,
    pub r#type: MediaType,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub size: Option<i32>,
}

#[derive(Deserialize, Validate)]
pub struct CreatePostDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Title's length must be between 1 and 100 characters"
    ))]
    pub title: String,
    #[validate(length(min = 1, message = "Post's content is required"))]
    pub content: String,
    #[serde(default = "Vec::new")]
    pub media: Vec<PostMediaDto>,
}

#[derive(Deserialize, Validate)]
pub struct UpdatePostDto {
    #[validate(length(min = 1, message = "Post's content is required"))]
    pub content: String,
}

#[derive(Serialize)]
pub struct CreatePostResponseDto {
    #[serde(flatten)]
    pub post: Post,
    pub media: Vec<PostMedia>,
}
