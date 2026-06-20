-- Down: drop sapiens.temporary_permissions table
DROP TABLE IF EXISTS sapiens.temporary_permissions CASCADE;
DROP FUNCTION IF EXISTS sapiens.temporary_permissions_audit_timestamp() CASCADE;
