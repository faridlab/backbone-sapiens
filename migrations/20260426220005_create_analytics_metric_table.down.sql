-- Down: drop analytics_metrics table
DROP TABLE IF EXISTS analytics_metrics CASCADE;
DROP FUNCTION IF EXISTS analytics_metrics_audit_timestamp() CASCADE;
