# Web App Template

A minimal web application template built with Rust.

## Tech Stack

- **Backend**: Axum
- **Database**: PostgreSQL with SQLx
- **Templates**: Maud
- **Frontend**: HTMX + Tailwind CSS
- **Sessions**: tower-sessions with PostgreSQL store
- **Authentication**: Magic Link (passwordless)

## Getting Started

1. Set up PostgreSQL database
2. Configure environment variables
3. Run migrations
4. Start the server

```bash
just
```

## Magic Link Authentication

This template uses passwordless authentication via magic links. Users simply enter their email address and receive a link to sign in - no passwords required!

### How It Works

1. User enters their email on the sign-in page
2. System generates a secure token and sends an email with a magic link
3. User clicks the link to authenticate (valid for 15 minutes)
4. System creates or retrieves user account and establishes a session

### Email Configuration

Configure email in `.env`:

```bash
# Development mode (logs magic links to console)
EMAIL_MODE=console
BASE_URL=http://127.0.0.1:8000
EMAIL_FROM_ADDRESS=noreply@example.com
EMAIL_FROM_NAME=Your App Name

# Production mode (sends real emails via SMTP)
EMAIL_MODE=smtp
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
```

**Development**: Use `EMAIL_MODE=console` to see magic links in your terminal output.

**Production**: Use `EMAIL_MODE=smtp` with your SMTP credentials (Gmail, SendGrid, AWS SES, etc.).

## Project Structure

- Type-first routing (`/pages/*`, `/forms/*`, `/actions/*`)
- RESTful API design with proper HTTP methods
- Single source of truth for paths in `src/paths.rs`
