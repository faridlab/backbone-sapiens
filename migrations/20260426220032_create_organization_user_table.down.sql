-- Down: drop sapiens.organization_users table
DROP TABLE IF EXISTS sapiens.organization_users CASCADE;
DROP FUNCTION IF EXISTS sapiens.organization_users_audit_timestamp() CASCADE;
