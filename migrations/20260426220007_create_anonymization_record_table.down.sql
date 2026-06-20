-- Down: drop sapiens.anonymization_records table
DROP TABLE IF EXISTS sapiens.anonymization_records CASCADE;
DROP FUNCTION IF EXISTS sapiens.anonymization_records_audit_timestamp() CASCADE;
