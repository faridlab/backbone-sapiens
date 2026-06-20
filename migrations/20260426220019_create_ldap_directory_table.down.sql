-- Down: drop ldap_directories table
DROP TABLE IF EXISTS ldap_directories CASCADE;
DROP FUNCTION IF EXISTS ldap_directories_audit_timestamp() CASCADE;
