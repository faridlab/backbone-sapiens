-- Down: remove the company RLS fence for sapiens module

-- Reverse the company RLS fence for sapiens.integration_credentials
DROP POLICY IF EXISTS integration_credentials_company_isolation ON sapiens.integration_credentials;
ALTER TABLE sapiens.integration_credentials NO FORCE ROW LEVEL SECURITY;
ALTER TABLE sapiens.integration_credentials DISABLE ROW LEVEL SECURITY;

