use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::user::User;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct LoginUserDto {
    #[validate(length(min = 1, message = "Email is required"))]
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    pub profile_image_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyEmailDto {
    #[validate(length(min = 1, message = "Email is required"))]
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 1, message = "Pin is required"))]
    pub pin: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyResponseDto {
    pub token: String,
    pub user: User,
}
