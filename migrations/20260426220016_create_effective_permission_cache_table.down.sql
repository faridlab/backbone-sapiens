-- Down: drop effective_permission_cache table
DROP TABLE IF EXISTS effective_permission_cache CASCADE;
DROP FUNCTION IF EXISTS effective_permission_cache_audit_timestamp() CASCADE;
