use serde::Deserialize;
use validator::Validate;

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
    pub media: Option<Vec<String>>,
}

#[derive(Deserialize, Validate)]
pub struct UpdatePostDto {
    #[validate(length(min = 1, message = "Post's content is required"))]
    pub content: String,
}
