-- Hand-authored (user-owned). Not regenerated.
--
-- Re-key the sapiens org-membership axis from `organization_id` to `org_unit_id`
-- (ADR-0028/0029). The values were always organization.org_units node ids — the
-- tenant spine mirrored the legacy company ids into org_units verbatim — so the
-- membership tables need a pure rename: no value changes, no constraint changes.
-- Two shared-row tables (password_policies, analytics_metrics) additionally
-- root-anchor their legacy NULL-tenant-global rows and gain NOT NULL (ADR-0028:
-- shared rows anchor on the root node, never on a NULL key). Indexes are renamed
-- to match the new column name.
--
-- Guarded per table on column existence so the file is re-runnable and safe on a
-- database whose chain stopped before these tables were created.

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'sapiens' AND table_name = 'organization_users' AND column_name = 'organization_id'
    ) THEN
        ALTER TABLE sapiens.organization_users RENAME COLUMN organization_id TO org_unit_id;
        ALTER INDEX IF EXISTS sapiens.idx_organization_users_organization_id
            RENAME TO idx_organization_users_org_unit_id;
        ALTER INDEX IF EXISTS sapiens.idx_organization_users_organization_id_user_id
            RENAME TO idx_organization_users_org_unit_id_user_id;
    END IF;

    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'sapiens' AND table_name = 'organization_roles' AND column_name = 'organization_id'
    ) THEN
        ALTER TABLE sapiens.organization_roles RENAME COLUMN organization_id TO org_unit_id;
        ALTER INDEX IF EXISTS sapiens.idx_organization_roles_organization_id
            RENAME TO idx_organization_roles_org_unit_id;
        ALTER INDEX IF EXISTS sapiens.idx_organization_roles_organization_id_name
            RENAME TO idx_organization_roles_org_unit_id_name;
    END IF;

    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'sapiens' AND table_name = 'organization_permissions' AND column_name = 'organization_id'
    ) THEN
        ALTER TABLE sapiens.organization_permissions RENAME COLUMN organization_id TO org_unit_id;
        ALTER INDEX IF EXISTS sapiens.idx_organization_permissions_organization_id
            RENAME TO idx_organization_permissions_org_unit_id;
    END IF;

    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'sapiens' AND table_name = 'password_policies' AND column_name = 'organization_id'
    ) THEN
        ALTER TABLE sapiens.password_policies RENAME COLUMN organization_id TO org_unit_id;
        -- Unique index; a rename keeps the uniqueness (no rebuild).
        ALTER INDEX IF EXISTS sapiens.idx_password_policies_organization_id
            RENAME TO idx_password_policies_org_unit_id;
        -- Root-anchor the legacy tenant-global rows: NULL used to mean
        -- tenant-wide; the org fence law anchors shared rows on the root node
        -- (ADR-0028). On an empty table this is a no-op; if NULLs remain and
        -- the org spine is missing, SET NOT NULL raises — the honest failure.
        UPDATE sapiens.password_policies
           SET org_unit_id = (SELECT id FROM organization.org_units WHERE kind = 'root' LIMIT 1)
         WHERE org_unit_id IS NULL;
        ALTER TABLE sapiens.password_policies ALTER COLUMN org_unit_id SET NOT NULL;
    END IF;

    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'sapiens' AND table_name = 'analytics_metrics' AND column_name = 'organization_id'
    ) THEN
        ALTER TABLE sapiens.analytics_metrics RENAME COLUMN organization_id TO org_unit_id;
        ALTER INDEX IF EXISTS sapiens.idx_analytics_metrics_organization_id_period_start
            RENAME TO idx_analytics_metrics_org_unit_id_period_start;
        -- Root-anchor the legacy tenant-wide rows (same shape as above).
        UPDATE sapiens.analytics_metrics
           SET org_unit_id = (SELECT id FROM organization.org_units WHERE kind = 'root' LIMIT 1)
         WHERE org_unit_id IS NULL;
        ALTER TABLE sapiens.analytics_metrics ALTER COLUMN org_unit_id SET NOT NULL;
    END IF;
END $$;
