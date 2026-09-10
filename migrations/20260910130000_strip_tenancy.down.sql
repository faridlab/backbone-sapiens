-- Hand-authored (user-owned). Not regenerated.
--
-- Best-effort restore sketch for the tenancy strip (ADR-0029). This is a breaking module
-- release against dev-stage databases: the down re-adds the company_id column as nullable
-- with the company-leading indexes and the company isolation policy shape, but restores NO
-- data — rows written after the strip (or after the decorator re-keyed them) carry
-- org_unit_id only. The composing service's tenancy decorator remains the live fence;
-- treat this down as a schema-shape sketch for archaeology, not a usable rollback.

ALTER TABLE sapiens.integration_credentials ADD COLUMN IF NOT EXISTS company_id uuid;

DROP INDEX IF EXISTS sapiens.uq_integration_credentials_active_scope;
DROP INDEX IF EXISTS sapiens.idx_integration_credentials_scope;
CREATE UNIQUE INDEX IF NOT EXISTS uq_integration_credentials_active_scope
    ON sapiens.integration_credentials (company_id, provider, account_ref, purpose) WHERE status = 'active';
CREATE INDEX IF NOT EXISTS idx_integration_credentials_scope
    ON sapiens.integration_credentials (company_id, provider, account_ref);
CREATE INDEX IF NOT EXISTS idx_integration_credentials_company_status
    ON sapiens.integration_credentials (company_id, status);

CREATE POLICY integration_credentials_company_isolation ON sapiens.integration_credentials
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
