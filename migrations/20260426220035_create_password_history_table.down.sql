-- Down: drop password_history table
DROP TABLE IF EXISTS password_history CASCADE;
DROP FUNCTION IF EXISTS password_history_audit_timestamp() CASCADE;
