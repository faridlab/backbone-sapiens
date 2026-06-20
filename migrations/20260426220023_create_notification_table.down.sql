-- Down: drop sapiens.notifications table
DROP TABLE IF EXISTS sapiens.notifications CASCADE;
DROP FUNCTION IF EXISTS sapiens.notifications_audit_timestamp() CASCADE;
