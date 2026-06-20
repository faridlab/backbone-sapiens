-- Down: drop sapiens.password_history table
DROP TABLE IF EXISTS sapiens.password_history CASCADE;
DROP FUNCTION IF EXISTS sapiens.password_history_audit_timestamp() CASCADE;
