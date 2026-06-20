-- Down: drop sapiens.workflow_action_executions table
DROP TABLE IF EXISTS sapiens.workflow_action_executions CASCADE;
DROP FUNCTION IF EXISTS sapiens.workflow_action_executions_audit_timestamp() CASCADE;
