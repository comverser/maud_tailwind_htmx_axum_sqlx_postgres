use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use crate::paths;

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Email build error: {0}")]
    Build(#[from] lettre::error::Error),
    #[error("Email transport error: {0}")]
    Transport(#[from] lettre::transport::smtp::Error),
    #[error("Invalid email address: {0}")]
    InvalidAddress(#[from] lettre::address::AddressError),
}

pub struct EmailConfig {
    mode: EmailMode,
    from_address: String,
    from_name: String,
    base_url: String,
}

pub enum EmailMode {
    Console,
    Smtp {
        host: String,
        port: u16,
        username: String,
        password: String,
    },
}

impl EmailConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let mode = match dotenvy::var("EMAIL_MODE").as_deref() {
            Ok("smtp") => {
                let host = dotenvy::var("SMTP_HOST").expect("SMTP_HOST must be set");
                let port = dotenvy::var("SMTP_PORT")
                    .expect("SMTP_PORT must be set")
                    .parse()
                    .expect("SMTP_PORT must be a valid number");
                let username = dotenvy::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
                let password = dotenvy::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

                EmailMode::Smtp {
                    host,
                    port,
                    username,
                    password,
                }
            }
            _ => EmailMode::Console,
        };

        let from_address = dotenvy::var("EMAIL_FROM_ADDRESS")
            .unwrap_or_else(|_| "noreply@example.com".to_string());
        let from_name =
            dotenvy::var("EMAIL_FROM_NAME").unwrap_or_else(|_| "Magic Link Auth".to_string());
        let base_url =
            dotenvy::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());

        Self {
            mode,
            from_address,
            from_name,
            base_url,
        }
    }
}

pub async fn send_magic_link(
    config: &EmailConfig,
    to_email: &str,
    token: &str,
) -> Result<(), EmailError> {
    let magic_link = format!("{}{}?token={}", config.base_url, paths::actions::VERIFY_MAGIC_LINK, token);

    let from_mailbox: Mailbox = format!("{} <{}>", config.from_name, config.from_address).parse()?;
    let to_mailbox: Mailbox = to_email.parse()?;

    let email = Message::builder()
        .from(from_mailbox)
        .to(to_mailbox)
        .subject("Sign in to your account")
        .header(ContentType::TEXT_HTML)
        .body(format!(
            r#"
            <html>
                <body style="font-family: sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
                    <h2>Sign in to your account</h2>
                    <p>Click the link below to sign in. This link will expire in 15 minutes.</p>
                    <p style="margin: 30px 0;">
                        <a href="{}" style="background-color: #4F46E5; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; display: inline-block;">
                            Sign In
                        </a>
                    </p>
                    <p style="color: #666; font-size: 14px;">
                        Or copy and paste this link into your browser:<br>
                        <a href="{}">{}</a>
                    </p>
                    <p style="color: #999; font-size: 12px; margin-top: 40px;">
                        If you didn't request this email, you can safely ignore it.
                    </p>
                </body>
            </html>
            "#,
            magic_link, magic_link, magic_link
        ))?;

    match &config.mode {
        EmailMode::Console => {
            tracing::info!("\n\n========== MAGIC LINK EMAIL ==========");
            tracing::info!("To: {}", to_email);
            tracing::info!("Magic Link: {}", magic_link);
            tracing::info!("======================================\n");
            Ok(())
        }
        EmailMode::Smtp {
            host,
            port,
            username,
            password,
        } => {
            let creds = Credentials::new(username.to_string(), password.to_string());
            let mailer = SmtpTransport::relay(host)?
                .port(*port)
                .credentials(creds)
                .build();

            mailer.send(&email)?;
            tracing::info!("Magic link email sent to {}", to_email);
            Ok(())
        }
    }
}
