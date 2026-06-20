-- Down: drop notifications table
DROP TABLE IF EXISTS notifications CASCADE;
DROP FUNCTION IF EXISTS notifications_audit_timestamp() CASCADE;
