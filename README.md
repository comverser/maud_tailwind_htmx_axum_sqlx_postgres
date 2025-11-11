# Web App Template

A minimal, production-ready Rust web application template. Built for developers who want a clean starting point with essential features already implemented: authentication, payments, sessions, and database integration.

**Use Case:** Start building your web app immediately without writing boilerplate for auth, payments, or basic CRUD operations.

## Tech Stack

**Backend:** Axum • PostgreSQL + SQLx • Maud templates<br>
**Frontend:** HTMX • Tailwind CSS<br>
**Auth:** Magic links • tower-sessions<br>
**Payments:** Toss Payments

## Quick Start

```bash
# 1. Install dependencies
# Requires: Rust, PostgreSQL, just (cargo install just)

# 2. Copy and configure environment
cp .env.example .env
# Edit .env with your values (see Configuration below)

# 3. Run migrations and start server
just
```

Server starts at `http://127.0.0.1:8000`

## Configuration

All configuration is required - the app will fail fast at startup if any required variable is missing.

### Minimal Setup (Development)

```bash
# Server
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres
SERVER_ADDR=127.0.0.1:8000
SITE_NAME="My App"

# Email (console mode - logs to terminal)
BASE_URL=http://127.0.0.1:8000
EMAIL_FROM_ADDRESS=dev@localhost
EMAIL_FROM_NAME="Dev"
EMAIL_MODE=console

# Payments (test keys from app.tosspayments.com)
TOSS_CLIENT_KEY=test_ck_CHANGE_ME
TOSS_SECRET_KEY=test_sk_CHANGE_ME
```

**Note:** Values with spaces must be quoted in `.env` file.

### Production Setup

**Email (SMTP):**
```bash
EMAIL_MODE=smtp
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
```

**Payments:**
1. Sign up at [Toss Payments](https://app.tosspayments.com/)
2. Get API keys from **Settings → API Keys**
3. Replace test keys with live keys in `.env`

## Features

This template includes:

- **Passwordless Authentication** - Magic link email authentication with 15-minute token expiry
- **Session Management** - PostgreSQL-backed sessions via tower-sessions
- **Payment Processing** - Full Toss Payments integration with order tracking
- **File Uploads** - Multipart form handling (10MB limit, text analysis demo)
- **Email Service** - Dual mode: console logging (dev) or SMTP (production)
- **CRUD Example** - Todo list demonstrating database operations
- **Security** - CSRF protection, security headers, server-side payment verification

### Demo Pages

- **Home** - Contact form (sends email)
- **Sign In** - Magic link authentication
- **Dashboard** - User orders list
- **Todos** - Simple CRUD example
- **Text Analyzer** - File upload → quote → payment → results flow

## Architecture

### Type-First Routing

This template organizes routes by **interaction type** rather than resource:

```
GET /pages              POST /forms               POST/DELETE /actions
├─ /                    ├─ /forms/sign_in         ├─ /actions/sign_out
├─ /dashboard           ├─ /forms/todos           ├─ /actions/todos/{id}
├─ /sign_in             ├─ /forms/text_analyzer   ├─ /actions/payment/verify
├─ /text_analyzer       └─ /forms/contact         └─ ...
└─ /todos
```

**Why?** URL structure reveals intent. Pages render UI, forms submit data, actions mutate state.

### Project Structure

```
src/
├── routes/           # Route registration + middleware
│   ├── pages.rs      # GET routes (render HTML)
│   ├── forms.rs      # POST routes (form submissions)
│   └── actions.rs    # POST/DELETE/PATCH (state changes)
├── handlers/         # Request handlers (business logic)
│   ├── pages/        # Page handlers
│   ├── forms/        # Form handlers
│   └── actions/      # Action handlers
├── views/            # Maud HTML templates
│   ├── pages/        # Page templates
│   ├── layout/       # Shared layouts
│   └── components/   # Reusable components
├── data/             # Database layer (CQRS)
│   ├── queries/      # Read operations (SELECT)
│   └── commands/     # Write operations (INSERT/UPDATE/DELETE)
├── models/           # Domain models + validation
├── middlewares/      # Request/response processing
├── paths.rs          # ALL URL paths (centralized)
├── constants.rs      # App-wide constants
└── config.rs         # Environment configuration
```

### Core Patterns

**1. CQRS Data Layer**
```rust
// Queries (reads)
data::queries::order::get_orders_for_user(&db, user_id, limit).await?

// Commands (writes)
data::commands::order::create_order(&db, user_id, filename, ...).await?
```

**2. Centralized Path Management**
```rust
// Define once in src/paths.rs
pub const DASHBOARD: &str = "/dashboard";
pub const QUOTE: &str = "/quote";

// Use everywhere
paths::pages::DASHBOARD
paths::with_param(paths::pages::QUOTE, "order_id", &order_id)
```

**3. Middleware Chain**
```
Request
  → Security Headers (CSP, HSTS, etc.)
  → HTTP Tracing
  → Session Management
  → Auth Context Injection
  → Handler
```

### Design Principles

See `CLAUDE.md` for full development guidelines. Key principles:

- **Single Standard Principle** - One consistent way to handle each case
- **Explicit Over Implicit** - Required config, fail-fast errors, no silent defaults
- **Keep Code Simple** - Don't abstract until duplication appears
- **Essential Elements Only** - Minimal UI with no decoration (no shadows, rounded corners, font weights)
- **Type-First Organization** - Group by interaction type, not resource
- **Centralized Constants** - No hardcoded paths, magic values, or scattered config

## Development

### Useful Commands

```bash
just                  # Run migrations + start server
just migrate          # Run database migrations only
just reset            # Drop and recreate database
cargo check           # Check compilation
cargo test            # Run tests
```

### Database Migrations

Migrations are in `migrations/` directory. Add new migrations with:

```bash
# Manual: Create file migrations/YYYYMMDDHHMMSS_description.sql
# Or use sqlx-cli:
sqlx migrate add description
```

### Adding New Features

1. **Add route** in `src/routes/` (pages/forms/actions)
2. **Add path constant** in `src/paths.rs`
3. **Create handler** in `src/handlers/`
4. **Create view** in `src/views/` (if rendering HTML)
5. **Add queries/commands** in `src/data/` (if touching database)
6. **Update navigation** in `src/views/layout/navigation.rs` (if needed)

## License

This is a template repository - use it however you want.
