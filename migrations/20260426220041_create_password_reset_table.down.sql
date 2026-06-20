-- Down: drop sapiens.password_resets table
DROP TABLE IF EXISTS sapiens.password_resets CASCADE;
DROP FUNCTION IF EXISTS sapiens.password_resets_audit_timestamp() CASCADE;
