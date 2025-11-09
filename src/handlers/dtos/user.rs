use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

// Field name constants used across forms, validation, and views
// These MUST match the struct field names below for proper deserialization
pub const FIELD_EMAIL: &str = "email";

static EMAIL_RX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .expect("Email regex pattern is invalid")
});

/// Magic Link authentication form - only requires email
#[derive(Deserialize, Validate)]
pub struct MagicLinkRequestForm {
    #[validate(regex(path = "*EMAIL_RX", message = "Invalid email format"))]
    pub email: String,  // Field name must match FIELD_EMAIL constant
}
