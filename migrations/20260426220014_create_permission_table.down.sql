-- Down: drop permissions table
DROP TABLE IF EXISTS permissions CASCADE;
DROP FUNCTION IF EXISTS permissions_audit_timestamp() CASCADE;
