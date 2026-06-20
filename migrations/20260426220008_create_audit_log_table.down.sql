-- Down: drop sapiens.audit_logs table
DROP TABLE IF EXISTS sapiens.audit_logs CASCADE;
DROP FUNCTION IF EXISTS sapiens.audit_logs_audit_timestamp() CASCADE;
