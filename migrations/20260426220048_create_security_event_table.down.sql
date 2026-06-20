-- Down: drop sapiens.security_events table
DROP TABLE IF EXISTS sapiens.security_events CASCADE;
DROP FUNCTION IF EXISTS sapiens.security_events_audit_timestamp() CASCADE;
