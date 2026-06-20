-- Down: drop security_events table
DROP TABLE IF EXISTS security_events CASCADE;
DROP FUNCTION IF EXISTS security_events_audit_timestamp() CASCADE;
