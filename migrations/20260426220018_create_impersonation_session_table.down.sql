-- Down: drop sapiens.impersonation_sessions table
DROP TABLE IF EXISTS sapiens.impersonation_sessions CASCADE;
DROP FUNCTION IF EXISTS sapiens.impersonation_sessions_audit_timestamp() CASCADE;
