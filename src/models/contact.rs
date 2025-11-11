use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

pub const FIELD_EMAIL: &str = "email";
pub const FIELD_MESSAGE: &str = "message";

static EMAIL_RX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .expect("Email regex pattern is invalid")
});

#[derive(Deserialize, Validate)]
pub struct ContactForm {
    #[validate(regex(path = "*EMAIL_RX", message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, message = "Message cannot be empty"))]
    pub message: String,
}
