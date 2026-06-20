-- Down: drop password_policies table
DROP TABLE IF EXISTS password_policies CASCADE;
DROP FUNCTION IF EXISTS password_policies_audit_timestamp() CASCADE;
