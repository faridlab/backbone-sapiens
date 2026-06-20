-- Down: drop sapiens.system_settings table
DROP TABLE IF EXISTS sapiens.system_settings CASCADE;
DROP FUNCTION IF EXISTS sapiens.system_settings_audit_timestamp() CASCADE;
