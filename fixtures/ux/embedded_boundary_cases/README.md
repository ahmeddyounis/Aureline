# Embedded-surface boundary fixtures

Worked fixtures for the contract frozen in
[`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../../../docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md)
and the schema at
[`/schemas/ux/embedded_surface_boundary.schema.json`](../../../schemas/ux/embedded_surface_boundary.schema.json).

The fixtures exist so docs/help panes, marketplace/account pages,
service dashboards, auth handoff sheets, extension-hosted surfaces,
support exports, and release-evidence audits can all compare against
the same host-owned boundary vocabulary instead of inventing local bars,
badges, or auth-fallback wording.

Each JSON file is a single record conforming to the schema. The fixture
may carry a `__fixture__` prelude for scenario description, but the
canonical vocabulary lives in the record fields.

## Cases

- [`docs_help_embedded_build_match.json`](./docs_help_embedded_build_match.json)
  — fresh embedded docs/help pane quoting ADR-0013
  `source_class`, `version_match_state`, and `freshness_class`
  into the host boundary card.
- [`marketplace_account_stale_scope.json`](./marketplace_account_stale_scope.json)
  — marketplace/account surface with explicit provider scope and actor
  chrome plus a `stale_snapshot` downgrade.
- [`service_dashboard_certificate_failed.json`](./service_dashboard_certificate_failed.json)
  — service dashboard that fails certificate/policy verification and
  withholds the embedded body while preserving recovery actions.
- [`auth_confirmation_device_code_fallback.json`](./auth_confirmation_device_code_fallback.json)
  — auth handoff cue using the system browser as the primary path and
  device code as the explicit degraded fallback.
- [`legacy_embedded_password_exception.json`](./legacy_embedded_password_exception.json)
  — embedded-auth exception-register row for a legacy provider that
  still requires an embedded password form under review and lower-trust
  cues.
- [`native_impersonation_denied_event.json`](./native_impersonation_denied_event.json)
  — audit event proving an embedded page was denied when it attempted
  to request a native-reserved host surface.
