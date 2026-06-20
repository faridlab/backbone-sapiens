-- Down: drop sapiens.analytics_events table
DROP TABLE IF EXISTS sapiens.analytics_events CASCADE;
DROP FUNCTION IF EXISTS sapiens.analytics_events_audit_timestamp() CASCADE;
