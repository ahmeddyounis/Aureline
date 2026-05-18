# Marketplace fact-grid beta

Marketplace browsing and install review use one shared fact grid for public
catalogs, approved mirrors, private registries, offline rows, and manual
imports. The implementation lives in
[`crates/aureline-extensions/src/fact_grid/`](../../../crates/aureline-extensions/src/fact_grid/),
the shell projection is in
[`crates/aureline-shell/src/extensions/marketplace/`](../../../crates/aureline-shell/src/extensions/marketplace/),
and the boundary schema is
[`schemas/extensions/marketplace_fact_grid.schema.json`](../../../schemas/extensions/marketplace_fact_grid.schema.json).

## Required Fact Grid

Every claimed ecosystem row must carry:

| Field family | Required truth |
| --- | --- |
| Client scope | `desktop`, `browser_companion`, `desktop_plus_browser_companion`, `headless_only`, or `unsupported` |
| Source lane | public registry, approved mirror, private registry, offline bundle, local archive, or quarantined local copy |
| Lifecycle and support | lifecycle badges, support chips, compatibility label, compatibility report row, and bridge matrix row |
| Workspace change | exact manifest changes, permission deltas, lockfile/generated-file impact, and rollback checkpoint |
| Script/native-build risk | no scripts, lifecycle scripts, native build, external helper/host, or unknown-blocked |
| Revocation posture | revocation state, snapshot age, last-known-good version, rollback manifest, and emergency-disable refs |
| Activation budget | CPU, memory, startup ceiling, feature gates, triggers, budget axes, runtime-cost class, and evidence class |

Rows that cannot explain client scope, script/native-build risk, lockfile
impact, revocation state, or activation budget narrow or block the install path
instead of guessing.

## Parity Rules

- Public, mirrored, private, offline, and manual-import lanes use the same
  field names and controlled vocabulary.
- Result rows, detail pages, install/update/rollback review, compatibility
  reports, diagnostics, and support exports read the same fact grid.
- Support exports quote client scope, registry source, compatibility,
  activation cost, script/native risk, lockfile impact, manifest-change count,
  permission-delta count, revocation state, and block posture without drift.
- Install review is treated as workspace-change review: manifest changes,
  lockfile/generated-file churn, script/native-build risk, permission deltas,
  activation budget, and rollback posture are visible before commit.

## Fixtures

The checked corpus lives at
[`fixtures/extensions/m3/fact_grid_and_install_review/`](../../../fixtures/extensions/m3/fact_grid_and_install_review/).
The same records are embedded in
[`fixtures/ux/m3/marketplace_truth/page.json`](../../../fixtures/ux/m3/marketplace_truth/page.json)
so shell, headless, and support-export consumers validate the same contract.

## Verification

```sh
cargo test -p aureline-extensions fact_grid
cargo test -p aureline-shell --lib extensions::marketplace
cargo test -p aureline-shell --test marketplace_truth_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_marketplace_truth -- validate
```
