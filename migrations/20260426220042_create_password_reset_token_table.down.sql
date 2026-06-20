-- Down: drop sapiens.password_reset_tokens table
DROP TABLE IF EXISTS sapiens.password_reset_tokens CASCADE;
DROP FUNCTION IF EXISTS sapiens.password_reset_tokens_audit_timestamp() CASCADE;
