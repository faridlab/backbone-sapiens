-- Down: drop resource_permissions table
DROP TABLE IF EXISTS resource_permissions CASCADE;
DROP FUNCTION IF EXISTS resource_permissions_audit_timestamp() CASCADE;
