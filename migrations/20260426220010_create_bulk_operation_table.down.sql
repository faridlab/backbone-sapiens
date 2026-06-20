-- Down: drop bulk_operations table
DROP TABLE IF EXISTS bulk_operations CASCADE;
DROP FUNCTION IF EXISTS bulk_operations_audit_timestamp() CASCADE;
