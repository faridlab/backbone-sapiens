-- Down: drop sapiens.email_verification_tokens table
DROP TABLE IF EXISTS sapiens.email_verification_tokens CASCADE;
DROP FUNCTION IF EXISTS sapiens.email_verification_tokens_audit_timestamp() CASCADE;
