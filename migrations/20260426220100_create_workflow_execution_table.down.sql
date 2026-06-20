-- Down: drop sapiens.workflow_executions table
DROP TABLE IF EXISTS sapiens.workflow_executions CASCADE;
DROP FUNCTION IF EXISTS sapiens.workflow_executions_audit_timestamp() CASCADE;
