use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UserLogin {
    // Since `required` doesn't accept messages, we need to bypass by asking
    // a message having at least one charater
    #[validate(length(min = 1, message = "This field is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "This field is required"))]
    pub password: String,
}
