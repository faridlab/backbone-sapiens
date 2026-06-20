-- Down: drop sapiens.backup_codes table
DROP TABLE IF EXISTS sapiens.backup_codes CASCADE;
DROP FUNCTION IF EXISTS sapiens.backup_codes_audit_timestamp() CASCADE;
