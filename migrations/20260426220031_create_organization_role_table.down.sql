-- Down: drop sapiens.organization_roles table
DROP TABLE IF EXISTS sapiens.organization_roles CASCADE;
DROP FUNCTION IF EXISTS sapiens.organization_roles_audit_timestamp() CASCADE;
