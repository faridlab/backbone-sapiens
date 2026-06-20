-- Down: drop sapiens.workflows table
DROP TABLE IF EXISTS sapiens.workflows CASCADE;
DROP FUNCTION IF EXISTS sapiens.workflows_audit_timestamp() CASCADE;
