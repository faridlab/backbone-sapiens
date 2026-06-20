-- Down: drop password_reset_verification_details table
DROP TABLE IF EXISTS password_reset_verification_details CASCADE;
DROP FUNCTION IF EXISTS password_reset_verification_details_audit_timestamp() CASCADE;
