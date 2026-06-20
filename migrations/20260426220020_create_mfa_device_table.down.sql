-- Down: drop sapiens.mfa_devices table
DROP TABLE IF EXISTS sapiens.mfa_devices CASCADE;
DROP FUNCTION IF EXISTS sapiens.mfa_devices_audit_timestamp() CASCADE;
