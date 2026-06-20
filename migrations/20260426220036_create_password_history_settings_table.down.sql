-- Down: drop sapiens.password_history_settings table
DROP TABLE IF EXISTS sapiens.password_history_settings CASCADE;
DROP FUNCTION IF EXISTS sapiens.password_history_settings_audit_timestamp() CASCADE;
