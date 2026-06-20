-- Down: drop mfa_devices table
DROP TABLE IF EXISTS mfa_devices CASCADE;
DROP FUNCTION IF EXISTS mfa_devices_audit_timestamp() CASCADE;
