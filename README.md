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
just run

# 4. (Optional) Grant admin role to your user
just admin-grant your-email@example.com
```

Server starts at `http://127.0.0.1:8000`

### First Steps

1. Visit `/sign_in` and enter your email
2. Check console for magic link (EMAIL_MODE=console)
3. Click link to sign in
4. Grant admin: `just admin-grant your-email@example.com`
5. Visit `/admin`

## Configuration

All configuration required - fails fast at startup if missing.

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

- **Passwordless Auth** - Magic link authentication (15-min expiry)
- **Sessions** - PostgreSQL-backed via tower-sessions
- **Payments** - Toss Payments integration with order tracking
- **Admin Dashboard** - Role-based access, user/order management
- **File Uploads** - Multipart forms (10MB limit)
- **Email** - Console (dev) or SMTP (production)
- **CRUD Example** - Todo list
- **Security** - CSRF protection, security headers, verified payments

### Demo Pages

- **Home** - Contact form
- **Sign In** - Magic link auth
- **Dashboard** - User orders
- **Todos** - CRUD example
- **Text Analyzer** - File upload → payment → results
- **Admin** - User/order management, stats (admin only)

## Architecture

### Type-First Routing

This template organizes routes by **interaction type** rather than resource:

```
GET /pages              POST /forms               POST/DELETE /actions
├─ /                    ├─ /forms/sign_in         ├─ /actions/sign_out
├─ /dashboard           ├─ /forms/todos           ├─ /actions/todos/{id}
├─ /sign_in             ├─ /forms/text_analyzer   ├─ /actions/payment/verify
├─ /text_analyzer       └─ /forms/contact         └─ /actions/auth/verify
├─ /todos
├─ /admin               Admin Routes (protected)
│  ├─ /admin            ├─ /forms/admin/...       ├─ /actions/admin/...
│  ├─ /admin/users      └─ Grant/revoke roles     └─ Delete resources
│  └─ /admin/orders
```

**Why?** URL structure reveals intent. Pages render UI, forms submit data, actions mutate state.

### Project Structure

```
src/
├── routes/        # Route registration (pages/forms/actions)
├── handlers/      # Request handlers (pages/forms/actions + admin)
├── views/         # Maud HTML templates (pages/components)
├── data/          # Database layer (CQRS)
│   ├── queries/   # Read operations
│   └── commands/  # Write operations
├── models/        # Domain models + validation
├── middlewares/   # Session, auth, admin protection
├── paths.rs       # Centralized URL paths
├── constants.rs   # App-wide constants
└── config.rs      # Environment configuration
```

### Core Patterns

**CQRS Data Layer**
```rust
data::queries::order::get_orders_for_user(&db, user_id, limit).await?
data::commands::order::create_order(&db, params).await?
```

**Centralized Paths**
```rust
// Define in src/paths.rs, use everywhere
paths::pages::DASHBOARD
paths::helpers::quote_path(&order_id)
```

**Type-Safe Errors**
```rust
pub async fn handler(State(db): State<PgPool>) -> HandlerResult {
    let data = queries::get_something(&db).await?;
    Ok(render_page(data).into_response())
}
// DataError variants → 404/401/400/500
```

### Design Principles

See `CLAUDE.md` for full guidelines. Key principles: single standard, explicit over implicit, simple code, minimal UI, type-first routing.

## Development

### Useful Commands

```bash
# Development
just                        # Show all available commands
just run                    # Run migrations + start server with auto-reload
just migrate                # Run database migrations only
just migrate-reset          # Reset database (revert all + re-run)

# Admin Management
just admin-grant <email>    # Grant admin role to user
just admin-revoke <email>   # Revoke admin role from user
just admin-list             # List all admin users

# Other
cargo check                 # Check compilation
cargo test                  # Run tests
```

### Database Migrations

Single consolidated migration creates all tables: `users`, `magic_links`, `todos`, `orders`, `user_roles`.

Add new migrations:
```bash
just migrate-add description
```

Migrations run automatically with `just run`.

### Adding Features

1. Add path in `src/paths.rs`
2. Add route in `src/routes/`
3. Create handler in `src/handlers/`
4. Create view in `src/views/` (if needed)
5. Add data layer in `src/data/` (if needed)

## License

This is a template repository - use it however you want.
