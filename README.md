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

### 1. Set up PostgreSQL database

Ensure PostgreSQL is running on your system.

### 2. Configure environment variables

Copy `.env.example` to `.env` and configure:

```bash
cp .env.example .env
```

All configuration variables are required:

```bash
# Server Configuration
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres
SERVER_ADDR=127.0.0.1:8000
SITE_NAME="My App"

# Email Configuration
BASE_URL=http://127.0.0.1:8000
EMAIL_FROM_ADDRESS=noreply@example.com
EMAIL_FROM_NAME="My App"
EMAIL_MODE=console  # or "smtp" for production
```

**Note**: Values with spaces must be quoted (e.g., `SITE_NAME="My App"`).

### 3. Run migrations and start

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

### Email Modes

**Development Mode** (logs magic links to console):
```bash
EMAIL_MODE=console
```
Magic links appear in terminal output - no SMTP configuration needed.

**Production Mode** (sends real emails):
```bash
EMAIL_MODE=smtp
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
```
Works with any SMTP provider (Gmail, SendGrid, AWS SES, etc.).

## Project Structure

- Type-first routing (`/pages/*`, `/forms/*`, `/actions/*`)
- RESTful API design with proper HTTP methods
- Single source of truth for paths in `src/paths.rs`
