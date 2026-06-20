-- Down: drop saml_providers table
DROP TABLE IF EXISTS saml_providers CASCADE;
DROP FUNCTION IF EXISTS saml_providers_audit_timestamp() CASCADE;
