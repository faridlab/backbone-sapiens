-- Down: drop analytics_events table
DROP TABLE IF EXISTS analytics_events CASCADE;
DROP FUNCTION IF EXISTS analytics_events_audit_timestamp() CASCADE;
