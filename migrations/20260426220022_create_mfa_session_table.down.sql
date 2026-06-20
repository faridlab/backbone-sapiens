-- Down: drop sapiens.mfa_sessions table
DROP TABLE IF EXISTS sapiens.mfa_sessions CASCADE;
DROP FUNCTION IF EXISTS sapiens.mfa_sessions_audit_timestamp() CASCADE;
