-- Down: drop sapiens.direct_permission_grants table
DROP TABLE IF EXISTS sapiens.direct_permission_grants CASCADE;
DROP FUNCTION IF EXISTS sapiens.direct_permission_grants_audit_timestamp() CASCADE;
