-- Down: drop notification_templates table
DROP TABLE IF EXISTS notification_templates CASCADE;
DROP FUNCTION IF EXISTS notification_templates_audit_timestamp() CASCADE;
