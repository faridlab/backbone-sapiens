-- Down: drop password_reset_security table
DROP TABLE IF EXISTS password_reset_security CASCADE;
DROP FUNCTION IF EXISTS password_reset_security_audit_timestamp() CASCADE;
