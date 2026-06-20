-- Down: drop sapiens.device_trusts table
DROP TABLE IF EXISTS sapiens.device_trusts CASCADE;
DROP FUNCTION IF EXISTS sapiens.device_trusts_audit_timestamp() CASCADE;
