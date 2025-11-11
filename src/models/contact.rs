use serde::Deserialize;
use validator::Validate;

use crate::validation::EMAIL_RX;

pub const FIELD_EMAIL: &str = "email";
pub const FIELD_MESSAGE: &str = "message";

#[derive(Deserialize, Validate)]
pub struct ContactForm {
    #[validate(regex(path = "*EMAIL_RX", message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, message = "Message cannot be empty"))]
    pub message: String,
}
