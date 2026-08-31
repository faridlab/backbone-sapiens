-- The transactional outbox for this module's own schema (mirrors backbone_outbox::outbox::migrate).
-- User-account domain events (UserDomainEvent, e.g. "UserCreated") are staged here INSIDE the same
-- transaction as the user write on the self-registration path, or immediately after the CRUD
-- repository insert via the service-layer event hook — so a crash between the state commit and any
-- downstream publish can never drop the event. A relay drains outbox_events at-least-once;
-- consumers dedup via their own inbox_consumed.
--
-- User accounts have no company dimension in this module, so the NOT NULL company_id column
-- (inserted unconditionally by backbone_outbox::outbox::stage) carries the nil sentinel uuid and
-- carries NO row-level-security fence — the same posture as the messaging module's platform
-- events. Consumers key on the event type and aggregate id, never on company_id.
CREATE TABLE IF NOT EXISTS sapiens.outbox_events (
  id             uuid PRIMARY KEY,
  event_type     text NOT NULL,
  aggregate_type text NOT NULL,
  aggregate_id   text NOT NULL,
  company_id     uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',
  payload        jsonb NOT NULL,
  occurred_at    timestamptz NOT NULL,
  correlation_id text,
  causation_id   text,
  version        int NOT NULL DEFAULT 1,
  created_at     timestamptz NOT NULL DEFAULT now(),
  published_at   timestamptz
);
CREATE INDEX IF NOT EXISTS idx_sapiens_outbox_unpublished
  ON sapiens.outbox_events (occurred_at) WHERE published_at IS NULL;

CREATE TABLE IF NOT EXISTS sapiens.inbox_consumed (
  consumer    text NOT NULL,
  event_id    uuid NOT NULL,
  consumed_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (consumer, event_id)
);
