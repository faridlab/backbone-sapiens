-- Down: drop backup_codes table
DROP TABLE IF EXISTS backup_codes CASCADE;
DROP FUNCTION IF EXISTS backup_codes_audit_timestamp() CASCADE;
