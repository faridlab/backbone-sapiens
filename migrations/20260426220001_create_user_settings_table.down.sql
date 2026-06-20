-- Down: drop sapiens.user_settings table
DROP TABLE IF EXISTS sapiens.user_settings CASCADE;
DROP FUNCTION IF EXISTS sapiens.user_settings_audit_timestamp() CASCADE;
