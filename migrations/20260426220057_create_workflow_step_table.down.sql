-- Down: drop sapiens.workflow_steps table
DROP TABLE IF EXISTS sapiens.workflow_steps CASCADE;
DROP FUNCTION IF EXISTS sapiens.workflow_steps_audit_timestamp() CASCADE;
