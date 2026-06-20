-- Down: drop sapiens.oauth_providers table
DROP TABLE IF EXISTS sapiens.oauth_providers CASCADE;
DROP FUNCTION IF EXISTS sapiens.oauth_providers_audit_timestamp() CASCADE;
