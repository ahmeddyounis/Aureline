# Difficult filesystem, external-change, and save-conflict regression
# matrix

This reviewer matrix doc is the contract for the regression suite that
runs the difficult filesystem-identity, external-change, and save-
conflict scenarios on the three claimed desktop platform rows
(`linux_desktop`, `macos_desktop`, `windows_desktop`). The suite folds
five required scenario classes — `external_change`, `save_conflict`,
`permission_loss`, `alias_drift`, and `difficult_save_path` — into a
typed corpus that re-uses the [filesystem-identity beta lane](./filesystem_identity_beta.md)
as its source of identity truth. Each row in the matrix binds:

- one `scenario_class` from the closed list,
- one `platform_row_class` from the three claimed desktop rows,
- one anchor `filesystem_identity_beta_case` fixture under
  [`/fixtures/recovery/m3/filesystem_identity/`](../../../fixtures/recovery/m3/filesystem_identity/)
  that proves the underlying identity contract,
- one `expected_behavior` (compare-before-write outcome, silent-
  overwrite ban, canonical save-target redirect, bounded resolution
  actions, and any required save-target blockers),
- one `expected_outcome` (`pass`, `downgrade_required`, or
  `blocked_until_fix`),
- one closed `downgrade_label` from the suite vocabulary
  (`none`, `red_blocks_beta_row`, `yellow_platform_skew`,
  `yellow_partial_coverage`, `degraded_to_safe_path_only`, or
  `stale_corpus_blocks_release_candidate`), and
- a closed-vocabulary `open_gaps` list (`platform_implementation_pending`,
  `external_dependency_unstable`, `manual_recovery_required`, or
  `watcher_fallback_only`).

Implementation:
[`crates/aureline-vfs/src/save_conflict_suite/mod.rs`](../../../crates/aureline-vfs/src/save_conflict_suite/mod.rs).
Boundary schema:
[`schemas/recovery/save_conflict_suite_beta.schema.json`](../../../schemas/recovery/save_conflict_suite_beta.schema.json).
Protected fixture corpus:
[`fixtures/recovery/m3/save_conflict_suite/`](../../../fixtures/recovery/m3/save_conflict_suite/).
Baseline report:
[`artifacts/support/m3/save_conflict_report.md`](../../../artifacts/support/m3/save_conflict_report.md).
Integration drill:
[`crates/aureline-vfs/tests/save_conflict_suite.rs`](../../../crates/aureline-vfs/tests/save_conflict_suite.rs).

## Why this matrix exists

The filesystem-identity beta lane proves that the five-layer identity
model resolves correctly for the difficult cases (symlink alias, case-
only drift, Unicode normalization, bind-mount overlay) on the typed
`local_posix_like` and `container_mount` synthetic roots. But beta
incidents on claimed desktop rows happen at the intersection of those
identity primitives, the platform's filesystem semantics, and the
save-conflict / external-change pipeline. This matrix is the protected
corpus that exercises that intersection so reviewers can audit:

- which (scenario_class × platform_row_class) tuples pass cleanly,
- which tuples are admitted only under a closed downgrade label,
- which tuples are blocked until a fix lands, and
- which open gaps remain — drawn from the closed vocabulary, never from
  free-form prose.

## What this matrix does NOT own

- Live runtime probe execution. The matrix encodes expected behavior
  for each tuple; the proving artifacts live under the filesystem-
  identity beta corpus and the alpha save-pipeline harness.
- New downgrade vocabulary. A failing row must downgrade using the
  closed `downgrade_label` list; new vocabulary lands as a schema and
  matrix update reviewed together.
- Cross-platform alias-kind extensions. New alias kinds (e.g. macOS
  firmlinks, Windows reparse-point variants) land in the filesystem-
  identity beta schema first and are referenced from this matrix.

## Required scenario × platform tuples

The corpus MUST seed one case for every cell of the matrix below. The
evaluator refuses any corpus missing a tuple, declaring duplicate
`(scenario_class, platform_row_class)` pairs, or omitting a row's
downgrade label.

| Scenario class \\ Platform | linux_desktop | macos_desktop | windows_desktop |
| --- | --- | --- | --- |
| `external_change` | pass | pass | downgrade_required (`yellow_platform_skew`) |
| `save_conflict` | pass | pass | downgrade_required (`yellow_platform_skew`) |
| `permission_loss` | pass | pass | downgrade_required (`degraded_to_safe_path_only`) |
| `alias_drift` | downgrade_required (`yellow_partial_coverage`) | pass | pass |
| `difficult_save_path` | pass | downgrade_required (`degraded_to_safe_path_only`) | downgrade_required (`degraded_to_safe_path_only`) |

## Acceptance and how this row meets it

- **The regression suite covers external change, conflict, permission
  loss, alias drift, and difficult save-path conditions on claimed
  desktop rows.** Five required scenario classes × three required
  platform-row classes = 15 seeded fixtures; the evaluator refuses a
  corpus missing any tuple.
- **The report records exact behavior, downgrade labels, and open gaps
  per platform.** The
  [`SaveConflictSuiteReport`](../../../crates/aureline-vfs/src/save_conflict_suite/mod.rs)
  projection emits one `ReportMatrixRow` per case (with compare
  outcome, expected outcome, downgrade label, and open-gap classes) and
  one `PlatformSummaryRow` per platform (with pass/downgrade/blocked
  counts and an open-gap count). The baseline artifact at
  [`artifacts/support/m3/save_conflict_report.md`](../../../artifacts/support/m3/save_conflict_report.md)
  is the human-reviewable rendering of that report.
- **A failing row can be downgraded without inventing new vocabulary
  outside the suite.** The closed `DowngradeLabel` enum admits exactly
  six labels; the closed `OpenGapClass` enum admits exactly five gap
  classes. The evaluator refuses any record carrying a label or class
  outside those enums (the JSON-schema boundary refuses unknown enum
  values).

## Failure-drill posture

The evaluator fails closed before widening any safety guarantee:

- A `pass` row that carries a non-`none` `downgrade_label` is refused.
- A `downgrade_required` or `blocked_until_fix` row that omits a
  non-`none` `open_gap` entry is refused.
- A `blocked_until_fix` row that does not downgrade with
  `red_blocks_beta_row` or `stale_corpus_blocks_release_candidate` is
  refused.
- An `external_change` row whose `compare_outcome` is not
  `external_change_detected` is refused; a `save_conflict` row whose
  `compare_outcome` is neither `save_conflict` nor
  `external_change_detected` is refused.
- A `permission_loss` row that does not declare at least one of
  `read_only`, `not_writable_per_snapshot`, or `policy_constrained`
  blockers is refused.
- A row whose expected `resolution_actions` include `write` while
  `compare_outcome` is non-`unchanged` (or `silent_overwrite_forbidden`
  is `true`) is refused.
- A row that admits raw private material, admits ambient authority,
  declares destructive resets, or drops user-authored-files
  preservation is refused.
- A corpus missing any required `(scenario_class, platform_row_class)`
  tuple is refused.

## First consumers

- The `aureline-vfs` `save_conflict_suite` module is the canonical
  loader, validator, and reporter for the matrix. The chrome's save-
  review surface and the support-export pipeline read off the same
  record so they never re-derive scenario / platform vocabulary from a
  side channel.
- The boundary schema is the contract the headless export writer and
  the support-export chrome share — both reconstruct the same shape
  from the on-disk record verbatim.
- The protected corpus is the proving ground for the 15
  (scenario, platform) tuples the beta release-candidate owes; the
  integration drill at
  [`crates/aureline-vfs/tests/save_conflict_suite.rs`](../../../crates/aureline-vfs/tests/save_conflict_suite.rs)
  re-proves schema, doc, fixture, anchor, and crate-consumer presence
  on disk and round-trips every case through serde.

## Out of scope

- Cross-platform deduplication of platform-specific alias kinds. The
  matrix records the per-platform expectations; cross-platform unification
  lands in the filesystem-identity beta vocabulary and is mirrored here
  when promoted.
- Hosted intake, ticket routing, or upload transport for the support
  packet. The report is consumed locally by the support-export pipeline
  and the chrome's matrix surface; sharing belongs to later milestones.
- Adding new downgrade labels or open-gap classes without updating the
  schema, the Rust module, the reviewer doc, the baseline report, and
  the protected corpus together.
