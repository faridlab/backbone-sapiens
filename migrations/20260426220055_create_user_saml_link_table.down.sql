-- Down: drop user_saml_links table
DROP TABLE IF EXISTS user_saml_links CASCADE;
DROP FUNCTION IF EXISTS user_saml_links_audit_timestamp() CASCADE;
