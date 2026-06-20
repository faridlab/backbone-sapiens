-- Down: drop deferred foreign keys for sapiens module
ALTER TABLE user_settings DROP CONSTRAINT IF EXISTS fk_user_settings_user_id;
