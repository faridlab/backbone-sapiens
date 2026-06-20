-- Down: drop sapiens.effective_permission_cache table
DROP TABLE IF EXISTS sapiens.effective_permission_cache CASCADE;
DROP FUNCTION IF EXISTS sapiens.effective_permission_cache_audit_timestamp() CASCADE;
