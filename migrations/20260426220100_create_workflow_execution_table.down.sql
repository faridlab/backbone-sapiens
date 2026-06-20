-- Down: drop workflow_executions table
DROP TABLE IF EXISTS workflow_executions CASCADE;
DROP FUNCTION IF EXISTS workflow_executions_audit_timestamp() CASCADE;
