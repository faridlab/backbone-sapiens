-- Down: drop sapiens.workflow_definitions table
DROP TABLE IF EXISTS sapiens.workflow_definitions CASCADE;
DROP FUNCTION IF EXISTS sapiens.workflow_definitions_audit_timestamp() CASCADE;
