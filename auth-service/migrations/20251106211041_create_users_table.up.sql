-- Add up migration script her
CREATE TABLE IF NOT EXISTS users (
    email TEXT NOT NULL PRIMARY KEY,
    password_hash TEXT NOT NULL,
    requires_2fa BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);