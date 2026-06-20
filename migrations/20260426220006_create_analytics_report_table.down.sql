-- Down: drop sapiens.analytics_reports table
DROP TABLE IF EXISTS sapiens.analytics_reports CASCADE;
DROP FUNCTION IF EXISTS sapiens.analytics_reports_audit_timestamp() CASCADE;
