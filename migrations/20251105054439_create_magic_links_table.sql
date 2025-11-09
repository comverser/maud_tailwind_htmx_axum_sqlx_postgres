-- Create magic_links table for passwordless authentication
CREATE TABLE magic_links (
    token TEXT PRIMARY KEY,
    email citext NOT NULL,
    expires_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

-- Index for faster email lookups and cleanup
CREATE INDEX idx_magic_links_email ON magic_links(email);
CREATE INDEX idx_magic_links_expires_at ON magic_links(expires_at);
