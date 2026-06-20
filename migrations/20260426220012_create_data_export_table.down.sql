-- Down: drop data_exports table
DROP TABLE IF EXISTS data_exports CASCADE;
DROP FUNCTION IF EXISTS data_exports_audit_timestamp() CASCADE;
