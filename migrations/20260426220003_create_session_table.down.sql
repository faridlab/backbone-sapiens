-- Down: drop sessions table
DROP TABLE IF EXISTS sessions CASCADE;
DROP FUNCTION IF EXISTS sessions_audit_timestamp() CASCADE;
