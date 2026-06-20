-- Down: drop mfa_backup_codes table
DROP TABLE IF EXISTS mfa_backup_codes CASCADE;
DROP FUNCTION IF EXISTS mfa_backup_codes_audit_timestamp() CASCADE;
