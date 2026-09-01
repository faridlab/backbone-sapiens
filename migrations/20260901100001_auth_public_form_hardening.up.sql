-- Hardening surfaces for the public auth forms.
--
-- Hand-authored (not schema-derived). Three tables:
--
--   auth_policy          — the signup kill-switch. A missing row means the most
--                          restrictive posture (signup OFF); nothing is inserted
--                          at install time, so a fresh deployment starts closed.
--   auth_throttle_buckets— durable fixed-window rate-limit counters for the
--                          public auth forms, keyed by (scope, key, window).
--                          Scope distinguishes per-identity and per-IP buckets so
--                          both limits are enforced independently and survive
--                          process restarts (no in-process limiter state).
--   device_trust_keys    — scoped, expiring, audit-trailed trusted-device keys.
--                          Revocation is a row update (revoked_at), so password
--                          changes can revoke every outstanding key for a user
--                          in one statement.

CREATE SCHEMA IF NOT EXISTS sapiens;

CREATE TABLE IF NOT EXISTS sapiens.auth_policy (
    key TEXT PRIMARY KEY,
    value JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Revocation list for invitation credentials. A row here permanently bars the
-- credential even if the token itself still verifies (archive/delete of the
-- invitee, explicit invite kill).
CREATE TABLE IF NOT EXISTS sapiens.auth_signup_revocations (
    credential_id TEXT PRIMARY KEY,
    revoked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reason TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sapiens.auth_throttle_buckets (
    scope TEXT NOT NULL,
    bucket_key TEXT NOT NULL,
    window_start TIMESTAMPTZ NOT NULL,
    count BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (scope, bucket_key, window_start)
);

CREATE INDEX IF NOT EXISTS idx_auth_throttle_buckets_window
    ON sapiens.auth_throttle_buckets (window_start);

CREATE TABLE IF NOT EXISTS sapiens.device_trust_keys (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    key_hash TEXT NOT NULL,
    scope TEXT NOT NULL,
    device_fingerprint TEXT,
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    last_used_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    issued_ip TEXT,
    metadata JSONB NOT NULL DEFAULT '{"created_at":null,"updated_at":null,"deleted_at":null,"created_by":null,"updated_by":null,"deleted_by":null}'::jsonb,
    PRIMARY KEY (id)
);

CREATE INDEX IF NOT EXISTS idx_device_trust_keys_user
    ON sapiens.device_trust_keys (user_id) WHERE revoked_at IS NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_device_trust_keys_key_hash
    ON sapiens.device_trust_keys (key_hash);
