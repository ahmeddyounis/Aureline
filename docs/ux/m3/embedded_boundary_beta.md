# Embedded-surface boundary audit (beta)

The beta embedded-surface boundary audit promotes the existing alpha
boundary card seed (docs/help, extension webview, marketplace/account)
to a page-level projection that covers all five claimed embedded
surface families (docs/help, extension webview, marketplace/account,
service dashboard, auth confirmation). The page is the reviewer
entrypoint for the M3 promise that owner-origin chrome and
browser-handoff rules remain real on every embedded surface, and that
support exports use the same vocabulary as the live UI.

The projection lives in
[`crates/aureline-shell/src/embedded_boundary_audit/mod.rs`](../../../crates/aureline-shell/src/embedded_boundary_audit/mod.rs).
It does not re-derive the per-card boundary truth — that still comes
from
[`crates/aureline-shell/src/embedded/boundary_card.rs`](../../../crates/aureline-shell/src/embedded/boundary_card.rs)
and
[`crates/aureline-shell/src/embedded/boundary_alpha.rs`](../../../crates/aureline-shell/src/embedded/boundary_alpha.rs).
The beta page projects, on every claimed row, the audit promises a
beta reviewer needs to inspect.

## Contract surface

The beta projection ships five record kinds, all under the shared
contract ref `shell:embedded_boundary_audit_beta:v1`:

- `shell_embedded_boundary_audit_beta_row_record` — one audited row
  per claimed embedded surface scenario. Each row carries a stable
  `case_id` and `row_id`, the surface family, the source card id,
  owner / publisher / origin disclosure, trust class disclosure
  (boundary state, data boundary class, permission class, identity
  mode, trust state), handoff rules disclosure (browser fallback
  posture, fallback target class, browser handoff packet ref, return
  target label), the optional auth-handoff block (flow class,
  provider domain, exception ref), the optional provider block (class,
  health, scope label), the host-owned native-reserved surface set,
  the closed audit axes the row promises, and a plain-language
  reviewer summary.
- `shell_embedded_boundary_audit_beta_support_row_record` — export-
  safe support row aligned 1:1 with the live row by `row_id`. The
  support row reuses the same closed-vocabulary tokens the live row
  paints; drift is a contract bug.
- `shell_embedded_boundary_audit_beta_defect_record` — typed defect
  emitted by the validator when a row drops a required field, drifts
  vocabulary across live & support rows, weakens the system-browser
  baseline on a risky surface, or hides a high-risk approval inside
  the embedded body.
- `shell_embedded_boundary_audit_beta_page_record` — top-level page
  with the aggregate summary banner (surface family, boundary state,
  permission class, fallback posture coverage), the live rows, the
  support rows, and the defects.
- `shell_embedded_boundary_audit_beta_support_export_record` —
  support-export wrapper that quotes the page plus a metadata-safe
  defect roll-up (`defect_kinds_present`, `defect_counts_by_kind`,
  `raw_private_material_excluded=true`).

## Audit axes

The audit checks every row against the closed
[`EmbeddedBoundaryAuditAxis`](../../../crates/aureline-shell/src/embedded_boundary_audit/mod.rs)
vocabulary:

| Axis | What the row must show |
| --- | --- |
| `owner_origin_publisher_disclosed` | Owner label, owner class, publisher/service label, origin host/domain, origin class, and origin verification token are all quoted on the row. |
| `trust_class_disclosed` | Boundary state, data boundary class, permission class, identity mode, and trust state are quoted on the row. |
| `handoff_rules_disclosed` | Browser-fallback posture and target are quoted; when an `open_in_system_browser` action is offered, the handoff packet ref is preserved. |
| `system_browser_baseline_for_identity_or_risky_web` | Identity rows (auth confirmation) and risky web-owned rows (marketplace/account, extension webview, service dashboard) quote a posture from the safe-baseline set: `system_browser_first`, `device_code_fallback_offered`, `external_open_blocked_by_policy`, `external_open_unavailable_offline`. |
| `host_owned_high_risk_approval` | All six native-reserved surfaces stay on `native_reserved_surface_tokens`. |
| `support_export_vocabulary_parity` | The support row reuses the same closed-vocabulary tokens as the live row. |

## Defect vocabulary

The audit emits one of the following typed defects when an axis is
violated:

| Defect kind | When the validator emits it |
| --- | --- |
| `missing_owner_label` | Row did not quote an owner label. |
| `missing_publisher_or_service_label` | Row did not quote a publisher or service label. |
| `missing_origin_host_label` | Row did not quote an origin host or domain. |
| `missing_boundary_state_token` | Row's boundary state token is not in the closed vocabulary. |
| `missing_permission_class_token` | Row's permission class token is not in the closed vocabulary. |
| `missing_browser_fallback_posture_token` | Row's browser-fallback posture token is not in the closed vocabulary. |
| `missing_browser_handoff_packet_ref` | `system_browser_first` posture without a handoff packet ref. |
| `system_browser_not_baseline_on_identity_or_risky_web` | Identity or risky-web row quoted `browser_fallback_not_applicable` instead of a safe-baseline posture. |
| `embedded_minted_native_reserved_surface` | Row dropped one of the six native-reserved surfaces from `native_reserved_surface_tokens`. |
| `support_row_vocabulary_drift` | Support row drifted from the live row on surface family, boundary state, permission class, fallback posture, owner label, host/domain label, identity mode, trust state, fallback target class, or handoff packet ref. |
| `boundary_state_inconsistent_with_origin_verification` | `live_verified` row whose origin is unverified, `policy_blocked` row whose origin is not policy-blocked, etc. |
| `auth_confirmation_missing_flow_class` | Auth-confirmation row missing `auth_flow_class_token` or set to `not_applicable`. |
| `embedded_auth_exception_missing_exception_ref` | Embedded password-exception flow without an `auth_exception_id_ref`. |

## Seeded coverage

The seeded page covers all five embedded surface families. Each row
carries the boundary state, permission class, and posture quoted in
the live alpha card or the inline beta seed:

| Surface family | Boundary state | Permission class | Posture |
| --- | --- | --- | --- |
| `embedded_docs_help` | `live_verified` | `host_owned_full_authority` | `system_browser_first` |
| `extension_hosted_surface` | `cross_origin_limited` | `host_owned_inspect_only` | `system_browser_first` |
| `embedded_marketplace_or_account` | `stale_snapshot` | `host_owned_inspect_only` | `system_browser_first` |
| `embedded_service_dashboard` | `policy_blocked` | `host_owned_browser_only` | `external_open_blocked_by_policy` |
| `embedded_auth_confirmation` | `live_verified` | `host_owned_with_native_step_up_required` | `system_browser_first` |

The seeded page seeds zero defects.

## Failure drills

The unit tests in
[`crates/aureline-shell/src/embedded_boundary_audit/mod.rs`](../../../crates/aureline-shell/src/embedded_boundary_audit/mod.rs)
include one drill per defect kind. The integration test in
[`crates/aureline-shell/tests/embedded_boundary_audit_beta_fixtures.rs`](../../../crates/aureline-shell/tests/embedded_boundary_audit_beta_fixtures.rs)
also replays three named drills against the seeded page (missing
owner label, system-browser baseline drift on auth row, dropped
native-reserved surface) so a regression in any of these surfaces
trips the build.

## Reproduce locally

```sh
cargo test -p aureline-shell --test embedded_boundary_audit_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- validate
```

Regenerate the fixtures from the seeded page:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- page          > fixtures/ux/m3/embedded_boundary_audit/page.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- rows          > fixtures/ux/m3/embedded_boundary_audit/rows.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- support-rows  > fixtures/ux/m3/embedded_boundary_audit/support_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- defects       > fixtures/ux/m3/embedded_boundary_audit/defects.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- support-export > fixtures/ux/m3/embedded_boundary_audit/support_export.json
```

## Storage / index

- Module: [`crates/aureline-shell/src/embedded_boundary_audit/mod.rs`](../../../crates/aureline-shell/src/embedded_boundary_audit/mod.rs)
- Inspector: [`crates/aureline-shell/src/bin/aureline_shell_embedded_boundary_audit.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_embedded_boundary_audit.rs)
- Fixtures: [`fixtures/ux/m3/embedded_boundary_audit/`](../../../fixtures/ux/m3/embedded_boundary_audit/)
- Schema: [`schemas/ux/embedded_boundary_audit_beta.schema.json`](../../../schemas/ux/embedded_boundary_audit_beta.schema.json)
- Integration test: [`crates/aureline-shell/tests/embedded_boundary_audit_beta_fixtures.rs`](../../../crates/aureline-shell/tests/embedded_boundary_audit_beta_fixtures.rs)
- Reviewer artifact: [`artifacts/ux/m3/embedded_surface_boundary_report.md`](../../../artifacts/ux/m3/embedded_surface_boundary_report.md)

## Relationship to adjacent lanes

This audit is **complementary** to the alpha embedded-boundary
projection and the boundary fallback alpha packet:

- The alpha projection
  ([`crates/aureline-shell/src/embedded/boundary_alpha.rs`](../../../crates/aureline-shell/src/embedded/boundary_alpha.rs))
  remains the per-card render contract. The beta audit consumes the
  same alpha card records and adds the page-level audit axes plus
  the typed defect vocabulary.
- The boundary-fallback alpha packet
  ([`crates/aureline-shell/src/embedded/boundary_fallback_alpha.rs`](../../../crates/aureline-shell/src/embedded/boundary_fallback_alpha.rs))
  remains the cross-lane validation packet for system-browser auth
  callbacks and native handoffs. The beta audit does not duplicate
  that packet; it instead pins the embedded boundary chrome itself.
- The beta audit feeds the same support-export pipeline as the
  notification-privacy and token-state audit beta projections, so a
  cross-surface review packet quotes the embedded boundary row, the
  notification row, and the token-state row from one wrapper.
