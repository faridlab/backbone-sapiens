-- Down: drop oauth_providers table
DROP TABLE IF EXISTS oauth_providers CASCADE;
DROP FUNCTION IF EXISTS oauth_providers_audit_timestamp() CASCADE;
