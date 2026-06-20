-- Down: drop sapiens.permission_conflicts table
DROP TABLE IF EXISTS sapiens.permission_conflicts CASCADE;
DROP FUNCTION IF EXISTS sapiens.permission_conflicts_audit_timestamp() CASCADE;
