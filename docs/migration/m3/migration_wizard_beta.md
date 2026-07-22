# Migration wizard (beta)

This page describes the beta-grade migration-wizard projection that
lives in `aureline-shell`. It wraps the lightweight import classifier
in [`crate::import`](../../../crates/aureline-shell/src/import/mod.rs)
and the import diff review packet in
[`crate::import::diff_review`](../../../crates/aureline-shell/src/import/diff_review.rs)
into one guided flow so a switching user sees the source, the
classified mapping report, the rollback-checkpoint requirement, and the
available compare paths in the same projection. A dry-run packet does not
prove that a checkpoint, restore record, or applied lifecycle exists.

The projection is the page-level surface that the live shell, the
headless inspector
([`aureline_shell_migration_wizard`](../../../crates/aureline-shell/src/bin/aureline_shell_migration_wizard.rs)),
and the support-export wrapper consume so UI rows, CLI rows, and
support-export rows always come from the same `wizard_session_id` and
`shared_contract_ref`.

Companion artifacts:

- [`/artifacts/migration/m3/mapping_report.schema.json`](../../../artifacts/migration/m3/mapping_report.schema.json)
  — boundary schema for `shell_migration_wizard_beta_mapping_report_record`.
- [`/fixtures/migration/m3/migration_wizard/`](../../../fixtures/migration/m3/migration_wizard/)
  — minted-from-truth wizard page, mapping report, unsupported gaps,
  compare actions, stage history, rollback-checkpoint requirement,
  and support-export wrapper.
- [`docs/migration/first_run_import_diff_and_rollback_contract.md`](../first_run_import_diff_and_rollback_contract.md)
  — first-run import contract the wizard composes with.
- [`docs/migration/migration_restore_and_shortcut_delta_packet.md`](../migration_restore_and_shortcut_delta_packet.md)
  — shortcut delta digest the wizard retains with the report.
- [`docs/release/m3/update_rollback_beta.md`](../../release/m3/update_rollback_beta.md)
  and
  [`/artifacts/release/m3/update_rollback/rollback_plan.json`](../../../artifacts/release/m3/update_rollback/rollback_plan.json)
  — update rollback plan that composes retained prior artifacts,
  schema/state hooks, and downgrade caveats with the migration
  rollback-checkpoint model.

## Contract surface

The beta wizard ships under the shared contract ref
`shell:migration_wizard_beta:v2` and emits the following record kinds. Version
2 is the breaking correction that replaces preview-fabricated session and
checkpoint identities with review and requirement identities:

- `shell_migration_wizard_beta_page_record` — the wizard page. Carries
  the stage history, source/target descriptors, the import diff
  preview ref, the mapping report, the rollback-checkpoint requirement,
  compare/undo action lists, the apply gate, and a summary banner. Preview
  packets emit no undo actions because no executable checkpoint exists yet.
- `shell_migration_wizard_beta_mapping_report_record` — the retained
  mapping report. Carries the per-row classification (`exact`,
  `translated`, `partial`, `shimmed`, `unsupported`), the per-domain
  before/after labels, the rollback requirement ref, the shortcut delta
  digest ref, and the reopen links for settings, help, and support
  export.
- `shell_migration_wizard_beta_mapping_row_record` — one classified
  mapping row inside the report. Its stable row id includes the source
  ecosystem, domain, classification, and redacted source-item suffix; two
  classifications for the same source item therefore never collapse onto one
  support pivot.
- `shell_migration_wizard_beta_unsupported_gap_record` — one pre-apply
  unsupported / bridge gap that MUST be visible before apply and
  retained after apply.
- `shell_migration_wizard_beta_support_export_record` — the support
  export wrapper that quotes the page plus the stable ids reviewers
  pivot on.

## Wizard stages

The wizard moves through a deterministic sequence of reviewable
stages, all named with stable schema tokens so surfaces never invent
their own status names:

1. `selecting_source` — the user has not yet chosen a readable source.
2. `source_detected` — the source root has been classified read-only.
3. `preview_ready` — the diff review packet is materialized and
   unsupported gaps are surfaced before apply.
4. `checkpoint_ready` — orchestration has supplied real execution evidence
   that a rollback checkpoint exists and the apply gate may open.
5. `applying` — apply is running against the reviewed preview and
   checkpoint.
6. `applied` / `partially_applied` / `blocked` — the apply landed
   cleanly, landed with retained partials, or was denied without
   mutating durable state.
7. `rolled_back` — the undo path triggered; the checkpoint restored
   prior state and the report still names what was reverted.

The dry-run page builder can establish only `selecting_source`,
`source_detected`, `preview_ready`, or `blocked`. Requests for later stages are
narrowed to `preview_ready`; the builder never fabricates lifecycle evidence.
The validator rejects checkpoint-ready, applying, applied, partial, or
rolled-back claims in a preview-only page. A future orchestration projection
may advance those stages only after binding a real execution checkpoint and
actual apply/restore results.

## Update Rollback Composition

Migration rollback checkpoints and update rollback plans use separate
records but the same review posture: a state-changing rollback must name
the checkpoint or hook that owns the restore path before durable writes
resume. The beta update rollback plan binds current exact build
`build-id:aureline:beta:2.1.0-beta.1:aarch64-apple-darwin:release:b7ee32adb5eb`
to rollback target `release_candidate:aureline.2_0_4_stable` and target
`exact_build_identity_ref`
`build-id:aureline:stable:2.0.4:aarch64-apple-darwin:release:1f40c9d2b4a1`.

The wizard may surface update rollback caveats beside migration undo
state, but it does not invoke update hooks directly. A
`schema_rollback_hook` is usable only through the reviewed
`checkpoint.update.*` flow named in the rollback plan. The shared
rollback vocabulary that migration, Help, docs, and support quote is
`retained_prior_artifact_set`, `schema_rollback_hook`,
`downgrade_eligibility_state`, and `exact_build_identity_ref`.

## Acceptance posture

The beta wizard delivers the M3 migration-wizard acceptance gates:

- **Every imported item is classified as Exact, Translated, Partial,
  Shimmed, or Unsupported and the report survives after import.** The
  mapping report stores the classification per row, the per-class
  count summary, and the `retained_after_first_run=true` invariant.
  Every row, including Exact and Translated rows, carries a bounded
  support-export ref so the retained report can reopen the precise row without
  copying source paths or values.
  The validator rejects any report that is missing a required
  classification or that is not retained.
- **The wizard records the checkpoint precondition without claiming it has
  already been met.** `rollback_requirement.requirement_state` is
  `required_before_apply`, every mapping row cites the same requirement, the
  preview exposes per-domain compare actions, restore remains disabled, and
  `undo_actions` is empty. Execution must bind a real checkpoint before apply;
  only an actual rollback result may mint a restore record.
- **Unsupported gaps are visible immediately instead of being
  discovered as hidden missing behavior later.** Every
  `unsupported_gap_row` carries `visible_before_apply=true` and
  `retained_after_apply=true`. The validator rejects a hidden gap.
- **Reopen paths are first-class.** The mapping report includes
  reopen links for settings, help, and support-export. The validator
  rejects a missing surface.

## Headless consumers

The beta wizard is exercised through the
`aureline_shell_migration_wizard` binary. The bin is the only
mint-from-truth path for the JSON checked in under
`fixtures/migration/m3/migration_wizard/`, so the live shell, the
review packet, and the support-export rows cannot drift.

```sh
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- page
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- mapping-report
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- unsupported-gaps
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- compare-actions
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- undo-actions
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- stage-history
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- rollback-requirement
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- emit-fixtures fixtures/migration/m3/migration_wizard
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- compact
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- validate
```

`validate` exits non-zero (status `3`) if any acceptance invariant is
violated; it is wired so CI can fail closed on a regression in any of
the record kinds.

## Fixtures

Reviewable fixtures live under
[`fixtures/migration/m3/migration_wizard/`](../../../fixtures/migration/m3/migration_wizard/):

- `page.json` — preview-only beta wizard page (stage history + mapping report
  + rollback requirement + compare + summary).
- `mapping_report.json` — retained mapping report with classification
  summary and reopen links.
- `unsupported_gaps.json` — pre-apply unsupported / bridge gaps that
  must remain visible.
- `compare_actions.json` — per-domain before/after compare paths.
- `undo_actions.json` — empty for the preview-only fixture; execution evidence
  is required before an undo action can be exported.
- `stage_history.json` — admitted stage transitions with the
  `durable_writes_authorized` invariant.
- `rollback_requirement.json` — the pre-apply checkpoint requirement; its ref
  is explicitly not a checkpoint handle.
- `support_export.json` — support-export wrapper that quotes the page
  and every case id.

## Verification

```sh
cargo test -p aureline-shell --test migration_wizard_fixtures
cargo test -p aureline-shell --lib migration_wizard
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- validate
```

The fixture test in
[`crates/aureline-shell/tests/migration_wizard_fixtures.rs`](../../../crates/aureline-shell/tests/migration_wizard_fixtures.rs)
replays every JSON fixture through the Rust types, asserts the
contract invariants, and asserts that the checked-in `page.json` is
bit-for-bit equal to the page returned by the seeded builder.
Regenerating with the headless bin is the only mint-from-truth path.
