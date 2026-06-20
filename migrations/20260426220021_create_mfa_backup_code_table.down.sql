-- Down: drop sapiens.mfa_backup_codes table
DROP TABLE IF EXISTS sapiens.mfa_backup_codes CASCADE;
DROP FUNCTION IF EXISTS sapiens.mfa_backup_codes_audit_timestamp() CASCADE;
