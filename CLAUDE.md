# Guidelines for Claude Code

## Project Purpose

This is a **template repository** for web applications. Keep all views and UI minimal and generic to serve as a starting point for various projects.

## Tech Stack

- **Backend**: Axum (Rust web framework)
- **Database**: PostgreSQL with SQLx
- **Templates**: Maud (compile-time HTML templates)
- **Frontend**: HTMX + Tailwind CSS
- **Sessions**: tower-sessions with PostgreSQL store

### HTMX Usage Policy

- Prefer standard HTML forms and links when possible
- Use HTMX only when:
  - Standard HTML cannot accomplish the task
  - HTMX provides significantly better UX
  - Action requires non-standard HTTP methods on non-form elements

## Code Style

- Follow modern Rust conventions
- **Single standard principle**: Maintain exactly one way to accomplish each task - avoid creating multiple functions or patterns for the same purpose
- Minimize symbol visibility: prefer private unless public is required
- **Don't care about backward compatibility**: This is a template in active development - breaking changes are acceptable for improvements

## Naming

- Improve naming for files, functions, variables, and identifiers
- Use `snake_case` for URLs (with underscores)
- HTTP handler names follow the pattern: `method_endpoint`
  - Endpoint derived from route path: static segments as-is, path params as `{resource}_id`, joined with underscores
  - Examples:
    - `/forms/sign_in` → `post_forms_sign_in`
    - `/actions/sign_out` → `post_actions_sign_out`
    - `/todos` → `get_todos`
- Module structure mirrors route structure
  - Form handlers in `handlers/forms/` for `/forms/*` routes
  - Action handlers in `handlers/actions/` for `/actions/*` routes
  - Page handlers in `handlers/pages/` for page routes
