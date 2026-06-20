-- Down: drop password_history_settings table
DROP TABLE IF EXISTS password_history_settings CASCADE;
DROP FUNCTION IF EXISTS password_history_settings_audit_timestamp() CASCADE;
