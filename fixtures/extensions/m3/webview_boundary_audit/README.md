# Extension webview boundary audit fixtures

This directory contains the checked fixture corpus for extension-owned
webviews and embedded surfaces. The records are generated from the Rust
seed in `crates/aureline-extensions/src/webview_boundary/`.

All generated records carry the shared contract ref
`extensions:webview_boundary_beta:v1` so product rows, headless output,
support exports, and the generated report pivot to the same row ids.

## Index

| Fixture | Coverage |
| --- | --- |
| `inputs.json` | Seed inputs for six extension-contributed surfaces: local webview panel, cross-origin hosted dashboard, cached documentation provider pane, provider auth checkpoint, browser-runtime bridge, and policy-blocked hosted account surface. |
| `rows.json` | Product-visible rows with owner, publisher, origin, permission state, trust class, handoff posture, native-approval boundary, and visible boundary finding refs. |
| `support_rows.json` | Metadata-safe support rows paired 1:1 with product rows. |
| `defects.json` | Validator output for the seeded corpus. The expected value is `[]`. |
| `audit_packet.json` | Full packet with summary, rows, support rows, and defects. |
| `support_export.json` | Support-export wrapper projected from the packet with raw private material excluded. |

## Fixture rules

- Every product row must show extension name, publisher, origin, boundary
  state, trust class, permission state, open-in-browser posture, support
  export affordance, and current scope in host chrome.
- Risky or provider-owned rows must use `system_browser_first`,
  `device_code_fallback_offered`, `external_open_blocked_by_policy`, or
  `external_open_unavailable_offline`.
- Rows using `system_browser_first` must quote a typed browser handoff
  packet ref.
- Host trust class and embedded-content trust class must match.
- High-risk approvals remain `host_owned_native_surface`; embedded
  content cannot mint permission sheets, trust prompts, rollback
  confirmations, or destructive approvals.
- Support rows must mirror product rows on owner, publisher, origin,
  boundary state, trust class, permission state, handoff posture, target
  scope, and visible boundary finding refs.

## Regenerate

```text
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- inputs > fixtures/extensions/m3/webview_boundary_audit/inputs.json
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- packet > fixtures/extensions/m3/webview_boundary_audit/audit_packet.json
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- rows > fixtures/extensions/m3/webview_boundary_audit/rows.json
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- support-rows > fixtures/extensions/m3/webview_boundary_audit/support_rows.json
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- defects > fixtures/extensions/m3/webview_boundary_audit/defects.json
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- support-export > fixtures/extensions/m3/webview_boundary_audit/support_export.json
```

## Verification

```text
cargo test -p aureline-extensions webview_boundary
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- validate
```
