-- Down: drop password_resets table
DROP TABLE IF EXISTS password_resets CASCADE;
DROP FUNCTION IF EXISTS password_resets_audit_timestamp() CASCADE;
