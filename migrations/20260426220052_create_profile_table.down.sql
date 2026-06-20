-- Down: drop sapiens.profiles table
DROP TABLE IF EXISTS sapiens.profiles CASCADE;
DROP FUNCTION IF EXISTS sapiens.profiles_audit_timestamp() CASCADE;
