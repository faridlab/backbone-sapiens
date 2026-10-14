-- The declared rule unique_active_codes allowed one unused backup code per user and batch, which
-- contradicts a batch holding at least eight; the rule is withdrawn, so its index goes with it.
DROP INDEX IF EXISTS sapiens.uq_backup_codes_unique_active_codes__backup_codes_exist;
