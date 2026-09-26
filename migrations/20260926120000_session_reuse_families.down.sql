DROP INDEX IF EXISTS sapiens.idx_sessions_family;
ALTER TABLE sapiens.sessions DROP COLUMN IF EXISTS replaced_by;
ALTER TABLE sapiens.sessions DROP COLUMN IF EXISTS family_id;
