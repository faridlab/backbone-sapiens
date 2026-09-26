-- Session reuse families: a replayed refresh token must revoke its chain.
--
-- Rotation already revoked the old row and inserted a successor, but the
-- successor had no link to its predecessor: a thief replaying a spent token
-- just failed, while the token they stole the successor of stayed alive.
-- `family_id` names the chain (one family per login, every rotation
-- appends), so a replay can revoke every live row in the family; and a
-- spent row now records `replaced_by`, the successor that superseded it.
--
-- Existing rows backfill as their own family head, which is exact for
-- live single-token chains and harmless for historical ones (a replay of
-- an old token can only revoke rows that descend from that same row).

ALTER TABLE sapiens.sessions ADD COLUMN IF NOT EXISTS family_id UUID;
UPDATE sapiens.sessions SET family_id = id WHERE family_id IS NULL;
ALTER TABLE sapiens.sessions ALTER COLUMN family_id SET NOT NULL;

ALTER TABLE sapiens.sessions ADD COLUMN IF NOT EXISTS replaced_by UUID;

CREATE INDEX IF NOT EXISTS idx_sessions_family ON sapiens.sessions (family_id);
