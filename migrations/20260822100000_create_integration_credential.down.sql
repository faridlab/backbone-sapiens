-- Down: drop the credential store table, triggers, fence and enum types.

DROP POLICY IF EXISTS integration_credentials_company_isolation ON sapiens.integration_credentials;
ALTER TABLE sapiens.integration_credentials NO FORCE ROW LEVEL SECURITY;
ALTER TABLE sapiens.integration_credentials DISABLE ROW LEVEL SECURITY;

DROP TRIGGER IF EXISTS integration_credentials_update_audit ON sapiens.integration_credentials;
DROP TRIGGER IF EXISTS integration_credentials_insert_audit ON sapiens.integration_credentials;
DROP FUNCTION IF EXISTS sapiens.integration_credentials_audit_timestamp();

DROP TABLE IF EXISTS sapiens.integration_credentials;

DROP TYPE IF EXISTS credential_status;
DROP TYPE IF EXISTS credential_purpose;
