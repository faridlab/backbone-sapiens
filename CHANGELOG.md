# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
