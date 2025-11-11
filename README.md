# Web App Template

A production-ready Rust web application template with authentication, payments, and file processing.

## Features

- 🔐 **Passwordless Auth** - Magic link email authentication (15-min expiry)
- 📧 **Email Services** - Console mode (dev) and SMTP mode (production)
- 💳 **Payment Processing** - Toss Payments integration with order management
- 📁 **File Uploads** - Multipart form uploads with text analysis (10MB limit)
- 🏗️ **Clean Architecture** - Type-first routing, CQRS data layer, centralized paths
- 🔒 **Security** - CSRF protection, security headers, server-side payment verification

## Tech Stack

**Backend:** Axum • PostgreSQL + SQLx • Maud templates
**Frontend:** HTMX • Tailwind CSS
**Auth:** Magic links • tower-sessions
**Payments:** Toss Payments

## Quick Start

```bash
# 1. Copy and configure environment
cp .env.example .env

# 2. Run migrations and start server
just
```

### Required Environment Variables

```bash
# Server
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres
SERVER_ADDR=127.0.0.1:8000
SITE_NAME="My App"

# Email
BASE_URL=http://127.0.0.1:8000
EMAIL_FROM_ADDRESS=your-email@your-domain.com
EMAIL_FROM_NAME="Support"
EMAIL_MODE=console  # "smtp" for production

# Payments (get keys from app.tosspayments.com)
TOSS_CLIENT_KEY=test_ck_CHANGE_ME
TOSS_SECRET_KEY=test_sk_CHANGE_ME
```

**Note:** Values with spaces must be quoted.

## Configuration

### Email Setup

**Development** (logs to console):
```bash
EMAIL_MODE=console
```

**Production** (requires SMTP):
```bash
EMAIL_MODE=smtp
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
```

### Payment Setup

1. Sign up at [Toss Payments](https://app.tosspayments.com/)
2. Get API keys from **Settings → API Keys**
3. Add to `.env`:
   - `TOSS_CLIENT_KEY` - For browser (SDK)
   - `TOSS_SECRET_KEY` - For server (API, keep secret)

## Architecture

### Type-First Routing

Routes organized by interaction type, not resource:

```
Pages (GET)           Forms (POST)              Actions (POST/DELETE)
├─ /                  ├─ /forms/sign_in         ├─ /actions/sign_out
├─ /todos             ├─ /forms/todos           ├─ /actions/todos/{id}
├─ /sign_in           ├─ /forms/text_analyzer   ├─ /actions/payment/initiate
├─ /quote/{id}        └─ /forms/contact         └─ /actions/payment/verify
└─ /checkout/{id}
```

**Benefits:** URL shows intent • Clear separation • RESTful within type

### Project Structure

```
src/
├── routes/          # Route definitions + middleware
│   ├── pages.rs     # GET routes
│   ├── forms.rs     # POST routes (forms)
│   └── actions.rs   # POST/DELETE/PATCH (mutations)
├── handlers/        # Request handlers
├── data/            # Database layer (CQRS)
│   ├── queries/     # SELECT operations
│   └── commands/    # INSERT/UPDATE/DELETE
├── views/           # Maud templates
├── models/          # Data structures + validation
├── middlewares/     # Request/response processing
├── paths.rs         # Centralized URL definitions
└── config.rs        # Environment configuration
```

### Key Patterns

**CQRS Data Layer**
```rust
// Reads
data::queries::todo::get_todos_by_user(db, user_id)

// Writes
data::commands::todo::create_todo(db, user_id, content)
```

**Centralized Paths**
```rust
paths::pages::TODOS           // "/todos"
paths::with_param(paths::actions::TODOS_TODO_ID, "todo_id", &123)
```

**Middleware Chain**
```
Request → Security Headers → HTTP Tracing → Session → Auth → Handler
```

### Design Principles

- **Single Standard** - One way to handle each case
- **Explicit Over Implicit** - Fail fast with clear errors
- **No Magic Values** - Constants for all repeated values
- **Type-First Organization** - Group by interaction type
- **No Path Hardcoding** - All URLs in `src/paths.rs`

## Demo Features

### Authentication Flow
User enters email → Receives magic link → Clicks link → Authenticated (15-min token)

### Payment Flow
Upload file → View quote → Checkout → Payment verification → View result

### Contact Form
User submits inquiry → Email sent to admin (console or SMTP)

## License

This is a template repository - use it however you want.
