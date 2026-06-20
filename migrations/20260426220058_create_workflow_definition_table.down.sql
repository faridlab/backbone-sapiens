-- Down: drop workflow_definitions table
DROP TABLE IF EXISTS workflow_definitions CASCADE;
DROP FUNCTION IF EXISTS workflow_definitions_audit_timestamp() CASCADE;
