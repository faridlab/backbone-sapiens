# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.4] - 2026-08-31

### Added
- Publish `UserDomainEvent::Created` on every user-creation path, not just
  self-registration. Both legs are wired: a durable outbox row staged into
  `sapiens.outbox_events` (new migration creates the outbox/inbox tables, with
  the nil-uuid company sentinel because users are platform-level) and an
  in-process publish on the module's typed event bus, translated to the
  cross-module `sapiens.user.created` integration event. Self-registration
  stages inside the registration transaction; the admin/CRUD paths
  (`POST /users`, `/users/bulk`, `/users/upsert`) stage post-commit through the
  generic CRUD event-publisher hook. When no integration bus is wired the
  in-process publish is silently dropped while the outbox row still lands —
  documented in `docs/spec-sapiens.md` §7.
- Re-fire idempotency on the CRUD publish path: a create whose email already
  has a live (non-soft-deleted) user row does not emit a second `UserCreated`.
  This absorbs a schema gap — the users table's soft-delete-aware email unique
  index does not refuse live duplicates because Postgres treats the NULL
  `deleted_at` as distinct; the durable fix is a partial unique index
  (`UNIQUE (email) WHERE metadata->>'deleted_at' IS NULL`), tracked as a
  follow-up.
- Internal-user predicate: `is_internal_user` / `internal_user_ids` (an
  internal user is one with an ACTIVE `organization_users` membership —
  `status = 'active'` and not soft-deleted). Exported for consumers to apply
  at event-consumption time; the event payload intentionally carries no
  `is_internal` flag since membership is point-in-time state. Definition
  recorded in `docs/spec-sapiens.md` §7.
- Database probes (`tests/user_created_publish_probes.rs`) covering: CRUD
  create stages exactly one outbox row; re-fired creates do not double-publish
  and soft-deleted emails re-publish; self-registration publishes exactly once
  (outbox + typed bus) and refuses duplicates; the translator delivers
  `sapiens.user.created`; the internal-user definition holds for none /
  non-active / active membership.

### Changed
- Add the `backbone-outbox` dependency (framework pin `v2.7.11`, tag-equal with
  the existing framework pins) for outbox staging, and bump the package
  version to `0.2.4` — the version manifest now matches the tag it ships in
  (the `v0.2.3` tag carried manifest `0.2.2`).

## [0.2.2] - 2026-08-23

### Fixed
- Stop mounting the `/notifications` and `/notification_templates` CRUD
  routers in `SapiensModule::routes()`. Those route bases are owned by the
  `backbone-notification` module; registering them from both modules
  double-registers the bases and panics the router at boot whenever both are
  composed into one service. The notification and notification_template
  entities, services, and per-entity handler functions remain available for
  hosts that want them; the `/notification_logs` and
  `/notification_preferences` routers are unaffected and stay mounted.

## [0.1.7] - 2026-05-17

### Changed
- Bump `backbone-framework` dependencies to `v2.1.9`.

## [0.1.6] - 2026-05-17

### Changed
- Bump `backbone-framework` dependencies to `v2.1.4`.
- Update documentation for account lockout duration (15 → 5 minutes).

### Fixed
- Reduce account lockout duration from 15 minutes to 5 minutes.

## [0.1.5] - 2026-05-17

### Changed
- Bump `backbone-framework` dependencies to `v2.1.3`.

## [0.1.4] - 2026-05-17

### Changed
- Bump `backbone-framework` dependencies to `v2.1.2`.

## [0.1.3] - 2026-05-17

### Changed
- Bump `backbone-framework` dependencies to `v2.1.1`.

## [0.1.2] - 2026-05-17

### Changed
- Version bump.

## [0.1.1] - 2026-05-17

### Changed
- Pin `backbone-framework` to tag `v2.0.0`.

## [0.1.0] - 2026-05-17

### Added
- Initial release of `backbone-sapiens` user management module.
- Domain, application, infrastructure, and presentation layers.
- HTTP, gRPC, CLI, and GraphQL handlers.
- Database migrations, seed data, and schema definitions.
- Unit, integration, and scenario tests.
