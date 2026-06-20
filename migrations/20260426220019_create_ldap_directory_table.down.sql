-- Down: drop sapiens.ldap_directories table
DROP TABLE IF EXISTS sapiens.ldap_directories CASCADE;
DROP FUNCTION IF EXISTS sapiens.ldap_directories_audit_timestamp() CASCADE;
