use serde::Deserialize;
use validator::Validate;

use crate::validation::EMAIL_RX;

// MUST match struct field names for proper form deserialization
pub const FIELD_EMAIL: &str = "email";

#[derive(Deserialize, Validate)]
pub struct MagicLinkRequestForm {
    #[validate(regex(path = "*EMAIL_RX", message = "Invalid email format"))]
    pub email: String,
}
