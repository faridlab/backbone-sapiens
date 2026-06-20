-- Down: drop mfa_sessions table
DROP TABLE IF EXISTS mfa_sessions CASCADE;
DROP FUNCTION IF EXISTS mfa_sessions_audit_timestamp() CASCADE;
