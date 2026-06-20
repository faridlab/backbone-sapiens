-- Down: drop sapiens.notification_logs table
DROP TABLE IF EXISTS sapiens.notification_logs CASCADE;
DROP FUNCTION IF EXISTS sapiens.notification_logs_audit_timestamp() CASCADE;
