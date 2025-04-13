use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UpdateProfileDto {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: Option<String>,
    #[validate(url(message = "Invalid profile image url"))]
    pub profile_image_url: Option<String>,
}
