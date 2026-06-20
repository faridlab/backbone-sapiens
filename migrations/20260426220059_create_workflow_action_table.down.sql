-- Down: drop workflow_actions table
DROP TABLE IF EXISTS workflow_actions CASCADE;
DROP FUNCTION IF EXISTS workflow_actions_audit_timestamp() CASCADE;
