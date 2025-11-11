use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

// MUST match struct field names for proper form deserialization
pub const FIELD_EMAIL: &str = "email";

static EMAIL_RX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .expect("Email regex pattern is invalid")
});

#[derive(Deserialize, Validate)]
pub struct MagicLinkRequestForm {
    #[validate(regex(path = "*EMAIL_RX", message = "Invalid email format"))]
    pub email: String,
}
