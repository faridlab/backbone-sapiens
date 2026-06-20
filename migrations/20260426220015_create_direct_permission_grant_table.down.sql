-- Down: drop direct_permission_grants table
DROP TABLE IF EXISTS direct_permission_grants CASCADE;
DROP FUNCTION IF EXISTS direct_permission_grants_audit_timestamp() CASCADE;
