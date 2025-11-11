-- Enable citext extension for case-insensitive text
CREATE EXTENSION IF NOT EXISTS citext;

-- ============================================================================
-- Users Table
-- ============================================================================
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    email citext UNIQUE NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

-- Trigger function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply trigger to users table
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Magic Links Table
-- ============================================================================
CREATE TABLE magic_links (
    token TEXT PRIMARY KEY,
    email citext NOT NULL,
    expires_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_magic_links_email ON magic_links(email);
CREATE INDEX idx_magic_links_expires_at ON magic_links(expires_at);

-- ============================================================================
-- Todos Table
-- ============================================================================
CREATE TABLE todos (
    todo_id SERIAL PRIMARY KEY,
    task text NOT NULL,
    is_done boolean NOT NULL DEFAULT false,
    created_at timestamptz NOT NULL DEFAULT now(),
    author_id integer NOT NULL REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE INDEX idx_todos_author_id ON todos(author_id);
CREATE INDEX idx_todos_created_at ON todos(created_at DESC);

-- ============================================================================
-- Orders Table
-- ============================================================================
CREATE TABLE orders (
    order_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id INTEGER NOT NULL REFERENCES users(user_id),
    user_email CITEXT NOT NULL,
    filename TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    text_content TEXT NOT NULL,
    text_length INTEGER NOT NULL,
    price_amount INTEGER NOT NULL,
    payment_status TEXT NOT NULL CHECK (payment_status IN ('pending', 'paid', 'failed', 'cancelled')),
    payment_key TEXT,
    order_number TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    paid_at TIMESTAMPTZ
);

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_user_email ON orders(user_email);
CREATE INDEX idx_orders_order_number ON orders(order_number);
CREATE INDEX idx_orders_payment_status ON orders(payment_status);

-- ============================================================================
-- User Roles Table (Admin System)
-- ============================================================================
CREATE TABLE user_roles (
    user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    granted_by INTEGER REFERENCES users(user_id),
    PRIMARY KEY (user_id, role)
);

CREATE INDEX idx_user_roles_user_id ON user_roles(user_id);
CREATE INDEX idx_user_roles_role ON user_roles(role);
