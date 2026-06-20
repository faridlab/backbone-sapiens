-- Down: drop device_trusts table
DROP TABLE IF EXISTS device_trusts CASCADE;
DROP FUNCTION IF EXISTS device_trusts_audit_timestamp() CASCADE;
