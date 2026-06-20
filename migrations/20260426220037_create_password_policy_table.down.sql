-- Down: drop sapiens.password_policies table
DROP TABLE IF EXISTS sapiens.password_policies CASCADE;
DROP FUNCTION IF EXISTS sapiens.password_policies_audit_timestamp() CASCADE;
