# Embedded-surface boundary fixtures

Worked fixtures for two related contracts:

1. The upstream record contract frozen in
   [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../../../docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md)
   with schema at
   [`/schemas/ux/embedded_surface_boundary.schema.json`](../../../schemas/ux/embedded_surface_boundary.schema.json).
   These cases use `.json` files and exercise the full
   `embedded_surface_boundary_record`,
   `embedded_auth_exception_record`, and
   `embedded_surface_boundary_audit_event_record` family.
2. The render-side boundary-card contract frozen in
   [`/docs/ux/embedded_surface_boundary_cards.md`](../../../docs/ux/embedded_surface_boundary_cards.md)
   with schema at
   [`/schemas/ux/embedded_boundary_card.schema.json`](../../../schemas/ux/embedded_boundary_card.schema.json).
   These cases use `.yaml` files and exercise the
   `embedded_boundary_card_record` projection that host shells render
   next to every embedded surface.

Both contracts share their closed vocabularies; the YAML and JSON cases
can quote the same `surface_id_ref` so the upstream record and the
render-side card stay attributable to the same surface family.

The fixtures exist so docs/help panes, marketplace/account pages,
service dashboards, auth handoff sheets, extension-hosted surfaces,
support exports, and release-evidence audits can all compare against
the same host-owned boundary vocabulary instead of inventing local bars,
badges, or auth-fallback wording.

Each file is a single record conforming to its schema. Fixtures may
carry a `__fixture__` prelude for scenario description, but the
canonical vocabulary lives in the record fields.

## Upstream record cases (`.json`)

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

## Boundary-card render cases (`.yaml`)

- [`docs_help_live_verified_card.yaml`](./docs_help_live_verified_card.yaml)
  — embedded docs/help card with `live_verified` state, exact build
  match, and inspect/copy partition.
- [`docs_help_stale_snapshot_card.yaml`](./docs_help_stale_snapshot_card.yaml)
  — embedded docs/help card with `stale_snapshot` state for a mirrored
  vendor docs pack that has drifted off the running build, including a
  snapshot-age label and a `system_browser_first` upstream-docs
  handoff.
- [`docs_help_external_open_only_card.yaml`](./docs_help_external_open_only_card.yaml)
  — embedded docs/help card narrowed to `external_open_only` for a
  policy-disabled curated knowledge pack, preserving source / version
  / freshness chrome and routing to a `system_browser_handoff_packet`
  fallback target.
- [`marketplace_account_offline_snapshot_card.yaml`](./marketplace_account_offline_snapshot_card.yaml)
  — marketplace/account card narrowed to `offline_snapshot` with an
  `external_open_unavailable_offline` browser-fallback posture.
- [`service_dashboard_certificate_failed_card.yaml`](./service_dashboard_certificate_failed_card.yaml)
  — service-dashboard card with `certificate_failed` state, withheld
  body, and a host-native inspect-certificate fallback.
- [`service_dashboard_policy_blocked_card.yaml`](./service_dashboard_policy_blocked_card.yaml)
  — service-dashboard card with `policy_blocked` state where managed
  policy denies the embedded body. The card stays visible with an
  `external_open_blocked_by_policy` posture and a
  `host_native_review_or_approval` fallback target.
- [`auth_confirmation_system_browser_first_card.yaml`](./auth_confirmation_system_browser_first_card.yaml)
  — auth-confirmation card using `system_browser_first` posture and
  `system_browser` flow class.
- [`auth_confirmation_password_exception_card.yaml`](./auth_confirmation_password_exception_card.yaml)
  — auth-confirmation card backed by an active embedded-password
  exception register row, `embedded_password_exception` flow class,
  and the lower-trust capability limitation.
- [`extension_hosted_cross_origin_limited_card.yaml`](./extension_hosted_cross_origin_limited_card.yaml)
  — extension-hosted card with `cross_origin_limited` state and a
  `system_browser_first` fallback posture.
- [`extension_hosted_unsupported_capability_card.yaml`](./extension_hosted_unsupported_capability_card.yaml)
  — extension-hosted card whose embedded body cannot honor an
  unsupported capability (workspace trust elevation), naming the exact
  missing capability and routing to a `host_native_review_or_approval`
  fallback target.
- [`marketplace_account_external_open_only_card.yaml`](./marketplace_account_external_open_only_card.yaml)
  — marketplace/account card narrowed to `external_open_only` with a
  `device_code_companion_card` fallback target.

## Audit packet and owner/origin chrome review

The audit packet at
[`/artifacts/ux/embedded_surface_audit_packet.md`](../../../artifacts/ux/embedded_surface_audit_packet.md)
and the owner/origin chrome review at
[`/artifacts/ux/owner_origin_chrome_review.yaml`](../../../artifacts/ux/owner_origin_chrome_review.yaml)
score real surfaces against this corpus. Their boundary-state coverage
matrix and per-surface-family review rows cite the fixtures above by
file path.
