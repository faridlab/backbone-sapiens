-- Down: drop profiles table
DROP TABLE IF EXISTS profiles CASCADE;
DROP FUNCTION IF EXISTS profiles_audit_timestamp() CASCADE;
