-- Down: drop notification_logs table
DROP TABLE IF EXISTS notification_logs CASCADE;
DROP FUNCTION IF EXISTS notification_logs_audit_timestamp() CASCADE;
