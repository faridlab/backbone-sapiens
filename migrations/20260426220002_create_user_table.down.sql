-- Down: drop users table
DROP TABLE IF EXISTS users CASCADE;
DROP FUNCTION IF EXISTS users_audit_timestamp() CASCADE;
