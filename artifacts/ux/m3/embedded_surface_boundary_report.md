# Embedded-surface boundary audit (reviewer entrypoint)

This page is the reviewer entrypoint for the M3 promise that
owner-origin chrome and browser-handoff rules remain real on every
embedded surface, and that support exports use the same boundary
vocabulary as the live UI. The audit's typed output, not screenshots
or local notes, is what M3 review cites when checking that:

- Each claimed embedded surface shows owner / origin, trust class, and
  handoff rules clearly enough for support and privacy review.
- The system-browser default remains the baseline on identity rows
  and risky web-owned flows.
- Support exports and docs use the same closed-vocabulary tokens as
  the live UI.

## What the audit covers

The audit projects all five embedded surface families declared in
[`crates/aureline-shell/src/embedded/boundary_card.rs`](../../../crates/aureline-shell/src/embedded/boundary_card.rs):

- `embedded_docs_help` — embedded docs/help panes.
- `extension_hosted_surface` — extension-owned webviews and panels.
- `embedded_marketplace_or_account` — marketplace and account
  surfaces (install scope, billing scope).
- `embedded_service_dashboard` — connected-service dashboards
  (status, payments, ops).
- `embedded_auth_confirmation` — auth handoff confirmation surfaces
  (system browser, device code, platform authenticator).

Each row carries the source card id, the surface family, the owner /
publisher / origin disclosure, the trust class disclosure (boundary
state, data boundary class, permission class, identity mode, trust
state), the handoff rules disclosure (browser fallback posture,
target class, handoff packet ref), the optional auth-handoff block,
the optional provider block, the host-owned native-reserved surfaces,
and the closed list of audit axes the row promises.

## Defect list — read it first

The audit's only output is a typed defect list under
[`fixtures/ux/m3/embedded_boundary_audit/defects.json`](../../../fixtures/ux/m3/embedded_boundary_audit/defects.json).
The seeded value is `[]` — every claimed beta row passes. Reviewers
should regenerate the fixture (see below) and confirm the defect list
remains empty before signing off the lane.

The closed defect vocabulary is documented in
[`docs/ux/m3/embedded_boundary_beta.md`](../../docs/ux/m3/embedded_boundary_beta.md);
the validator drills under
[`crates/aureline-shell/src/embedded_boundary_audit/mod.rs`](../../../crates/aureline-shell/src/embedded_boundary_audit/mod.rs)
and
[`crates/aureline-shell/tests/embedded_boundary_audit_beta_fixtures.rs`](../../../crates/aureline-shell/tests/embedded_boundary_audit_beta_fixtures.rs)
prove each defect kind surfaces when the matching field is broken.

The reusable toolkit projection adds the render row, event-log, and
support-export shape that product surfaces consume:

[`fixtures/ux/m3/embedded_boundary/page.json`](../../../fixtures/ux/m3/embedded_boundary/page.json)

Its seeded defect list is also empty:

[`fixtures/ux/m3/embedded_boundary/defects.json`](../../../fixtures/ux/m3/embedded_boundary/defects.json)

## Reproduce locally

Validate the seeded page (exits non-zero on any defect):

```sh
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- validate
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- validate
```

Run the fixture replay test (proves the checked-in JSON cannot drift
from the seeded page):

```sh
cargo test -p aureline-shell --test embedded_boundary_audit_beta_fixtures
cargo test -p aureline-shell --test embedded_boundary_toolkit_fixtures
```

Regenerate the fixture corpus from the seeded page (the inspector is
the only mint-from-truth path):

```sh
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- page          > fixtures/ux/m3/embedded_boundary_audit/page.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- rows          > fixtures/ux/m3/embedded_boundary_audit/rows.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- support-rows  > fixtures/ux/m3/embedded_boundary_audit/support_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- defects       > fixtures/ux/m3/embedded_boundary_audit/defects.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- support-export > fixtures/ux/m3/embedded_boundary_audit/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- page          > fixtures/ux/m3/embedded_boundary/page.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- rows          > fixtures/ux/m3/embedded_boundary/rows.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- events        > fixtures/ux/m3/embedded_boundary/event_log.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- defects       > fixtures/ux/m3/embedded_boundary/defects.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- support-export > fixtures/ux/m3/embedded_boundary/support_export.json
```

## Failure drills (proves the lane fails loudly)

The unit tests under
[`crates/aureline-shell/src/embedded_boundary_audit/mod.rs`](../../../crates/aureline-shell/src/embedded_boundary_audit/mod.rs)
and the integration test under
[`crates/aureline-shell/tests/embedded_boundary_audit_beta_fixtures.rs`](../../../crates/aureline-shell/tests/embedded_boundary_audit_beta_fixtures.rs)
include one drill per axis. Patching one of the drill cases (e.g.
clearing the owner label, switching the auth row's posture to
`browser_fallback_not_applicable`, dropping `ai_apply_review` from
`native_reserved_surface_tokens`) and re-running `cargo test` confirms
the validator surfaces the matching defect entry.

A drill that fails to surface its expected defect is itself a
regression — the test fails the build.

## Rows and source files

| Surface family | Audit row source |
| --- | --- |
| `embedded_docs_help` | Alpha card seeded by [`crate::embedded::docs_help::seeded_docs_help_boundary_card`](../../../crates/aureline-shell/src/embedded/docs_help.rs) |
| `extension_hosted_surface` | [`fixtures/ux/embedded_boundary_alpha/extension_webview_alpha_card.json`](../../../fixtures/ux/embedded_boundary_alpha/extension_webview_alpha_card.json) |
| `embedded_marketplace_or_account` | [`fixtures/ux/embedded_boundary_alpha/marketplace_account_alpha_card.json`](../../../fixtures/ux/embedded_boundary_alpha/marketplace_account_alpha_card.json) |
| `embedded_service_dashboard` | Inline beta seed in [`crate::embedded_boundary_audit::service_dashboard_policy_blocked_card`](../../../crates/aureline-shell/src/embedded_boundary_audit/mod.rs) |
| `embedded_auth_confirmation` | Inline beta seed in [`crate::embedded_boundary_audit::auth_confirmation_system_browser_first_card`](../../../crates/aureline-shell/src/embedded_boundary_audit/mod.rs) |

## Storage / index

- Module: [`crates/aureline-shell/src/embedded_boundary_audit/mod.rs`](../../../crates/aureline-shell/src/embedded_boundary_audit/mod.rs)
- Inspector: [`crates/aureline-shell/src/bin/aureline_shell_embedded_boundary_audit.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_embedded_boundary_audit.rs)
- Fixtures: [`fixtures/ux/m3/embedded_boundary_audit/`](../../../fixtures/ux/m3/embedded_boundary_audit/)
- Schema: [`schemas/ux/embedded_boundary_audit_beta.schema.json`](../../../schemas/ux/embedded_boundary_audit_beta.schema.json)
- Integration test: [`crates/aureline-shell/tests/embedded_boundary_audit_beta_fixtures.rs`](../../../crates/aureline-shell/tests/embedded_boundary_audit_beta_fixtures.rs)
- Companion doc: [`docs/ux/m3/embedded_boundary_beta.md`](../../../docs/ux/m3/embedded_boundary_beta.md)
- Toolkit module: [`crates/aureline-shell/src/embedded_boundary/mod.rs`](../../../crates/aureline-shell/src/embedded_boundary/mod.rs)
- Toolkit inspector: [`crates/aureline-shell/src/bin/aureline_shell_embedded_boundary_toolkit.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_embedded_boundary_toolkit.rs)
- Toolkit fixtures: [`fixtures/ux/m3/embedded_boundary/`](../../../fixtures/ux/m3/embedded_boundary/)
- Toolkit doc: [`docs/ux/m3/embedded_boundary_toolkit.md`](../../../docs/ux/m3/embedded_boundary_toolkit.md)
- Native approval review: [`artifacts/ux/m3/native_approval_boundary_review.md`](./native_approval_boundary_review.md)
- System-browser auth drill: [`artifacts/ux/m3/system_browser_auth_drill.md`](./system_browser_auth_drill.md)

## Relationship to adjacent lanes

This audit is **complementary** to the alpha embedded-boundary
projection and the boundary-fallback alpha packet:

- The alpha projection
  ([`crates/aureline-shell/src/embedded/boundary_alpha.rs`](../../../crates/aureline-shell/src/embedded/boundary_alpha.rs))
  remains the per-card render contract. The beta audit consumes the
  same alpha card records and layers the page-level audit axes plus
  the typed defect vocabulary.
- The boundary-fallback alpha packet
  ([`crates/aureline-shell/src/embedded/boundary_fallback_alpha.rs`](../../../crates/aureline-shell/src/embedded/boundary_fallback_alpha.rs))
  remains the cross-lane validation packet for system-browser auth
  callbacks and native handoffs.
- The beta audit feeds the same support-export pipeline as the
  notification-privacy and token-state audit beta projections, so a
  cross-surface review packet quotes the embedded boundary row, the
  notification row, and the token-state row from one wrapper.
