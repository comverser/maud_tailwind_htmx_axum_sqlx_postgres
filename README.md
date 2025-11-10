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

## Core Structure

This template follows a **type-first routing architecture** that organizes code by interaction type rather than by resource. This makes the application's behavior immediately clear from its URL structure.

### Route Organization

Routes are grouped by **what they do**, not **what they operate on**:

```
GET  /                     → Render homepage
GET  /todos                → Render todos page
GET  /sign_in              → Render sign-in page

POST /forms/sign_in        → Process sign-in form
POST /forms/todos          → Process new todo form

POST   /actions/sign_out   → Sign out action
DELETE /actions/todos/{id} → Delete todo action
POST   /actions/todos/{id}/toggle → Toggle todo action
```

**Why this pattern?**
- URL immediately tells you the interaction type (viewing, submitting form, or state change)
- Clear separation between reads (pages), form submissions (forms), and mutations (actions)
- RESTful HTTP methods (GET, POST, DELETE, PATCH) within each type

### Layer Architecture

The codebase is organized into clear layers with distinct responsibilities:

```
src/
├── routes/          # Route definitions & middleware configuration
│   ├── pages.rs     # GET-only routes (viewing)
│   ├── forms.rs     # POST routes (form submissions)
│   └── actions.rs   # POST/DELETE/PATCH routes (state changes)
│
├── handlers/        # Request handlers (one per route)
│   ├── pages/       # Render views
│   ├── forms/       # Process & validate form data
│   └── actions/     # Execute state-changing operations
│
├── data/            # Database access layer (CQRS pattern)
│   ├── queries/     # Read operations (SELECT)
│   └── commands/    # Write operations (INSERT, UPDATE, DELETE)
│
├── views/           # HTML templates (Maud)
│   ├── layout/      # Base structure & navigation
│   ├── components/  # Reusable UI elements
│   └── pages/       # Full page templates
│
├── middlewares/     # Request/response processing
│   ├── session.rs   # Load user from session
│   ├── auth.rs      # Require authentication
│   └── ...
│
├── models/          # Data structures & validation
├── paths.rs         # Single source of truth for all URLs
└── config.rs        # Application configuration
```

### Key Architectural Patterns

**1. CQRS Data Layer**
```rust
// Queries - read operations
data::queries::todo::get_todos_by_user(db, user_id)

// Commands - write operations
data::commands::todo::create_todo(db, user_id, content)
```
Separates reads from writes for clearer intent and better organization.

**2. Middleware Chain**
```
Request
  ↓
Security Headers → HTTP Tracing → Session Loading → Auth Check → Handler
```
The order matters! See `src/routes/mod.rs` for the critical middleware ordering.

**3. Path Management**
```rust
// All paths defined in one place
paths::pages::TODOS           // "/todos"
paths::forms::SIGN_IN         // "/forms/sign_in"
paths::actions::TODOS_TODO_ID // "/actions/todos/{todo_id}"

// Helper for parameters
paths::with_param(paths::actions::TODOS_TODO_ID, "todo_id", &123)
// Returns: "/actions/todos/123"
```

**4. Authentication Flow**
```
Public routes   → Always accessible
                  ↓
Protected routes → require_authentication middleware
                  → Checks CurrentUser extension
                  → Redirect to sign-in if guest
                  → Allow through if authenticated
```

### Design Principles

- **Single Standard**: Same case handled the same way everywhere
- **Explicit Over Implicit**: Required values fail fast with clear errors, no silent defaults
- **No Path Hardcoding**: All URLs defined in `src/paths.rs`
- **Type-First Organization**: Group by interaction type, not resource
- **Durable Comments**: Document architecture and safety, not implementation details
