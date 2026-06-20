-- Down: drop user_roles table
DROP TABLE IF EXISTS user_roles CASCADE;
DROP FUNCTION IF EXISTS user_roles_audit_timestamp() CASCADE;
