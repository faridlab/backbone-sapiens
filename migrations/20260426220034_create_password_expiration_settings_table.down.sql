-- Down: drop password_expiration_settings table
DROP TABLE IF EXISTS password_expiration_settings CASCADE;
DROP FUNCTION IF EXISTS password_expiration_settings_audit_timestamp() CASCADE;
