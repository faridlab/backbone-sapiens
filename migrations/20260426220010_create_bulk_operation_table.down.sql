-- Down: drop sapiens.bulk_operations table
DROP TABLE IF EXISTS sapiens.bulk_operations CASCADE;
DROP FUNCTION IF EXISTS sapiens.bulk_operations_audit_timestamp() CASCADE;
