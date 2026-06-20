-- Down: drop sapiens.password_reset_security table
DROP TABLE IF EXISTS sapiens.password_reset_security CASCADE;
DROP FUNCTION IF EXISTS sapiens.password_reset_security_audit_timestamp() CASCADE;
