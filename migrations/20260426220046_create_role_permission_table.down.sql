-- Down: drop role_permissions table
DROP TABLE IF EXISTS role_permissions CASCADE;
DROP FUNCTION IF EXISTS role_permissions_audit_timestamp() CASCADE;
