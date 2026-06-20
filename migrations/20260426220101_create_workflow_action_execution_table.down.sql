-- Down: drop workflow_action_executions table
DROP TABLE IF EXISTS workflow_action_executions CASCADE;
DROP FUNCTION IF EXISTS workflow_action_executions_audit_timestamp() CASCADE;
