# O-2 identity hardening — threat notes (the cycle-27 closure record)

> This is the O-2 DoD artifact: the threat-notes doc in sapiens mapping every
> cycle-27 portal-auth finding **PI-01..PI-44** to exactly one disposition —
> **sapiens behavior / portal behavior / replaced-by-design / fenced /
> WB-3-homed** — plus the honest interim posture and the test obligations.
>
> Written 2026-09-01 at the WB-1 pass (portal/auth + the deferred identity
> closure). Spec sources: the pillar
> ([workspace docs/plan/08-pillar-website.md](../../../../../products/serpa-workspace/docs/plan/08-pillar-website.md) §WB-1),
> the O-2 deferral ([01-pillar-organization.md](../../../../../products/serpa-workspace/docs/plan/01-pillar-organization.md) §O-2),
> and the cycle-27 cohort
> ([docs/odoo/foundation/portal-auth/](../../../docs/odoo/foundation/portal-auth/) —
> README, features, business-logic, auth-providers, and the five schema/
> annotation files carrying the PI census). File:line anchors below refer to
> the working trees as they stand at this pass (pre-tag-cut).

## 1. The interim posture, stated precisely (as of this pass)

What is TRUE at write time — no more, no less:

1. **The signup kill-switch now EXISTS and is default OFF, twice over.**
   Sapiens: `sapiens.auth_policy` holds the switch; a missing row (the fresh
   database state — nothing is inserted at install time) reads as signup OFF
   (migration `20260901100001_auth_public_form_hardening.up.sql`;
   `src/presentation/http/public_auth_routes.rs` mounts the policy check).
   Portal: `portal.portal_signup_policies` is a singleton row read
   fail-closed — absent row, read error, or disabled row all read as closed
   (`modules/backbone-portal/src/application/service/policy_service.rs:37-49`,
   probe `signup_is_off_by_default_and_fail_closed`). Signup-off is therefore
   no longer held merely by unmounted-ness; it is held by policy default AND
   by the mount state below.

2. **The rebuilt sapiens public auth router stays UNMOUNTED.**
   `create_public_auth_routes` is exported, never mounted:
   `SapiensModule::routes()` does not include it and must never grow it
   (`src/presentation/http/mod.rs:141-142`; the mounting contract is the file
   header of `public_auth_routes.rs`). Whether and where the public forms
   answer is a recorded host decision per deployment — the router is shaped
   so a host cannot mount it without also accepting the declared posture
   (policy-gated signup, per-identity AND per-IP throttle, de-oracled
   replies, reverse-proxy client-address headers).

3. **The live host-owned session bridge remains the ONLY exposed auth
   surface.** The employee bridge answers at `/api/v1/auth` (login, me,
   refresh — host `src/presentation/auth.rs`, mounted in host `main.rs`),
   with uniform errors verified (wrong-password and unknown-email both answer
   the same 401 body), and it is NOW Tier B throttled in-service per-identity:
   5 failures lock for 30 s, doubling to a 900 s cap, 1 s minimum spacing,
   success resets, keyed per normalized email; the lockout answers 429 +
   `Retry-After` BEFORE the handler runs (host `src/middleware/auth_throttle.rs`).
   The middleware ALSO carries a per-IP leg keyed on the first
   `X-Forwarded-For` hop — but that hop is client-controlled behind Caddy's
   append semantics, so the per-IP leg is spray-hardening only, NOT a
   trustworthy anti-enumeration bound (verified live: distinct spoofed
   `X-Forwarded-For` values each get a fresh IP budget; the per-identity leg
   holds under the same attack). The compensating-control claim is therefore
   **per-identity only** until the per-IP key moves to the rightmost hop
   behind an explicit trusted-proxy flag. Caddy still fronts the surface.

4. **The portal tree composes behind a cargo feature gate, not by default.**
   The portal module's public surface (`/api/v1/portal`: auth/redeem-invite,
   auth/login, auth/signup, auth/rotate, me GET+PATCH, me/access-history —
   bare module nest, throttled, uniform refusals) is composed by the host
   behind `#[cfg(feature = "compose-portal")]` while the portal pin is
   dev-time. In the DEFAULT build the portal base 404s; the feature exists so
   verification/training trees compose the real pin beside it. Flipping the
   gate off cannot change any wire shape of the default build.

5. **The portal's Tier A credential pair is split and independent.** An
   expiring, rotatable bearer (Authorization header) and a per-recipient
   HMAC credential whose input does NOT cover the bearer — either rotates
   without invalidating the other
   (`modules/backbone-portal/src/application/service/token_service.rs`;
   probes `bearer_mint_verify_rotate_audited`,
   `credentials_rotate_independently`). Mints and rotations are audited.

## 2. Carried residuals and gaps (the honest list)

These are known, deliberately not fixed in this pass, and tracked:

- **The live bridge's pre-credential 403 oracle and unknown-email argon2
  skip** (host `src/middleware/auth_throttle.rs:30-34` — the note is in the
  file). Changing either alters the live admin webapp's login contract; the
  residual is a status/timing oracle, bounded by the throttle now answering
  before the handler. It closes when the bridge is absorbed into or
  re-fronted by the sapiens public router — a recorded host decision.
- **The lifecycle producers are wired in-tree; runtime awaits the release
  leg.** The portal subscribes to `sapiens.user.deactivated` / `.deleted`
  (revoke access + invalidate every live bearer), and this pass wires the
  producers in the sapiens working tree: `UserDeactivationLifecycle` fires
  exactly once on the active→inactive transition
  (`src/infrastructure/messaging/user_lifecycle_outbox.rs`), `UserDeleted`
  stages from `CrudEvent::SoftDeleted`
  (`src/infrastructure/messaging/user_created_outbox.rs:203`), and the
  anonymization record publishes on its own CRUD path (publisher wired at
  `src/lib.rs`, the `sapiens.user.anonymized` vocabulary at
  `src/infrastructure/messaging/integration_events.rs:336,:347`; the staged
  three-arm probe is `lifecycle_outbox_stages_deactivated_anonymized_deleted`
  in `tests/auth_hardening_probes.rs:562`). The revocation arms fire in
  production once the sapiens release train leg lands (the increment is
  currently an uncommitted diff on the published v0.2.4 tag — the portal
  module re-pins to that release in the same train); until that leg ships,
  archive/delete of a linked identity in a DEPLOYED build does not revoke
  portal access, and the officer-side revoke verb (probe
  `revoke_access_sweeps_everything`) remains the interim control. The
  anonymize arm itself is a named train step: the portal's subscription adds
  `sapiens.user.anonymized` (revoke + bearer sweep, mirroring the deleted
  arm) compiled against the released event.
- **The mail dual-resolve debt is untouched here** (recorded W7 condition;
  heals at the events-mail pass on its own train).
- **OAuth-as-login is filed on-demand, not built**: the one OAuth generation
  (ADR-0024) is a credential-exchange surface on backbone-integrations; it
  is not a login method for this service.

## 3. auth_timeout — carried explicitly (do not lose it between texts)

`auth_timeout` appears in the wave coverage map
(`docs/plan/app-coverage-map.md:339` — "idle session timeout in the identity
pass") but in **no** WB-1 bullet of the pillar: it would be easy to drop it
between the two texts. It is part of THIS closure and it LANDED:

- **PI-42 (timeout enforcement)** — sapiens now declares both postures on the
  session surface, on the last-activity basis:
  `SESSION_TIMEOUT_POLICY { absolute: 30 days, idle: 7 days }`
  (`src/application/service/auth_service.rs:92-112`). The absolute cap is the
  `expires_at` stamped at login/refresh; the idle posture is enforced at
  token refresh against `last_activity` (rows predating activity tracking
  fall back to creation time) — an idle-expired refresh is refused and its
  session revoked even if not yet absolutely expired
  (`auth_service.rs:762-772`).
- **PI-43 (trusted-device clamp)** — a trusted-device key's requested
  lifetime is clamped to the MFA session timeout, so a remembered device can
  never outlive the MFA window
  (`src/application/service/device_trust_key_service.rs:59-65`).
- **PI-44 (the re-auth surface)** — vocabulary landed, surface not yet
  exposed: trusted-device keys are "reserved for the step-up flow the host
  mounts later" (`public_auth_routes.rs:83-84`). The two-factor re-auth
  endpoints arm when a host mounts the public router; nothing answers today.

## 4. The PI-01..PI-44 disposition map

Anchors are `src/...` in this crate unless a tree is named. "Portal" anchors
live in `modules/backbone-portal/`; "integrations" in the backbone-integrations
release cited; "host" in the consuming service app.

| ID | Cycle-27 finding (one line) | Disposition | Where it lands / landed |
|----|------------------------------|-------------|--------------------------|
| PI-01 | Lazy plaintext uuid4 bearer, never rotated or revoked | replaced-by-design | Portal bearer is expiring + rotatable with mint/rotation audit (`token_service.rs:486`, probes `bearer_mint_verify_rotate_audited`) |
| PI-02 | One token field backs both the bearer and the pid-hash HMAC input | replaced-by-design | Two independent credentials, separate secrets; the HMAC input excludes the bearer (`token_service.rs:306-316`; probe `credentials_rotate_independently`) |
| PI-03 | `_document_check_access` = SUPERUSER browse + consteq fallback gate | replaced-by-design | Declared read models + explicit verbs, ownership-scoped by the bearer principal — no sudo-browse-then-filter (portal `portal_surface.rs`; host composes the nest) |
| PI-04 | Sudo address writes fenced only by route ownership + a 12-field allowlist | portal behavior | The detail verb's field whitelist — `PortalDetailPatch`, 10 writable fields, edge-side drop of everything else (portal `portal_surface.rs:93-104`, `public_routes.rs:307-313`) |
| PI-05 | The address engine's one public readonly route (`country_info`) | replaced-by-design | The public surface is an explicit six-route allowlist; no public address-metadata route ships. Country data reaches the portal only through the whitelist write path |
| PI-06 | Chatter visibility domain (internal messages invisible in portal chatter) | portal behavior | No chatter route ships in this pass; the visibility contract is declared with the first portal document surface (its consumers land with the website/companions passes) |
| PI-07 | Grant wizard's python-only email-uniqueness (concurrent grants pass the search) | replaced-by-design | Database-backed fence: partial unique on live portal emails (`20260426220100_portal_hardening_constraints.up.sql:17`) + exactly-once redemption (probe `invite_redeems_exactly_once`) |
| PI-08 | Share b2c/b2b link shapes (public link vs individual signup links) | portal behavior | Invitation redemption is the shipped path — Tier A invite credentials, single-use, expiring (probes `invite_redeems_exactly_once`, `forwarded_link_refuses`, `expired_invites_refuse`) |
| PI-09 | Token invalidation state-based; no revocation list ("partner unchanged, link leaked" survives) | replaced-by-design | Explicit revocation: invite `revoked_at`/`revocation_reason` (probe `revocation_list_bites`), `sapiens.auth_signup_revocations` bars a credential even when it still verifies; the lifecycle arms revoke on identity death — producers wired in-tree this pass (§2), live in deployed builds when the sapiens release + re-pin train leg ships, with the anonymize arm added in that same train |
| PI-10 | Signup/reset token expiry 4h/144h as ICPs | portal behavior | Declared lifetimes: invite `token_expires_at`, bearer TTL at mint and rotation (`DEFAULT_BEARER_TTL_HOURS`) |
| PI-11 | User-enumeration oracles on public signup/reset forms | replaced-by-design | Sapiens register replies the SAME status/body whether the address was new or already present (`public_auth_routes.rs:30-32`); portal login/refusal is one byte-identical body (`public_routes.rs:101-112`; probes `login_is_de_oracled_and_throttled`, host `login_enumeration_is_indistinguishable`). Residual on the LIVE bridge carried in §2 |
| PI-12 | Signup token parked in the session from any URL query, outliving the page | replaced-by-design | No server-side session store on the public surface; credentials are presented per-request (Authorization header / request body), never parked from a URL |
| PI-13 | OAuth audience never checked (confused deputy); nonce commented out | replaced-by-design | Closed under ADR-0024 via backbone-integrations v0.4.4: `aud == client_id` enforced, `nonce == state` enforced (the one OAuth generation; HMAC-bound state + PKCE) |
| PI-14 | OAuth token sent as a URL query param by default | replaced-by-design | ADR-0024 credential store: no token-in-URL anywhere in the generation |
| PI-15 | OAuth token stored plaintext, re-authenticates sessions by direct match | replaced-by-design | Credential store (envelope-encrypted, verbs only), one generation; no stored-plaintext token participates in any re-auth |
| PI-16 | OAuth state context injection + external redirect honoring; first-login user creation default | replaced-by-design | No implicit flow, no context merge from state, no first-login user creation (opt-out flag not needed — creation does not exist); OAuth-as-login filed on-demand (§2) |
| PI-17 | Synthetic logins `provider_<id>_user_<uid>` when the provider returns no email | replaced-by-design | Follows PI-16: no OAuth-driven user creation, so no synthetic login minting exists |
| PI-18 | Free signup ON by install data (b2c) — an infrastructure default | replaced-by-design | Signup is a kill-switchable policy, default OFF, nothing inserted at install (§1.1; probes `signup_is_off_by_default_and_fail_closed`, `the_switch_flips_both_ways`); installs are inert |
| PI-19 | TOTP secret lives in a raw-SQL column outside ORM field machinery | sapiens behavior | Sapiens owns the MFA stack (enrollment/verify, mfa sessions/devices, derived-key email codes) — secret handling stays in sapiens services, never in the public surface |
| PI-20 | Password change does NOT revoke other sessions (only 2FA changes do) | sapiens behavior | LANDED: password change and reset revoke every other session (and all device keys) — `auth_service.rs:990, :1076`; probes `password_change_revokes_other_sessions_and_device_keys`, `password_reset_revokes_device_keys` |
| PI-21 | Rate limit per-USER only (IP recorded but excluded), check-then-insert | sapiens behavior | LANDED: durable fixed-window buckets keyed per-identity AND per-IP (migration `auth_throttle_buckets`; `public_auth_routes.rs:182-196`); probes THR-1 (same identity, rotating IPs) and THR-2 (distinct identities, one IP) |
| PI-22 | Trusted devices finalize a session from a cookie with no code | sapiens behavior | LANDED: trusted devices are scoped, expiring, audit-trailed KEYS (`device_trust_keys` table), revoked wholesale on password change; not a silent cookie (§3 PI-43) |
| PI-23 | Email OTP sent on PAGE RENDER (GET) of the MFA page — a harassment vector | sapiens behavior | LANDED: the public router has zero GET paths that send or verify anything; codes are submitted (`POST /verify-email`) and the send paths are throttled (register/login/forgot/verify limits at `public_auth_routes.rs:57-72`) |
| PI-24 | Policy can force MFA (totp_mail) onto users who never enrolled | sapiens behavior | Enforcement vocabulary lives with the MFA stack; forced-OTP posture is declared there, not on the public forms |
| PI-25 | Password policy is one integer at one choke-point; two dead client knobs | sapiens behavior | Sapiens password-policy entities own the choke-point; the dead client-side meter knobs are simply not ported (no client policy UI ships) |
| PI-26 | The two 10-line auto_install policy bridges (portal/signup carry the number) | replaced-by-design | No auto_install concept exists; the portal reads policy through the declared credential port (the host installs `SapiensCredentialVerifier` into the portal's credential slot) — no bridge overlays |
| PI-27 | http_routing correction: it is NOT the multi-website resolver | WB-3-homed | The routing layer is the website-engine pass's surface (SEO/routing layer) |
| PI-28 | The 9-case language matcher (bot/POST/alias/redirect semantics) | WB-3-homed | Same — table-driven matcher lands with the website engine |
| PI-29 | Canonical-URL 301 + `//`-collapse rebuilt from route args | WB-3-homed | Same — SEO/redirect layer |
| PI-30 | Negative-id slugs retry `abs(id)` | WB-3-homed | Same — slug converter semantics |
| PI-31 | Error pages: rollback-first, fallback seam, 418 last resort | WB-3-homed | Same — error-page semantics |
| PI-32 | `url_rewrite` ormcached, POST match preferred | WB-3-homed | Same |
| PI-33 | `/website/translations` force-loads arbitrary module translations | WB-3-homed | Re-homed with the routing family; the mods passthrough is dropped by the already-ruled website-fence decision |
| PI-34 | Install-time `_post_init_hook` stamps `is_frontend` for upgrade renders | WB-3-homed | Install-time render machinery does not exist here (installs are inert; no server-side template rendering) — the guard is moot in the headless design |
| PI-35 | LDAP empty-password binds refused (RFC 4513) — keep the check FIRST on any port | fenced | `auth_ldap` fenced, on-demand; re-entry trigger = an enterprise LDAP requirement. On re-entry: parameterize the filter (never string-build) and keep the empty-password refusal first |
| PI-36 | LDAP `filter_format` escaping — the only injection fence | fenced | Same fence and same re-entry guidance |
| PI-37 | LDAP provisioning by raw-SQL login lookup + sudo template copy (normalization mismatch mints duplicates) | fenced | Same fence; a re-entry port must resolve logins through the identity service, not a side lookup |
| PI-38 | Passkey challenges session-resident, popped on use | fenced | `auth_passkey(_portal)` fenced, on-demand; re-entry trigger = a consumer demanding WebAuthn |
| PI-39 | `expected_origins` embeds hardcoded Android apk-key-hash fingerprints | fenced | Same fence; a re-entry port pins origins per release build |
| PI-40 | Passkey `public_key` storage bypasses ORM machinery (raw SQL both directions) | fenced | Same fence |
| PI-41 | Passkey own-only ir.rules keyed on `create_uid` | fenced | Same fence (ownership-via-issuer is the right shape to keep) |
| PI-42 | Session timeout enforcement (absolute age vs inactivity, distinct re-auth) | sapiens behavior | LANDED — §3: `SESSION_TIMEOUT_POLICY` absolute 30d + idle 7d on the last-activity basis, enforced at refresh |
| PI-43 | Trusted-device age clamped to the MFA timeout | sapiens behavior | LANDED — §3: clamp at `device_trust_key_service.rs:59-65` |
| PI-44 | The re-auth surface (`/auth-timeout/*`; second factor must differ) | sapiens behavior | Vocabulary landed (§3); the endpoints arm when a host mounts the public router — nothing answers today |

Census completeness: PI-01..PI-44 contiguous, each row above carries exactly
one disposition. The cohort's schema-hook annotations (PI-06/26/30/32 live
there, not in the prose) were read in full for this map.

## 5. Test obligations and where their evidence lives

The O-2 DoD names three proof families. Each is covered by probes in the
landed working trees, run with **exit codes verified, never output text**
(the lint-gate lesson): commands run with output redirected and `$?` checked.

1. **Throttle per-identity AND per-IP.**
   Sapiens `tests/auth_hardening_probes.rs`: THR-1 (one submitted address
   under rotating IPs trips the identity budget) and THR-2 (distinct
   addresses under one IP trip the IP budget), both over the durable bucket
   store. Portal `tests/probes/policy.rs::login_is_de_oracled_and_throttled`.
   Host live curve (read-only curl): 5 failures → 429 + `Retry-After: 28` on
   the 6th attempt, pre-handler.
2. **Kill-switch off-by-default + revocation.**
   Portal `tests/probes/policy.rs`: `signup_is_off_by_default_and_fail_closed`
   (no row AND disabled row both refuse), `the_switch_flips_both_ways`,
   `revoke_access_sweeps_everything`; `tests/probes/invites.rs`:
   `revocation_list_bites`, `invite_redeems_exactly_once`,
   `forwarded_link_refuses`, `expired_invites_refuse`. Host composed-mount
   probe: `policy_off_refuses_signup_with_typed_error` (403
   `portal_signup_closed`).
3. **Bearer rotation without breaking recipient HMACs.**
   Portal `tests/probes/credentials.rs`: `credentials_rotate_independently`
   (rotating the bearer leaves the recipient credential verifying; re-minting
   the recipient credential does not touch bearers) and
   `bearer_mint_verify_rotate_audited` (mint → verify → rotate → old bearer
   refused, audit rows written).

The composed-surface behavior beside the employee bridge is proven by the
host probes `composed_mount_is_reachable_beside_the_bridge_base` and
`unwired_credential_port_is_a_loud_503` (an unwired credential port answers
503, never a fake 401), plus the credential-verifier unit probes (unknown
email and wrong password both `Ok(false)`; soft-deleted account invisible;
unparseable hash is an error, not a forged refusal).

## 6. Update discipline

This doc is the O-2 closure record: when a disposition changes (a fence
re-enters, the deactivate/delete producers land, the public router mounts on
a host, the bridge residual closes), update the row and the interim posture
here in the same change — the map is only useful while it tells the truth.
