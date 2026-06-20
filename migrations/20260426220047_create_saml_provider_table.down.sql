-- Down: drop sapiens.saml_providers table
DROP TABLE IF EXISTS sapiens.saml_providers CASCADE;
DROP FUNCTION IF EXISTS sapiens.saml_providers_audit_timestamp() CASCADE;
