-- Down: drop sapiens.sapiens_notification_preferences table
DROP TABLE IF EXISTS sapiens.sapiens_notification_preferences CASCADE;
DROP FUNCTION IF EXISTS sapiens.sapiens_notification_preferences_audit_timestamp() CASCADE;
