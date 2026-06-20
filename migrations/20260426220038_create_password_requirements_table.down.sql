-- Down: drop sapiens.password_requirements table
DROP TABLE IF EXISTS sapiens.password_requirements CASCADE;
DROP FUNCTION IF EXISTS sapiens.password_requirements_audit_timestamp() CASCADE;
