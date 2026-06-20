-- Down: drop sapiens.password_creation_contexts table
DROP TABLE IF EXISTS sapiens.password_creation_contexts CASCADE;
DROP FUNCTION IF EXISTS sapiens.password_creation_contexts_audit_timestamp() CASCADE;
