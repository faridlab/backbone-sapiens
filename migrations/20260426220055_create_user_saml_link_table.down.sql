-- Down: drop sapiens.user_saml_links table
DROP TABLE IF EXISTS sapiens.user_saml_links CASCADE;
DROP FUNCTION IF EXISTS sapiens.user_saml_links_audit_timestamp() CASCADE;
