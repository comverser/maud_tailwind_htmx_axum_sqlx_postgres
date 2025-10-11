# Web App Template

A minimal web application template built with Rust.

## Tech Stack

- **Backend**: Axum
- **Database**: PostgreSQL with SQLx
- **Templates**: Maud
- **Frontend**: HTMX + Tailwind CSS
- **Sessions**: tower-sessions with PostgreSQL store

## Getting Started

1. Set up PostgreSQL database
2. Configure environment variables
3. Run migrations
4. Start the server

```bash
just
```

## Project Structure

- Type-first routing (`/pages/*`, `/forms/*`, `/actions/*`)
- RESTful API design with proper HTTP methods
- Single source of truth for paths in `src/paths.rs`
