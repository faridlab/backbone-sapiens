-- Down: drop sapiens.analytics_metrics table
DROP TABLE IF EXISTS sapiens.analytics_metrics CASCADE;
DROP FUNCTION IF EXISTS sapiens.analytics_metrics_audit_timestamp() CASCADE;
