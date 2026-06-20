-- Down: drop sapiens.role_assignments table
DROP TABLE IF EXISTS sapiens.role_assignments CASCADE;
DROP FUNCTION IF EXISTS sapiens.role_assignments_audit_timestamp() CASCADE;
