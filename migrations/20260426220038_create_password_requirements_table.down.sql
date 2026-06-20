-- Down: drop password_requirements table
DROP TABLE IF EXISTS password_requirements CASCADE;
DROP FUNCTION IF EXISTS password_requirements_audit_timestamp() CASCADE;
