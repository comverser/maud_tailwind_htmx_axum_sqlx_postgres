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

- Use HTMX to enable RESTful patterns (DELETE, PATCH, PUT) from HTML forms
- Progressive enhancement: forms should work with or without HTMX when practical
- Use HTMX for improved UX: partial page updates, optimistic UI, etc.

## Code Style

- Follow modern Rust conventions
- **Single standard principle**: Maintain exactly one way to accomplish each task - avoid creating multiple functions or patterns for the same purpose
- Minimize symbol visibility: prefer private unless public is required
- **Don't care about backward compatibility**: This is a template in active development - breaking changes are acceptable for improvements

## Routing & API Design

- **Type-first organization**: Routes organized by interaction type (`/forms/*`, `/actions/*`, `/pages/*`)
- **RESTful within type**: Use proper HTTP methods (DELETE, PATCH, PUT) and path parameters for resources
- **Path parameters for resource identification**: `/forms/todos/{todo_id}` instead of `todo_id` in body

### Route Structure Examples

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

## Naming

- Improve naming for files, functions, variables, and identifiers
- Use `snake_case` for URLs (with underscores)
- HTTP handler names follow the pattern: `method_type_resource_[param]_[action]`
  - Examples shown in routing section above
- Module structure mirrors route type structure
  - Form handlers in `handlers/forms/`
  - Action handlers in `handlers/actions/`
  - Page handlers in `handlers/pages/`

## Path Management

- **No hardcoded paths** - All paths must be defined in the `paths` module
- **Single source of truth** - All URL structure lives in `src/paths.rs`
