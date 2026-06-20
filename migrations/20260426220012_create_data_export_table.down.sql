-- Down: drop sapiens.data_exports table
DROP TABLE IF EXISTS sapiens.data_exports CASCADE;
DROP FUNCTION IF EXISTS sapiens.data_exports_audit_timestamp() CASCADE;
