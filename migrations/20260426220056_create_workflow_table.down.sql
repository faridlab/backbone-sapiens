-- Down: drop workflows table
DROP TABLE IF EXISTS workflows CASCADE;
DROP FUNCTION IF EXISTS workflows_audit_timestamp() CASCADE;
