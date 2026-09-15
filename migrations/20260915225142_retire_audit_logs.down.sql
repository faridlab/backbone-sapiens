-- Recreate the retired shell, empty, so a rollback leaves a schema the old code
-- can start against. It was empty when it was dropped.
CREATE TYPE audit_log_severity AS ENUM ('info', 'warning', 'error', 'critical');

CREATE TABLE IF NOT EXISTS sapiens.audit_logs (
    id            uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id       uuid,
    action        text NOT NULL,
    details       jsonb,
    severity      audit_log_severity NOT NULL DEFAULT 'info',
    ip_address    text,
    resource_type text,
    resource_id   uuid,
    timestamp     timestamptz NOT NULL DEFAULT now()
);
