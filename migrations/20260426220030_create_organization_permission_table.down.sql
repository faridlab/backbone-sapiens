-- Down: drop sapiens.organization_permissions table
DROP TABLE IF EXISTS sapiens.organization_permissions CASCADE;
DROP FUNCTION IF EXISTS sapiens.organization_permissions_audit_timestamp() CASCADE;
