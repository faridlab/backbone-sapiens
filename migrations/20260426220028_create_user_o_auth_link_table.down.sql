-- Down: drop sapiens.user_oauth_links table
DROP TABLE IF EXISTS sapiens.user_oauth_links CASCADE;
DROP FUNCTION IF EXISTS sapiens.user_oauth_links_audit_timestamp() CASCADE;
