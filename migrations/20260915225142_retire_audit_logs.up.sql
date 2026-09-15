-- Retire the identity-side audit table.
--
-- Auditing is one estate now: `auditlog.audit_trails`, written by the capture
-- triggers and by the audit verbs, read by the record-history and activity-feed
-- surfaces. This table had no producer at all — no verb, no trigger, no
-- workflow wrote to it — and no rows in any environment checked. It was a
-- generated CRUD surface that looked like identity auditing without ever
-- recording any.
--
-- Nothing is migrated because there is nothing to migrate. Identity events that
-- should be audited belong on the shared trail, where they can be read beside
-- every other module's.
DROP TABLE IF EXISTS sapiens.audit_logs;
DROP TYPE IF EXISTS audit_log_severity;
