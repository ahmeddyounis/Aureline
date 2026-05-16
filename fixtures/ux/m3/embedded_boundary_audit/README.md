# Embedded-surface boundary audit (beta) fixture corpus

Reviewable fixtures for the beta embedded-surface boundary audit
projection that lives in
[`crates/aureline-shell/src/embedded_boundary_audit/mod.rs`](../../../../crates/aureline-shell/src/embedded_boundary_audit/mod.rs).

Each JSON file is a literal projection of the seeded
`EmbeddedBoundaryAuditPage` produced by the headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_embedded_boundary_audit.rs`](../../../../crates/aureline-shell/src/bin/aureline_shell_embedded_boundary_audit.rs)).
The inspector is the only mint-from-truth path for these fixtures, so
the checked-in JSON cannot drift from the Rust types.

All records carry the shared contract ref
`shell:embedded_boundary_audit_beta:v1` so shell UI rows, headless
CLI rows, and support-export rows pivot to the same `row_id` and
`case_id`.

## Index

| Fixture | Coverage |
| --- | --- |
| [`rows.json`](./rows.json) | Audited live rows for docs/help (live verified, embedded-build match), extension webview (cross-origin limited), marketplace/account (stale provider session), service dashboard (policy blocked), and auth confirmation (system-browser first). |
| [`support_rows.json`](./support_rows.json) | Support-export rows aligned 1:1 with the live rows by `row_id`. They reuse the same surface-family, boundary-state, permission-class, fallback-posture, owner-label, host-or-domain-label, identity-mode, trust-state, and handoff-packet-ref tokens. |
| [`defects.json`](./defects.json) | Typed defect list emitted by the validator. Seeded value is `[]`. |
| [`page.json`](./page.json) | Full beta page with aggregate summary banner (surface family, boundary state, permission class, and fallback posture coverage), the live rows, the support rows, and the defects. |
| [`support_export.json`](./support_export.json) | Support-export wrapper that quotes the page and a metadata-safe defect roll-up. Raw private material is excluded by construction. |

## Fixture rules

- Every record carries a stable `case_id`, `row_id`, and the shared
  contract ref `shell:embedded_boundary_audit_beta:v1`; record kinds
  are stable Rust constants.
- Every row quotes owner label, owner class, publisher/service label,
  publisher/service class, origin label, origin host/domain, origin
  class, and origin verification token. Hidden owner/origin is a
  contract bug the validator rejects.
- Every row quotes a boundary-state token, a permission-class token,
  an identity-mode token, and a trust-state token from the closed
  vocabulary. Hue-only or label-only trust treatments are rejected.
- Every row quotes a browser-fallback posture token and a fallback
  target class token; when an `open_in_system_browser` action is
  offered the row also quotes the handoff packet ref.
- **Identity & risky web rows** (auth confirmation, marketplace/
  account, extension webview, service dashboard) MUST quote one of
  the safe-baseline postures: `system_browser_first`,
  `device_code_fallback_offered`, `external_open_blocked_by_policy`,
  or `external_open_unavailable_offline`. The validator rejects
  `browser_fallback_not_applicable` on those rows.
- Every row keeps all six native-reserved surfaces
  (`product_security_messaging`, `update_verification`,
  `workspace_trust_elevation`, `rollback_or_restore_confirmation`,
  `ai_apply_review`, `high_risk_approval_sheet`) on
  `native_reserved_surface_tokens`.
- Auth-confirmation rows MUST declare an `auth_flow_class_token`
  other than `not_applicable`. `embedded_password_exception` flows
  MUST also name an `auth_exception_id_ref`.
- Support rows MUST agree with the matching live row on
  `surface_family_token`, `boundary_state_token`,
  `permission_class_token`, `browser_fallback_posture_token`,
  `fallback_target_class_token`, `owner_label`,
  `host_or_domain_label`, `identity_mode_token`, `trust_state_token`,
  and `browser_handoff_packet_ref`. Drift is a contract bug the
  validator rejects.

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- page          > fixtures/ux/m3/embedded_boundary_audit/page.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- rows          > fixtures/ux/m3/embedded_boundary_audit/rows.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- support-rows  > fixtures/ux/m3/embedded_boundary_audit/support_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- defects       > fixtures/ux/m3/embedded_boundary_audit/defects.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- support-export > fixtures/ux/m3/embedded_boundary_audit/support_export.json
```

## Verification

```sh
cargo test -p aureline-shell --test embedded_boundary_audit_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- validate
```
