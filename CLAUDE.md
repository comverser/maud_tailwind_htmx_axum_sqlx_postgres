# Guidelines for Claude Code

## Project Overview

### Purpose
This is a **template repository** for web applications. Keep all views and UI minimal and generic to serve as a starting point for various projects.

### Tech Stack
- **Backend**: Axum (Rust web framework)
- **Database**: PostgreSQL with SQLx
- **Templates**: Maud (compile-time HTML templates)
- **Frontend**: HTMX + Tailwind CSS
- **Sessions**: tower-sessions with PostgreSQL store

---

## Core Principles

### Single Standard Principle
When handling the same case multiple times, use exactly one consistent approach.

- If you find yourself handling the same case in different ways, extract to a single function
- Example: If multiple places need to read env vars with validation, create one helper. Don't mix patterns.

### Be Explicit Over Implicit
Make everything clear and fail fast with explicit errors.

- **Make everything required** - don't create optional/default patterns
- **Don't handle "if not set" cases** - if something is needed, require it with `.expect()`
- **Fail fast** with explicit errors at startup rather than silent defaults
- **Use simpler constructs**: `unwrap_or(value)` not `unwrap_or_else(|| value)` when value is already available
- **No magic values** - use constants for values that appear in multiple places
- **Clear error messages** - `.expect("VARIABLE_NAME must be set")` not just `.expect()`
- If it can be wrong, make it impossible to ignore - panic early with clear messages

### Keep Code Simple
Don't abstract until duplication appears.

- Wait for the second use before extracting
- Inline is fine for single use - don't premature optimize
- Extract when you find duplication, not before

### Other Principles
- Follow modern Rust conventions
- Minimize symbol visibility: prefer private unless public is required
- **Don't care about backward compatibility**: This is a template in active development - breaking changes are acceptable for improvements

---

## Architecture & Organization

### Routing & API Design
**Type-first organization**: Routes organized by interaction type (`/forms/*`, `/actions/*`, `/pages/*`)

- **RESTful within type**: Use proper HTTP methods (DELETE, PATCH, PUT) and path parameters for resources
- **Path parameters for resource identification**: `/forms/todos/{todo_id}` instead of `todo_id` in body

**Route Structure Examples:**
```http
Pages (GET only):
  GET /              → get_root
  GET /todos         → get_todos
  GET /sign_in       → get_sign_in

Forms (with form data):
  POST   /forms/sign_in                    → post_forms_sign_in
  POST   /forms/todos                      → post_forms_todos
  POST   /forms/todos/{todo_id}/toggle     → post_forms_todos_todo_id_toggle

Actions (state changes, typically no form data):
  POST   /actions/sign_out                 → post_actions_sign_out
  DELETE /actions/todos/{todo_id}          → delete_actions_todos_todo_id
```

### Path Management
- **No hardcoded paths** - All paths must be defined in the `paths` module
- **Single source of truth** - All URL structure lives in `src/paths.rs`

### Naming Conventions
- Improve naming for files, functions, variables, and identifiers
- Use `snake_case` for URLs (with underscores)
- HTTP handler names follow the pattern: `method_type_resource_[param]_[action]`
  - Examples: `post_forms_sign_in`, `delete_actions_todos_todo_id`
- Module structure mirrors route type structure:
  - Form handlers in `handlers/forms/`
  - Action handlers in `handlers/actions/`
  - Page handlers in `handlers/pages/`

### Configuration Management
- **All configuration is required** - use `.expect()` with clear error messages
- **No defaults for environment variables** - if it's needed, it must be set explicitly
- **Fail fast at startup** - panic immediately if required configuration is missing
- Store only true constants in `src/constants.rs` (values used in multiple places, not env var defaults)
- Example: `MAGIC_LINK_EXPIRY_MINUTES` is a constant; `SITE_NAME` is required config, not a default

---

## Technology-Specific Guidelines

### HTMX Usage
- Use HTMX to enable RESTful patterns (DELETE, PATCH, PUT) from HTML forms
- Progressive enhancement: forms should work with or without HTMX when practical
- Use HTMX for improved UX: partial page updates, optimistic UI, etc.
