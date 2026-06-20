-- Down: drop sapiens.session_limits table
DROP TABLE IF EXISTS sapiens.session_limits CASCADE;
DROP FUNCTION IF EXISTS sapiens.session_limits_audit_timestamp() CASCADE;
