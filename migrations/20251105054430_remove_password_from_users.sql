-- Remove password_hash column as we're switching to magic link authentication
ALTER TABLE users DROP COLUMN IF EXISTS password_hash;
