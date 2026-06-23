# Embedded service-dashboard / auth-handoff surfaces — evidence companion

Human-readable companion to
[`/fixtures/ops/m5-embedded-dashboards/canonical_surfaces.json`](../../fixtures/ops/m5-embedded-dashboards/canonical_surfaces.json)
and its boundary schema
[`/schemas/ops/m5-embedded-dashboards.schema.json`](../../schemas/ops/m5-embedded-dashboards.schema.json).
It gives reviewers the frozen card, origin, device-permission, handoff, and
invariant tables without reading the JSON. The contract narrative lives in
[`/docs/ops/m5-embedded-dashboards.md`](../../docs/ops/m5-embedded-dashboards.md).

- Set id: `m5-embedded-dashboards:set:0001`
- Record kind: `m5_embedded_surface_set`
- Bound matrix: `fixtures/ops/m5-operator-surfaces/canonical_matrix.json`
  (`m5_operator_surface_matrix`), embedded-boundary family
  `operator_surface.embedded_boundary_state`
- Surfaces: 8 · Invariants: 13
- Kinds exercised: service dashboard, provider page, device capture, browser auth
  handoff, device-code auth handoff

## Cards, origin, and computed state

Each card binds the embedded-boundary matrix family by that matrix's own surface id
and points at one canonical `aureline://` object. `effective_state` is the computed
no-silent-green downgrade of the displayed state and the origin-bar freshness, so a
`clear` surface with stale content is never reported `clear`.

| Card | Kind | Owner | Permission | Freshness | Displayed → Effective | Native impersonation |
| --- | --- | --- | --- | --- | --- | --- |
| 0001 | service_dashboard | first_party_webview | sandboxed | fresh | clear → clear | false |
| 0002 | service_dashboard | extension_provided | scoped_granted | recent | clear → clear | false |
| 0003 | provider_page | third_party_provider | broad_granted | stale | clear → **unconfirmed** | false |
| 0004 | provider_page | **unknown_origin** | requires_review | fresh | **boundary_drift_recheck_required** | false |
| 0005 | device_capture_surface | first_party_native_chrome | scoped_granted | fresh | clear → clear | false |
| 0006 | browser_auth_handoff | third_party_provider | sandboxed | fresh | embedded_boundary_handoff | false |
| 0007 | device_code_auth_handoff | third_party_provider | sandboxed | fresh | embedded_boundary_handoff | false |
| 0008 | device_code_auth_handoff | third_party_provider | sandboxed | fresh | **blocked** (code expired) | false |

Every embedded webview names the `no_native_approval` capability limitation; cards
0001–0004 also offer an open-in-browser exit.

## Origin bars and capability limitations

- **0001 — Service health dashboard** — Aureline webview; limitations:
  no_native_approval, read_only_content, network_scoped_to_origin; open-in-browser →
  system_browser.
- **0002 — CI pipeline dashboard** — extension `aureline://extension/pipeline-dashboard`;
  limitations: no_native_approval, no_credential_access, no_local_command_execution;
  open-in-browser → system_browser.
- **0003 — Provider billing console** — third-party provider; broad grant but stale
  content downgrades the headline to unconfirmed; open-in-browser →
  vendor_portal_in_browser.
- **0004 — Unverified embedded page** — unknown origin; held read-only and routed to
  a boundary recheck; open-in-browser → system_browser to verify.

## Device-permission rows (card 0005)

| Permission | Actor | Processing | Retention | Revoke action | Opens system settings |
| --- | --- | --- | --- | --- | --- |
| screen_capture | Aureline session capture | local_only | local_session_only | revoke_and_open_system_settings | yes |
| microphone | Aureline session capture | mixed_local_then_provider | provider_retained | open_system_settings | yes |

Each row names what stays local if revoked: stopping screen capture leaves the rest
of the bundle intact; muting the microphone keeps the screen recording running.

## Auth handoff cards

| Card | Reason | Target | Fallback | Code shown | Expiry | Return path |
| --- | --- | --- | --- | --- | --- | --- |
| 0006 | claimed_identity_auth | system_browser | system_browser_opened | no | — | return to workspace on completion |
| 0007 | provider_auth_required | device_code_verification | polling_for_completion | yes (`short_user_code`) | 2026-06-22T00:15:00Z | poll, then return here |
| 0008 | provider_auth_required | device_code_verification | manual_code_entry_available | yes (`short_user_code`) | 2026-06-21T23:50:00Z (**expired**) | request a fresh code |

No handoff is exposed behind a generic Continue; each names its reason, target,
fallback, and return path. The verification-code *value* never crosses the boundary
— only its display class and expiry. Card 0008's expired code forces the `blocked`
state and offers a fresh code rather than a silent retry.

## Invariants

All 13 invariants are computed from the built data and frozen as `holds: true`:

- `embedded_dashboards.surface_binding`
- `embedded_dashboards.canonical_object_identity`
- `embedded_dashboards.origin_owner_visible`
- `embedded_dashboards.no_native_surface_impersonation`
- `embedded_dashboards.capability_limitations_named`
- `embedded_dashboards.device_permissions_disclose_truth`
- `embedded_dashboards.capture_surfaces_carry_permissions`
- `embedded_dashboards.handoff_reason_and_return_visible`
- `embedded_dashboards.device_code_shows_code_and_expiry`
- `embedded_dashboards.effective_state_computed`
- `embedded_dashboards.origin_and_capability_exportable`
- `embedded_dashboards.all_kinds_present`
- `embedded_dashboards.stable_ids_unique`

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw payloads,
verification-code values, or absolute paths — only opaque `aureline://` object
handles, repo-relative refs, stable tokens, exact timestamps, and short reviewable
sentences. `is_support_export_safe()` enforces the boundary, so device /
browser-auth boundary state exports cleanly into support / export packets and
companion / help surfaces without losing origin or capability-limitation detail.
