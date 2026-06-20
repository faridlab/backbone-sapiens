-- Down: drop organization_permissions table
DROP TABLE IF EXISTS organization_permissions CASCADE;
DROP FUNCTION IF EXISTS organization_permissions_audit_timestamp() CASCADE;
