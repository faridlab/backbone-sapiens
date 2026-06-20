-- Down: drop email_verification_tokens table
DROP TABLE IF EXISTS email_verification_tokens CASCADE;
DROP FUNCTION IF EXISTS email_verification_tokens_audit_timestamp() CASCADE;
