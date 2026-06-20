-- Down: drop password_reset_tokens table
DROP TABLE IF EXISTS password_reset_tokens CASCADE;
DROP FUNCTION IF EXISTS password_reset_tokens_audit_timestamp() CASCADE;
