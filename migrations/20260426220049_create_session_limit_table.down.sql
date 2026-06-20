-- Down: drop session_limits table
DROP TABLE IF EXISTS session_limits CASCADE;
DROP FUNCTION IF EXISTS session_limits_audit_timestamp() CASCADE;
