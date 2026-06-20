-- Down: drop sapiens.notification_templates table
DROP TABLE IF EXISTS sapiens.notification_templates CASCADE;
DROP FUNCTION IF EXISTS sapiens.notification_templates_audit_timestamp() CASCADE;
