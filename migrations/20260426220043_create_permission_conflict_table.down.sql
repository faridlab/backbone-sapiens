-- Down: drop permission_conflicts table
DROP TABLE IF EXISTS permission_conflicts CASCADE;
DROP FUNCTION IF EXISTS permission_conflicts_audit_timestamp() CASCADE;
