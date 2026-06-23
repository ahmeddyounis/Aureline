# Embedded service-dashboard / auth-handoff boundary contract

This document freezes Aureline's **embedded boundary** surfaces: the
service-dashboard / webview **origin bars**, the **device-permission rows**, and
the browser / device-code **auth handoff cards** that let an operator tell who owns
an embedded surface, what is native product chrome versus provider / webview
chrome, what device or browser permissions are in play, and when Aureline
intentionally hands work to the system browser or a device-code flow. Where the
[operator-surface matrix](./m5-operator-surfaces.md) freezes the surface
*families* — including the embedded provider/auth boundary — and the
[maintenance windows](./m5-maintenance-windows.md) and
[response panes](./m5-response-panes.md) lanes build planned-operation and
guided-response surfaces, this contract builds the embedded boundary an operator
sees *before* trusting a dashboard or completing a handoff. Each card binds the
embedded-boundary matrix family by that matrix's own surface id, so it renders the
shared surface contract rather than a parallel truth model.

The goal is the one in the spec: keep embedded service dashboards and auth handoffs
boundary-honest so no embedded content masquerades as a native approval, update, or
product-security surface, and no browser / device-code / auth boundary hides behind
a generic "Continue". This contract pins three things:

1. **An origin bar.** Every embedded surface names its `owner_class`
   (`first_party_native_chrome`, `first_party_webview`, `extension_provided`,
   `third_party_provider`, or `unknown_origin`), an opaque owner label and
   `aureline://` origin handle, the optional providing extension, the
   `permission_state`, the named `capability_limitations`, an open-in-browser
   action, a freshness stamp, and the `required_visible_language` shown verbatim.
   `native_surface_impersonation` is always `false`.
2. **Device-permission rows.** A surface that uses a device capability carries rows
   that name the `actor`, the `processing_class` (`local_only`,
   `provider_processed`, or `mixed_local_then_provider`), the `retention_class` and
   a storage / retention note, a revoke / open-system-settings action, and the
   local-continuity posture if the permission is revoked.
3. **Browser / device-code auth handoff.** A surface that hands auth out names the
   `reason`, the `target` (system browser, vendor portal, provider console, or
   device-code verification), the `fallback`, the verification-code class and
   expiry, and the `return_path` — never behind a generic Continue. An expired or
   no-fallback handoff is in the `blocked` state, not retried silently.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update in
the same change.

## Companion artifacts

- [`/schemas/ops/m5-embedded-dashboards.schema.json`](../../schemas/ops/m5-embedded-dashboards.schema.json)
  — boundary schema for `m5_embedded_surface_set`.
- [`/fixtures/ops/m5-embedded-dashboards/canonical_surfaces.json`](../../fixtures/ops/m5-embedded-dashboards/canonical_surfaces.json)
  — the published canonical set; the freeze gate asserts the in-code builder equals
  it byte-for-byte.
- [`/artifacts/ops/m5-embedded-dashboards.md`](../../artifacts/ops/m5-embedded-dashboards.md)
  — the human-readable companion (card, origin, device-permission, and handoff
  tables).
- `crates/aureline-support/src/m5_embedded_dashboards/` — the builder, the
  surface-kind and origin-owner model, the capability-limitation model, the
  device-permission processing/retention/revoke model, the auth-handoff model, the
  computed displayed and effective state, validation, and the human-readable
  projection.
- `cargo run -p aureline-support --example dump_m5_embedded_dashboards` — the
  headless emitter (JSON, or `-- --lines` for the projection).

## Surface kinds and origin bars

A card is exactly one `kind`. The kind selects which sub-blocks it carries:

| Kind | Origin bar | Device rows | Handoff card | Open-in-browser |
| --- | --- | --- | --- | --- |
| `service_dashboard` | yes | — | — | required |
| `provider_page` | yes | — | — | required |
| `device_capture_surface` | yes | yes | — | optional |
| `browser_auth_handoff` | yes | — | yes | optional |
| `device_code_auth_handoff` | yes | — | yes | optional |

Every embedded webview (`service_dashboard`, `provider_page`,
`device_capture_surface`) names at least one capability limitation, and always
declares the `no_native_approval` limitation, so a webview never reads as having the
same authority as native product chrome. A service dashboard or provider page also
offers an open-in-browser exit; a device-capture surface is native chrome that
happens to use device permissions, so it carries no open-in-browser action.

An `unknown_origin` surface is held read-only and mapped to
`boundary_drift_recheck_required`, so an unverifiable origin can never present as
native chrome or a trusted provider.

## Computed displayed and effective state

`displayed_state` is computed from the kind, origin owner, live-versus-snapshot
posture, and whether a handoff is blocked: a blocked or expired handoff is
`blocked`; an `unknown_origin` requires a boundary recheck; a snapshot-only surface
is `imported_snapshot_no_live`; an auth handoff is `embedded_boundary_handoff`;
everything else is `clear`. `effective_state` is the shared no-silent-green
downgrade of the displayed state and the origin-bar freshness, so a `clear` provider
page whose content is `stale` is downgraded to `unconfirmed` rather than reading as
a confirmed green.

## Device-permission rows

A device-capture surface carries one row per device capability (`camera`,
`microphone`, `screen_capture`, `clipboard`, `location`, `notifications`). Each row
names the `actor` using the capability, where the captured data is processed
(`processing_class`), how long it is kept (`retention_class` plus a retention note),
the revoke action (`revoke_in_aureline`, `open_system_settings`, or
`revoke_and_open_system_settings` — `opens_system_settings` is computed from it),
and what stays local if the permission is revoked. This makes the device boundary
concrete: an operator sees the actor, the local-versus-provider processing class,
and the storage/retention posture before trusting a capture.

## Browser / device-code auth handoff

An auth handoff card prefers an external, attributable surface (`prefers_external`
is always `true`) and is never hidden behind a generic Continue
(`hidden_behind_generic_continue` is always `false`). It names:

- the `reason` (`claimed_identity_auth`, `provider_auth_required`,
  `high_risk_approval`, `provider_managed_page`, or `policy_requires_external`) and
  a reason note,
- the `target` (`system_browser`, `vendor_portal_in_browser`,
  `provider_console_in_browser`, or `device_code_verification`),
- the `fallback` (`system_browser_opened`, `device_code_fallback_available`,
  `manual_code_entry_available`, `polling_for_completion`, or `no_fallback_blocked`),
- for a device-code flow, the `code_display_class` and `code_expiry_at` — the code
  value itself never crosses the boundary,
- and the `return_path` plus an `aureline://` return anchor.

An expired code (`code_expired: true`) or a `no_fallback_blocked` fallback forces the
`blocked` displayed state and offers a fresh code rather than a silent retry.

## Export safety

The record is support-export safe: it carries no endpoint URLs, hostnames,
credentials, raw payloads, verification-code values, or absolute paths — only opaque
`aureline://` object handles, repo-relative refs, stable tokens, exact timestamps,
and short reviewable sentences. `is_support_export_safe()` enforces the boundary,
and `validate()` re-checks it, so device / browser-auth boundary state exports
cleanly into support / export packets and companion / help surfaces without losing
origin or capability-limitation detail.

## Consumers

Desktop shell UI, CLI/headless inspect, the incident workspace, the admin queue,
support export, managed-service surfaces, and the companion/browser surface all
render this one set instead of restating embedded-surface, device-permission, or
auth-handoff truth by hand. The checked-in descriptors here are the canonical M5
source for embedded provider/auth boundary truth; downstream service-health,
help/About, support, companion, and release surfaces should consume them directly.
