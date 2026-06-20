-- Down: drop sapiens.sessions table
DROP TABLE IF EXISTS sapiens.sessions CASCADE;
DROP FUNCTION IF EXISTS sapiens.sessions_audit_timestamp() CASCADE;
