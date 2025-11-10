pub mod auth {
    /// Magic link token expiry time in minutes
    pub const MAGIC_LINK_EXPIRY_MINUTES: i64 = 15;
}

pub mod messages {
    /// Success message after sending magic link email
    pub const MAGIC_LINK_SENT: &str = "Check your email! We sent you a link to sign in.";

    /// Success message after signing in
    pub const SIGNED_IN: &str = "Successfully signed in!";

    /// Success message after signing out
    pub const SIGNED_OUT: &str = "You have been signed out.";

    /// Success message after creating a todo
    pub const TODO_CREATED: &str = "Todo created successfully";

    /// Error message when email service is not configured
    pub const EMAIL_NOT_CONFIGURED: &str = "Email service is not configured. Please contact support.";

    /// Error message when sending email fails
    pub const EMAIL_SEND_FAILED: &str = "Failed to send email. Please try again.";

    /// Error message for invalid or expired magic link
    pub const MAGIC_LINK_INVALID: &str = "Invalid or expired magic link. Please request a new one.";
}
