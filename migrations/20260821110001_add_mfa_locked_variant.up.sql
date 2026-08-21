-- Migration: add the `locked` variant to mfa_device_status
--
-- The temporary-lockout boolean `mfa_devices.is_locked` folds into the
-- existing status enum (the tree-wide one-status-field convention lives in
-- docs/refactoring-schema in the serpa workspace). Lockout auto-expires, so
-- it stays a distinct variant from `suspended` (an admin action).
--
-- This stamp only adds the value: Postgres cannot reliably use a newly added
-- enum value inside the same transaction that added it, so the data
-- translation rides the next stamp (20260821110002_status_lifecycle).

ALTER TYPE mfa_device_status ADD VALUE IF NOT EXISTS 'locked';
