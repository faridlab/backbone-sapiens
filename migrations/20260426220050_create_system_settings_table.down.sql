-- Down: drop system_settings table
DROP TABLE IF EXISTS system_settings CASCADE;
DROP FUNCTION IF EXISTS system_settings_audit_timestamp() CASCADE;
