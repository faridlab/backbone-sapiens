-- Migration: create sapiens.integration_credentials (the credential store)
--
-- Minimal ADR-0024 surface: envelope-encrypted secrets scoped to
-- (company, provider, account, purpose) with an explicit lifecycle. The secret
-- material exists only in `ciphertext` (base64 nonce‖ct‖tag under AES-256-GCM);
-- the HTTP surface is verbs only and returns metadata, never the secret.
--
-- Enum types are created unqualified so they land in public beside the module's
-- other enum types, where the generated sqlx type_name resolves. The company
-- fence is strict (ADR-0014): an unset app.company_id sees zero rows.

DO $$ BEGIN CREATE TYPE credential_purpose AS ENUM ('webhook_verify', 'api_read', 'api_write', 'oauth_token'); EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE TYPE credential_status AS ENUM ('active', 'expired', 'revoked'); EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS sapiens.integration_credentials (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    company_id UUID NOT NULL,
    provider TEXT NOT NULL,
    account_ref TEXT NOT NULL,
    purpose credential_purpose NOT NULL,
    key_id TEXT NOT NULL,
    ciphertext TEXT NOT NULL,
    status credential_status NOT NULL DEFAULT 'active',
    expires_at TIMESTAMPTZ,
    rotated_from UUID,
    last_used_at TIMESTAMPTZ,
    metadata JSONB NOT NULL DEFAULT '{"created_at":null,"updated_at":null,"deleted_at":null,"created_by":null,"updated_by":null,"deleted_by":null}'::jsonb,
    PRIMARY KEY (id),
    CONSTRAINT fk_integration_credentials_rotated_from FOREIGN KEY (rotated_from) REFERENCES sapiens.integration_credentials (id) ON DELETE SET NULL ON UPDATE CASCADE
);

-- At most one active credential per scope+purpose; expired/revoked lineage rows
-- are exempt (rotation history keeps its unique ids).
CREATE UNIQUE INDEX IF NOT EXISTS uq_integration_credentials_active_scope
    ON sapiens.integration_credentials (company_id, provider, account_ref, purpose)
    WHERE status = 'active';

CREATE INDEX IF NOT EXISTS idx_integration_credentials_scope
    ON sapiens.integration_credentials (company_id, provider, account_ref);

CREATE INDEX IF NOT EXISTS idx_integration_credentials_company_status
    ON sapiens.integration_credentials (company_id, status);

CREATE INDEX IF NOT EXISTS idx_integration_credentials_expires_at
    ON sapiens.integration_credentials (expires_at)
    WHERE status = 'active';

-- Audit metadata timestamps, same trigger shape as every other sapiens table.
CREATE OR REPLACE FUNCTION sapiens.integration_credentials_audit_timestamp() RETURNS trigger AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        NEW.metadata = jsonb_set(NEW.metadata::jsonb, '{created_at}', to_jsonb(NOW()));
        NEW.metadata = jsonb_set(NEW.metadata::jsonb, '{updated_at}', to_jsonb(NOW()));
    ELSIF TG_OP = 'UPDATE' THEN
        NEW.metadata = jsonb_set(NEW.metadata::jsonb, '{updated_at}', to_jsonb(NOW()));
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS integration_credentials_insert_audit ON sapiens.integration_credentials;
CREATE TRIGGER integration_credentials_insert_audit BEFORE INSERT ON sapiens.integration_credentials
    FOR EACH ROW EXECUTE FUNCTION sapiens.integration_credentials_audit_timestamp();

DROP TRIGGER IF EXISTS integration_credentials_update_audit ON sapiens.integration_credentials;
CREATE TRIGGER integration_credentials_update_audit BEFORE UPDATE ON sapiens.integration_credentials
    FOR EACH ROW EXECUTE FUNCTION sapiens.integration_credentials_audit_timestamp();

-- Company fence (strict): a session sees only rows whose company_id equals the
-- request-scoped company (set_config('app.company_id', <uuid>, true)); an unset
-- var sees zero rows (fail-closed).
ALTER TABLE sapiens.integration_credentials ENABLE ROW LEVEL SECURITY;
ALTER TABLE sapiens.integration_credentials FORCE  ROW LEVEL SECURITY;
DROP POLICY IF EXISTS integration_credentials_company_isolation ON sapiens.integration_credentials;
CREATE POLICY integration_credentials_company_isolation ON sapiens.integration_credentials
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
