# Embedded-surface state matrix fixtures

Worked fixtures for the contract frozen in
[`/docs/ux/embedded_surface_state_matrix.md`](../../../docs/ux/embedded_surface_state_matrix.md)
and the schema at
[`/schemas/ux/embedded_surface_state.schema.json`](../../../schemas/ux/embedded_surface_state.schema.json).

The fixtures exist so docs/help panes, marketplace/account surfaces,
service dashboards, and extension-owned webviews can validate that they
use **typed, non-collapsible** embedded-surface states and that any
browser fallback preserves object identity and return path.

Each YAML file is a single `embedded_surface_state_record`. The
`__fixture__` prelude is reviewer metadata; the canonical vocabulary
lives in the record itself.

## Cases

- [`docs_help_fresh_live.yaml`](./docs_help_fresh_live.yaml)
  - fresh/live embedded docs/help surface state with no fallback required.
- [`docs_help_cached_mirrored.yaml`](./docs_help_cached_mirrored.yaml)
  - cached/mirrored docs state requiring explicit non-live cues and a
    browser handoff that preserves identity and return path.
- [`marketplace_account_policy_limited.yaml`](./marketplace_account_policy_limited.yaml)
  - policy-limited marketplace/account state with explicit policy-limited
    disclosure and typed fallback posture.
- [`service_dashboard_certificate_trust_blocked.yaml`](./service_dashboard_certificate_trust_blocked.yaml)
  - certificate/trust-blocked service dashboard state with host-native
    inspection fallback.
- [`extension_webview_cross_origin_limited.yaml`](./extension_webview_cross_origin_limited.yaml)
  - cross-origin-limited extension-owned webview state with explicit
    capability limitation and system browser fallback.
- [`marketplace_account_offline_snapshot.yaml`](./marketplace_account_offline_snapshot.yaml)
  - offline snapshot state with explicit non-live export posture and a
    local-inspect/export fallback target.

