-- Down: drop roles table
DROP TABLE IF EXISTS roles CASCADE;
DROP FUNCTION IF EXISTS roles_audit_timestamp() CASCADE;
