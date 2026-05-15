# Migration wizard (beta)

This page describes the beta-grade migration-wizard projection that
lives in `aureline-shell`. It wraps the lightweight import classifier
in [`crate::import`](../../../crates/aureline-shell/src/import/mod.rs)
and the import diff review packet in
[`crate::import::diff_review`](../../../crates/aureline-shell/src/import/diff_review.rs)
into one guided flow so a switching user sees the source, the
classified mapping report, the rollback checkpoint, and the
compare/undo paths in the same projection.

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
  compare actions, undo actions, stage history, rollback-checkpoint
  binding, and support-export wrapper.
- [`docs/migration/first_run_import_diff_and_rollback_contract.md`](../first_run_import_diff_and_rollback_contract.md)
  — first-run import contract the wizard composes with.
- [`docs/migration/migration_restore_and_shortcut_delta_packet.md`](../migration_restore_and_shortcut_delta_packet.md)
  — shortcut delta digest the wizard retains with the report.

## Contract surface

The beta wizard ships under the shared contract ref
`shell:migration_wizard_beta:v1` and emits the following record kinds:

- `shell_migration_wizard_beta_page_record` — the wizard page. Carries
  the stage history, source/target descriptors, the import diff
  preview ref, the mapping report, the rollback-checkpoint binding,
  compare/undo action lists, the apply gate, and a summary banner.
- `shell_migration_wizard_beta_mapping_report_record` — the retained
  mapping report. Carries the per-row classification (`exact`,
  `translated`, `partial`, `shimmed`, `unsupported`), the per-domain
  before/after labels, the rollback checkpoint ref, the shortcut delta
  digest ref, and the reopen links for settings, help, and support
  export.
- `shell_migration_wizard_beta_mapping_row_record` — one classified
  mapping row inside the report.
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
4. `checkpoint_ready` — the rollback checkpoint is minted and the
   apply gate opens.
5. `applying` — apply is running against the reviewed preview and
   checkpoint.
6. `applied` / `partially_applied` / `blocked` — the apply landed
   cleanly, landed with retained partials, or was denied without
   mutating durable state.
7. `rolled_back` — the undo path triggered; the checkpoint restored
   prior state and the report still names what was reverted.

The page's `stage_history` records every entered stage with the
`durable_writes_authorized` invariant: no stage that admits durable
writes may appear before the wizard recorded
`stage="checkpoint_ready"`. The validator
(`validate_migration_wizard_page`) rejects any stage history that
admits durable writes before the checkpoint.

## Acceptance posture

The beta wizard delivers the M3 migration-wizard acceptance gates:

- **Every imported item is classified as Exact, Translated, Partial,
  Shimmed, or Unsupported and the report survives after import.** The
  mapping report stores the classification per row, the per-class
  count summary, and the `retained_after_first_run=true` invariant.
  The validator rejects any report that is missing a required
  classification or that is not retained.
- **The wizard creates a rollback checkpoint before mutating durable
  state and exposes compare/undo paths after import.** The
  `rollback_checkpoint` binding requires `created_before_apply=true`
  and `protects_every_domain=true`; the page exposes a non-empty
  `compare_actions` list (one per touched domain) and a non-empty
  `undo_actions` list (`restore_from_checkpoint` plus
  `export_for_support`). The validator rejects post-apply stages with
  missing compare or undo paths.
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
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- rollback-checkpoint
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- compact
cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- validate
```

`validate` exits non-zero (status `3`) if any acceptance invariant is
violated; it is wired so CI can fail closed on a regression in any of
the record kinds.

## Fixtures

Reviewable fixtures live under
[`fixtures/migration/m3/migration_wizard/`](../../../fixtures/migration/m3/migration_wizard/):

- `page.json` — full beta wizard page (stage history + mapping report
  + rollback checkpoint + compare/undo + summary).
- `mapping_report.json` — retained mapping report with classification
  summary and reopen links.
- `unsupported_gaps.json` — pre-apply unsupported / bridge gaps that
  must remain visible.
- `compare_actions.json` — per-domain compare paths exposed after
  apply.
- `undo_actions.json` — restore-from-checkpoint and export-for-support
  paths exposed after apply.
- `stage_history.json` — admitted stage transitions with the
  `durable_writes_authorized` invariant.
- `rollback_checkpoint.json` — the rollback-checkpoint binding the
  wizard minted before apply.
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
