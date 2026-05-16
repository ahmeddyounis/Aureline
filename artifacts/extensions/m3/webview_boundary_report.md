# Extension webview boundary report

Generated from `fixtures/extensions/m3/webview_boundary_audit/audit_packet.json`.

## Summary

| Metric | Value |
| --- | ---: |
| Product rows | 6 |
| Support rows | 6 |
| Conformant rows | 6 |
| Rows requiring safe browser baseline | 5 |
| Rows satisfying safe browser baseline | 5 |
| Rows with trust-class parity | 6 |
| Rows with host-owned native approval | 6 |
| Defects | 0 |

## Audited rows

| Row | Surface | Boundary | Browser posture | Trust | Native approval |
| --- | --- | --- | --- | --- | --- |
| `dev.aureline.samples/wasm-notes` README preview | Extension webview panel | `live_verified` | `system_browser_first` | `trusted` | host-owned |
| `com.acme.cloud-tools` status dashboard | Hosted dashboard | `cross_origin_limited` | `system_browser_first` | `limited` | host-owned |
| `org.python.docs` cached provider pane | Documentation provider pane | `stale_snapshot` | `external_open_unavailable_offline` | `stale` | host-owned |
| `org.gitforge.review` auth checkpoint | Provider auth surface | `live_verified` | `system_browser_first` | `limited` | host-owned |
| `dev.browser.inspect` runtime storage bridge | Browser-runtime bridge | `stale_snapshot` | `system_browser_first` | `stale` | host-owned |
| `com.vendor.billing` hosted account dashboard | Hosted dashboard | `policy_blocked` | `external_open_blocked_by_policy` | `policy_blocked` | host-owned |

## Findings

- Owner, publisher, origin, boundary state, trust class, permission
  state, open-in-browser posture, support-export affordance, and
  current scope are present in host chrome on every row.
- Risky/provider-owned rows use a safe browser baseline:
  `system_browser_first`, `external_open_unavailable_offline`, or
  `external_open_blocked_by_policy`.
- Rows using `system_browser_first` quote a typed browser handoff packet
  ref.
- Host chrome trust class matches the embedded-content trust class on
  every row.
- Support rows mirror product rows on visible boundary finding refs and
  export no raw private material.
- No row grants unbounded host authority or moves native approvals into
  embedded content.

## Evidence refs

- Schema: `schemas/extensions/webview_boundary_audit.schema.json`
- Docs: `docs/extensions/m3/webview_boundary_beta.md`
- Fixture packet: `fixtures/extensions/m3/webview_boundary_audit/audit_packet.json`
- Support export: `fixtures/extensions/m3/webview_boundary_audit/support_export.json`

## Regeneration

```text
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- packet > fixtures/extensions/m3/webview_boundary_audit/audit_packet.json
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- support-export > fixtures/extensions/m3/webview_boundary_audit/support_export.json
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- validate
```
