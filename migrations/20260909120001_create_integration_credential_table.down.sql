-- Down: drop sapiens.integration_credentials table
DROP TABLE IF EXISTS sapiens.integration_credentials CASCADE;
DROP FUNCTION IF EXISTS sapiens.integration_credentials_audit_timestamp() CASCADE;
