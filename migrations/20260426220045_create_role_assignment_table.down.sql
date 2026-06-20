-- Down: drop role_assignments table
DROP TABLE IF EXISTS role_assignments CASCADE;
DROP FUNCTION IF EXISTS role_assignments_audit_timestamp() CASCADE;
