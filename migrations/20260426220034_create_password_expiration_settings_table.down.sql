-- Down: drop sapiens.password_expiration_settings table
DROP TABLE IF EXISTS sapiens.password_expiration_settings CASCADE;
DROP FUNCTION IF EXISTS sapiens.password_expiration_settings_audit_timestamp() CASCADE;
