-- Down: drop sapiens.workflow_actions table
DROP TABLE IF EXISTS sapiens.workflow_actions CASCADE;
DROP FUNCTION IF EXISTS sapiens.workflow_actions_audit_timestamp() CASCADE;
