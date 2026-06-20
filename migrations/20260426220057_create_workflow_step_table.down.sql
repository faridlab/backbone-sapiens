-- Down: drop workflow_steps table
DROP TABLE IF EXISTS workflow_steps CASCADE;
DROP FUNCTION IF EXISTS workflow_steps_audit_timestamp() CASCADE;
