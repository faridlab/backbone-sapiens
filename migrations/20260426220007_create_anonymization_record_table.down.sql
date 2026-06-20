-- Down: drop anonymization_records table
DROP TABLE IF EXISTS anonymization_records CASCADE;
DROP FUNCTION IF EXISTS anonymization_records_audit_timestamp() CASCADE;
