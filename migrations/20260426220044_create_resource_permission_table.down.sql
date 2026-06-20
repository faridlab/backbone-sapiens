-- Down: drop sapiens.resource_permissions table
DROP TABLE IF EXISTS sapiens.resource_permissions CASCADE;
DROP FUNCTION IF EXISTS sapiens.resource_permissions_audit_timestamp() CASCADE;
