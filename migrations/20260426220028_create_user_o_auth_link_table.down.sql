-- Down: drop user_oauth_links table
DROP TABLE IF EXISTS user_oauth_links CASCADE;
DROP FUNCTION IF EXISTS user_oauth_links_audit_timestamp() CASCADE;
