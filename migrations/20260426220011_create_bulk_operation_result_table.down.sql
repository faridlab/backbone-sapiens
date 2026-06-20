-- Down: drop sapiens.bulk_operation_results table
DROP TABLE IF EXISTS sapiens.bulk_operation_results CASCADE;
DROP FUNCTION IF EXISTS sapiens.bulk_operation_results_audit_timestamp() CASCADE;
