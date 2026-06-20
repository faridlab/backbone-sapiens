-- Down: drop audit_logs table
DROP TABLE IF EXISTS audit_logs CASCADE;
DROP FUNCTION IF EXISTS audit_logs_audit_timestamp() CASCADE;
