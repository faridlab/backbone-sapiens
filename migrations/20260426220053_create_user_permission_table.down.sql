-- Down: drop user_permissions table
DROP TABLE IF EXISTS user_permissions CASCADE;
DROP FUNCTION IF EXISTS user_permissions_audit_timestamp() CASCADE;
