-- Down: drop organization_users table
DROP TABLE IF EXISTS organization_users CASCADE;
DROP FUNCTION IF EXISTS organization_users_audit_timestamp() CASCADE;
