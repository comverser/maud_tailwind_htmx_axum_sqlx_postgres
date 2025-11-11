# Guidelines for Claude Code

## Overview

This is a **template repository** for web applications. Keep everything minimal and generic.

**Tech Stack:** Axum • PostgreSQL + SQLx • Maud • HTMX + Tailwind CSS • tower-sessions

---

## Important: Modifying This File

**Do not modify CLAUDE.md unless explicitly ordered by the user.**

This file contains the core development guidelines and should only be updated when the user directly requests changes to these guidelines.

---

## Code Principles

### Single Standard
One consistent approach for each case.
- Extract to single function if handling same case differently
- No mixed patterns

### Explicit Over Implicit
Fail fast with clear errors.
- Make everything required, no optional/default patterns
- Use `.expect("VARIABLE_NAME must be set")` for required values
- Constants for repeated values, no magic numbers
- Panic early at startup, not silently during runtime

### Keep It Simple
Don't abstract until duplication appears.
- Wait for second use before extracting
- Inline is fine for single use

### Comments
Only durable comments that won't become stale.
- Document "why" and architecture, not "what"
- Examples: middleware ordering, safety assumptions

### General
- Follow modern Rust conventions
- Minimize symbol visibility (private by default)
- Breaking changes acceptable - this is a template

---

## UI Principles

Minimal and functional - no decoration.
- No shadows, rounded corners, font weights, or decorative backgrounds
- Simple borders, single primary color
- Hover for interaction, border change for focus
- Tight spacing, maximum density

---

## Architecture

### Type-First Routing
Routes organized by interaction type: `/pages/*`, `/forms/*`, `/actions/*`

```http
GET /              → get_root             (pages - render HTML)
POST /forms/todos  → post_forms_todos    (forms - submit data)
DELETE /actions/todos/{id} → delete_...  (actions - state changes)
```

- Use RESTful methods (GET, POST, DELETE, PATCH, PUT)
- Path parameters for resource IDs: `/todos/{todo_id}` not in body
- Handler names: `method_type_resource_[param]_[action]`

### Paths
- No hardcoded paths - define all in `src/paths.rs`

### Naming
- URLs: `snake_case`
- Handlers: `post_forms_sign_in`, `delete_actions_todos_todo_id`
- Module structure mirrors route types: `handlers/forms/`, `handlers/actions/`, `handlers/pages/`

### Configuration
- All config required - `.expect()` with clear messages
- No env var defaults - fail fast at startup
- True constants in `src/constants.rs`

---

## Technology Usage

### HTMX
- Enable RESTful patterns (DELETE, PATCH, PUT) from HTML
- Progressive enhancement when practical
- Partial updates, optimistic UI
