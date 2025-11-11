set dotenv-load

default:
    @just --list

# Development - run with auto-reload and git pull
run:
    git pull --rebase --autostash
    RUST_LOG=debug cargo watch -c -x run

# Database - run migrations
migrate:
    sqlx migrate run

# Database - create new migration
migrate-add name:
    sqlx migrate add {{name}}

# Database - reset (revert all and re-run)
migrate-reset:
    sqlx migrate revert --target-version 0 || true
    sqlx migrate run

# Admin - grant admin role to user
admin-grant email:
    #!/usr/bin/env bash
    set -euo pipefail
    docker exec -i postgres_docker_container-postgres-1 psql -U postgres -d postgres -c "INSERT INTO user_roles (user_id, role) SELECT user_id, 'admin' FROM users WHERE email = '{{email}}' ON CONFLICT DO NOTHING RETURNING user_id;" \
    && echo "✓ Admin role granted to {{email}}" \
    || echo "✗ Failed. User may not exist."

# Admin - revoke admin role from user
admin-revoke email:
    #!/usr/bin/env bash
    set -euo pipefail
    docker exec -i postgres_docker_container-postgres-1 psql -U postgres -d postgres -c "DELETE FROM user_roles WHERE user_id = (SELECT user_id FROM users WHERE email = '{{email}}') AND role = 'admin' RETURNING user_id;" \
    && echo "✓ Admin role revoked from {{email}}" \
    || echo "✗ Failed."

# Admin - list all admin users
admin-list:
    @docker exec -i postgres_docker_container-postgres-1 psql -U postgres -d postgres -c "SELECT u.email, ur.granted_at, granter.email as granted_by FROM user_roles ur JOIN users u ON ur.user_id = u.user_id LEFT JOIN users granter ON ur.granted_by = granter.user_id WHERE ur.role = 'admin' ORDER BY ur.granted_at DESC;"

# Setup - install development tools
setup:
    cargo install sqlx-cli --no-default-features --features postgres
    cargo install cargo-watch
    @echo "\n✓ Setup complete. Next: copy .env.example to .env, then run 'just migrate'"