-- Down: drop analytics_reports table
DROP TABLE IF EXISTS analytics_reports CASCADE;
DROP FUNCTION IF EXISTS analytics_reports_audit_timestamp() CASCADE;
