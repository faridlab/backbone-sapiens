-- Hand-authored (user-owned). Not regenerated.
--
-- Strip every company-fence artifact from sapiens.integration_credentials (ADR-0029): the
-- credential store is tenant-agnostic; org scoping is installed by the COMPOSING service's
-- tenancy decorator, never by the module. Dropped here: the company-leading indexes and
-- uniques (both the hand-authored names and the generator-emitted duplicates), the
-- integration_credentials_company_isolation RLS policy, and the company_id column itself.
--
-- The tenant-free domain invariant is re-declared immediately: at most one ACTIVE credential
-- per (provider, account_ref, purpose) per database. A provider account reference is
-- tenant-level configuration, so the per-scope unique needs no tenant column; a composing
-- service decorating this table re-creates the per-unit form (org_unit_id leading) at
-- composition time.
--
-- Ordering guard (the decorator must run FIRST on any database with data): the module
-- never moves tenancy data. The table is safe to strip when EITHER
--   a) it carries org_unit_id with no NULLs — the decorator backfilled it from company_id —
--      or b) it is empty (a fresh database: the earlier chain files created it empty).
-- Otherwise the strip RAISEs, naming the decorator step, rather than dropping a column
-- that still holds the only tenancy key. The file is re-runnable (every drop is IF EXISTS
-- and the tracker has no checksums), so a failed run retries cleanly after the decorator
-- lands.

DO $$
DECLARE
    has_org boolean;
    org_nulls bigint;
    total bigint;
BEGIN
    IF to_regclass('sapiens.integration_credentials') IS NULL THEN
        RETURN; -- chain not fully applied on this database; nothing to strip
    END IF;

    SELECT EXISTS (
               SELECT 1 FROM information_schema.columns
               WHERE table_schema = 'sapiens' AND table_name = 'integration_credentials' AND column_name = 'company_id'
           )
    INTO has_org;

    IF NOT has_org THEN
        RETURN; -- already stripped (or created without the column): nothing to do
    END IF;

    SELECT EXISTS (
               SELECT 1 FROM information_schema.columns
               WHERE table_schema = 'sapiens' AND table_name = 'integration_credentials' AND column_name = 'org_unit_id'
           )
    INTO has_org;

    EXECUTE 'SELECT count(*) FROM sapiens.integration_credentials' INTO total;

    IF has_org THEN
        EXECUTE 'SELECT count(*) FROM sapiens.integration_credentials WHERE org_unit_id IS NULL'
        INTO org_nulls;
    ELSE
        org_nulls := total; -- no org column: every row's only tenancy key is company_id
    END IF;

    IF NOT (has_org AND org_nulls = 0) AND total > 0 THEN
        RAISE EXCEPTION 'refusing to strip company_id — sapiens.integration_credentials holds % rows not covered by org_unit_id. Apply the composing service''s tenancy decorator (it backfills org_unit_id from company_id) and re-run; it is the only step that moves tenancy data.', org_nulls;
    END IF;
END $$;

DROP INDEX IF EXISTS sapiens.uq_integration_credentials_active_scope;
DROP INDEX IF EXISTS sapiens.idx_integration_credentials_company_id_provider_account_ref_purpose;
DROP INDEX IF EXISTS sapiens.idx_integration_credentials_scope;
DROP INDEX IF EXISTS sapiens.idx_integration_credentials_company_id_provider_account_ref;
DROP INDEX IF EXISTS sapiens.idx_integration_credentials_company_status;
DROP INDEX IF EXISTS sapiens.idx_integration_credentials_company_id_status;
DROP POLICY IF EXISTS integration_credentials_company_isolation ON sapiens.integration_credentials;
ALTER TABLE sapiens.integration_credentials DROP COLUMN IF EXISTS company_id;

-- ── Restore the tenant-free domain invariant ─────────────────────────────────
-- One active credential per provider account + purpose is a DOMAIN invariant, not a
-- tenancy posture: the provider account reference is tenant-level configuration, so the
-- per-scope unique needs no tenant column. The unique keeps its established name in its
-- tenant-free form; the per-unit form is owned by the composing service's tenancy
-- decorator and is intentionally NOT declared here (the global form below already holds
-- within one tenant database, and a decorated deployment adds the org-leading twin at
-- composition time).
CREATE UNIQUE INDEX IF NOT EXISTS uq_integration_credentials_active_scope
    ON sapiens.integration_credentials (provider, account_ref, purpose) WHERE status = 'active';

CREATE INDEX IF NOT EXISTS idx_integration_credentials_scope
    ON sapiens.integration_credentials (provider, account_ref);
