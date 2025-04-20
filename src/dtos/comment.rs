use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateCommentDto {
    pub parent_id: Option<String>,
    #[validate(length(min = 1, message = "Post ID is required"))]
    pub post_id: String,
    #[validate(length(
        min = 1,
        max = 10000,
        message = "Content characters must be between 1 and 10000"
    ))]
    pub content: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateCommentDto {
    #[validate(length(
        min = 1,
        max = 10000,
        message = "Content characters must be between 1 and 10000"
    ))]
    pub content: String,
}
