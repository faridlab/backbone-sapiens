-- Down: drop temporary_permissions table
DROP TABLE IF EXISTS temporary_permissions CASCADE;
DROP FUNCTION IF EXISTS temporary_permissions_audit_timestamp() CASCADE;
