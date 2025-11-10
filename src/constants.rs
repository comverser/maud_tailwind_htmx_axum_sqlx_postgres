pub mod auth {
    /// Magic link token expiry time in minutes
    pub const MAGIC_LINK_EXPIRY_MINUTES: i64 = 15;
}

pub mod cdn {
    /// Tailwind CSS browser CDN URL
    pub const TAILWIND_CSS_URL: &str = "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4";

    /// HTMX library CDN URL
    pub const HTMX_URL: &str = "https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js";

    /// HTMX library integrity hash for SRI (Subresource Integrity)
    pub const HTMX_INTEGRITY: &str = "sha384-ZBXiYtYQ6hJ2Y0ZNoYuI+Nq5MqWBr+chMrS/RkXpNzQCApHEhOt2aY8EJgqwHLkJ";

    /// Hyperscript library CDN URL
    pub const HYPERSCRIPT_URL: &str = "https://unpkg.com/hyperscript.org@0.9.14";
}

pub mod error_pages {
    /// Fallback site name used in error pages when config is not available
    pub const FALLBACK_SITE_NAME: &str = "App";
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

    /// Error message when sending email fails
    pub const EMAIL_SEND_FAILED: &str = "Failed to send email. Please try again.";

    /// Error message for invalid or expired magic link
    pub const MAGIC_LINK_INVALID: &str = "Invalid or expired magic link. Please request a new one.";
}
