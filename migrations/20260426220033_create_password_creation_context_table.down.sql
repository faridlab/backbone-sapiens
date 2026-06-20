-- Down: drop password_creation_contexts table
DROP TABLE IF EXISTS password_creation_contexts CASCADE;
DROP FUNCTION IF EXISTS password_creation_contexts_audit_timestamp() CASCADE;
