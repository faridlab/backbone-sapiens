-- Down: drop impersonation_sessions table
DROP TABLE IF EXISTS impersonation_sessions CASCADE;
DROP FUNCTION IF EXISTS impersonation_sessions_audit_timestamp() CASCADE;
