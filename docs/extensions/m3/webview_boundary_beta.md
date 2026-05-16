# Extension webview boundary audit (beta)

This page documents the extension-specific boundary audit for webviews,
hosted dashboards, provider-auth checkpoints, documentation panes, and
browser-runtime bridges.

The audit exists to prevent extension-contributed embedded content from
becoming an unlabeled browser, provider, or approval boundary. Extension
surfaces may render rich web content, but the host owns the chrome that
identifies who owns the surface, which origin is active, what permission
state applies, which trust class is current, and where browser handoff
or support export goes.

## Canonical records

- Rust source:
  `crates/aureline-extensions/src/webview_boundary/`
- Cross-tool schema:
  `schemas/extensions/webview_boundary_audit.schema.json`
- Fixture corpus:
  `fixtures/extensions/m3/webview_boundary_audit/`
- Generated packet mirror:
  `artifacts/extensions/m3/webview_boundary_audit/`
- Human-readable report:
  `artifacts/extensions/m3/webview_boundary_report.md`

Every record carries `extensions:webview_boundary_beta:v1`. Product
rows, support rows, and docs all cite the same row ids and visible
boundary finding refs.

## Required host chrome

Each audited row must render these facts outside the embedded body:

| Required fact | Why it is host-owned |
| --- | --- |
| Extension name and publisher | Prevents anonymous or impersonating embedded pages. |
| Origin label and host/domain | Keeps browser, provider, and cross-origin boundaries visible. |
| Boundary state | Distinguishes live verified, cross-origin limited, stale, policy blocked, and certificate-failed surfaces. |
| Trust class | Keeps the embedded body's trust claim aligned with product vocabulary. |
| Permission state | Shows whether the row is inspect-only, browser-only, native-step-up, or mutation-capable with host review. |
| Open-in-browser posture | Makes system-browser, device-code, policy-blocked, and offline-blocked paths explicit. |
| Support/export affordance | Lets support read the same boundary findings visible in product. |
| Current scope | Names the workspace, provider account, target, or policy scope behind the row. |

Theme, zoom, density, focus, reduced-motion, and contrast inheritance
must be disclosed as `inherits`, `partial`, or `does_not_inherit`.
Undisclosed parity is a defect.

## Safe browser baseline

Risky or provider-owned rows are those with active content, credentials,
mutation capability, hosted dashboard authority, provider auth, runtime
inspection authority, or publisher-hosted origin. Those rows must use
one of the safe baseline postures:

- `system_browser_first`
- `device_code_fallback_offered`
- `external_open_blocked_by_policy`
- `external_open_unavailable_offline`

Rows using `system_browser_first` must quote a browser handoff packet
ref. Policy-blocked and offline-blocked rows still render the external
path as blocked or unavailable so the user can tell why the embedded
surface did not navigate.

## Native approval boundary

Extension webviews must not imitate product-native permission sheets,
workspace trust prompts, rollback confirmations, AI apply review, or
destructive confirmation dialogs. The audit requires
`host_owned_native_surface` on every row. Any
`embedded_surface_attempted_approval` row is refused.

## Support export parity

The support export mirrors product rows without raw URLs containing
tokens, cookies, DOM text, storage values, screenshots, request bodies,
or private payloads. Support rows must match product rows on:

- extension id, extension name, and publisher label;
- surface family and origin host label;
- boundary state, trust class, permission state, handoff posture, and
  fallback target;
- current scope label;
- visible boundary finding refs and row defect tokens.

## Seeded coverage

The checked packet covers six conformant rows:

| Surface | Boundary state | Browser posture | Trust class |
| --- | --- | --- | --- |
| Local extension webview panel | `live_verified` | `system_browser_first` | `trusted` |
| Cross-origin hosted dashboard | `cross_origin_limited` | `system_browser_first` | `limited` |
| Cached documentation provider pane | `stale_snapshot` | `external_open_unavailable_offline` | `stale` |
| Provider auth checkpoint | `live_verified` | `system_browser_first` | `limited` |
| Browser-runtime bridge | `stale_snapshot` | `system_browser_first` | `stale` |
| Policy-blocked hosted account surface | `policy_blocked` | `external_open_blocked_by_policy` | `policy_blocked` |

The seeded summary has zero defects, five risky/provider-owned rows
that satisfy the safe browser baseline, six rows with trust-class
parity, and six rows with host-owned native approval authority.

## Headless consumer

```text
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- packet
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- rows
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- support-rows
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- defects
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- support-export
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- validate
```

`validate` fails if owner/origin chrome disappears, a risky row drops
the safe browser baseline, a system-browser row lacks a handoff packet,
host and embedded trust classes drift, native approval moves into the
embedded body, host authority becomes unbounded, inheritance disclosure
is missing, or support export parity breaks.

## How to verify

```text
cargo test -p aureline-extensions webview_boundary
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- validate
```
