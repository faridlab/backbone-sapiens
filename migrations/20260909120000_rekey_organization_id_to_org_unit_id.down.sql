-- Reverse of the org-membership re-key: `org_unit_id` back to `organization_id`.
-- Same per-table column guard so the file is re-runnable and no-ops on a
-- database that never applied the up direction (or already rolled it back).

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'sapiens' AND table_name = 'organization_users' AND column_name = 'org_unit_id'
    ) THEN
        ALTER TABLE sapiens.organization_users RENAME COLUMN org_unit_id TO organization_id;
        ALTER INDEX IF EXISTS sapiens.idx_organization_users_org_unit_id
            RENAME TO idx_organization_users_organization_id;
        ALTER INDEX IF EXISTS sapiens.idx_organization_users_org_unit_id_user_id
            RENAME TO idx_organization_users_organization_id_user_id;
    END IF;

    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'sapiens' AND table_name = 'organization_roles' AND column_name = 'org_unit_id'
    ) THEN
        ALTER TABLE sapiens.organization_roles RENAME COLUMN org_unit_id TO organization_id;
        ALTER INDEX IF EXISTS sapiens.idx_organization_roles_org_unit_id
            RENAME TO idx_organization_roles_organization_id;
        ALTER INDEX IF EXISTS sapiens.idx_organization_roles_org_unit_id_name
            RENAME TO idx_organization_roles_organization_id_name;
    END IF;

    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'sapiens' AND table_name = 'organization_permissions' AND column_name = 'org_unit_id'
    ) THEN
        ALTER TABLE sapiens.organization_permissions RENAME COLUMN org_unit_id TO organization_id;
        ALTER INDEX IF EXISTS sapiens.idx_organization_permissions_org_unit_id
            RENAME TO idx_organization_permissions_organization_id;
    END IF;

    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'sapiens' AND table_name = 'password_policies' AND column_name = 'org_unit_id'
    ) THEN
        ALTER TABLE sapiens.password_policies ALTER COLUMN org_unit_id DROP NOT NULL;
        ALTER TABLE sapiens.password_policies RENAME COLUMN org_unit_id TO organization_id;
        -- Restore the legacy NULL encoding of tenant-global rows.
        UPDATE sapiens.password_policies
           SET organization_id = NULL
         WHERE organization_id = (SELECT id FROM organization.org_units WHERE kind = 'root' LIMIT 1);
        ALTER INDEX IF EXISTS sapiens.idx_password_policies_org_unit_id
            RENAME TO idx_password_policies_organization_id;
    END IF;

    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'sapiens' AND table_name = 'analytics_metrics' AND column_name = 'org_unit_id'
    ) THEN
        ALTER TABLE sapiens.analytics_metrics ALTER COLUMN org_unit_id DROP NOT NULL;
        ALTER TABLE sapiens.analytics_metrics RENAME COLUMN org_unit_id TO organization_id;
        UPDATE sapiens.analytics_metrics
           SET organization_id = NULL
         WHERE organization_id = (SELECT id FROM organization.org_units WHERE kind = 'root' LIMIT 1);
        ALTER INDEX IF EXISTS sapiens.idx_analytics_metrics_org_unit_id_period_start
            RENAME TO idx_analytics_metrics_organization_id_period_start;
    END IF;
END $$;
