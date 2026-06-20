-- Down: drop organization_roles table
DROP TABLE IF EXISTS organization_roles CASCADE;
DROP FUNCTION IF EXISTS organization_roles_audit_timestamp() CASCADE;
