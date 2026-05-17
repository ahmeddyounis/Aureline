# Embedded-Boundary Toolkit

The embedded-boundary toolkit is the reusable shell projection for
host-rendered chrome around embedded content. It consumes the existing
embedded-boundary audit page and emits render rows, event-log rows, and
support-export rows under one contract:

`shell:embedded_boundary_toolkit:v1`

The implementation lives in
[`crates/aureline-shell/src/embedded_boundary/mod.rs`](../../../crates/aureline-shell/src/embedded_boundary/mod.rs).
The headless inspector is
[`crates/aureline-shell/src/bin/aureline_shell_embedded_boundary_toolkit.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_embedded_boundary_toolkit.rs).

## Toolkit Rows

Each row exposes:

- owner, publisher/service, origin, and host/domain labels
- source, version, freshness, provider health, or offline posture summary
- data-boundary, boundary-state, permission, identity, and trust tokens
- browser fallback posture, handoff packet ref, and return target
- auth flow, provider/domain, reason code, and exception ref where applicable
- native approval owner and the full host-owned approval surface set
- support-export row identity and boundary event ids

The seeded row set covers:

| Surface | Required boundary truth |
| --- | --- |
| Docs/help | source, version/freshness posture, open-external path |
| Extension webview | extension owner, publisher, origin, permissions, browser fallback |
| Marketplace/account | account or org scope, service owner, network/session posture |
| Service dashboard | target/service identity, policy/offline state, external review path |
| Auth confirmation | system-browser default, provider/domain label, bounded fallback |

## Event Log

The toolkit emits boundary events for each row:

- `boundary_chrome_rendered`
- `browser_handoff_declared`
- `auth_handoff_default_recorded`
- `native_approval_fence_confirmed`
- `support_export_projected`

Every event records the embedded content owner, embedded origin, native
approval owner, browser handoff packet when present, support row id, and
privacy consequence label. Raw embedded body content, cookies, secrets,
and raw URLs are excluded.

## Native Approval Fence

Embedded content never owns or imitates these native approval surfaces:

- `product_security_messaging`
- `update_verification`
- `workspace_trust_elevation`
- `rollback_or_restore_confirmation`
- `ai_apply_review`
- `high_risk_approval_sheet`

The validator fails if any toolkit row drops a required surface, if an
auth row no longer defaults to system browser or device-code fallback, or
if support export drifts from the live row vocabulary.

## Fixtures And Verification

Fixtures live in
[`fixtures/ux/m3/embedded_boundary/`](../../../fixtures/ux/m3/embedded_boundary/).

```sh
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- validate
cargo test -p aureline-shell --test embedded_boundary_toolkit_fixtures
```

Regenerate fixtures from the inspector:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- page > fixtures/ux/m3/embedded_boundary/page.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- rows > fixtures/ux/m3/embedded_boundary/rows.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- events > fixtures/ux/m3/embedded_boundary/event_log.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- support-export > fixtures/ux/m3/embedded_boundary/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- defects > fixtures/ux/m3/embedded_boundary/defects.json
```
