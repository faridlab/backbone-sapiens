-- Down: drop user_settings table
DROP TABLE IF EXISTS user_settings CASCADE;
DROP FUNCTION IF EXISTS user_settings_audit_timestamp() CASCADE;
