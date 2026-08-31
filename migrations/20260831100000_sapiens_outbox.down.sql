-- Reverse the sapiens outbox migration.
DROP TABLE IF EXISTS sapiens.inbox_consumed;
DROP TABLE IF EXISTS sapiens.outbox_events;
