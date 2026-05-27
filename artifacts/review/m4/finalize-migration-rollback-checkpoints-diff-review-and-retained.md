# Artifact: Finalize migration rollback checkpoints, diff review, and retained diagnostics for failed imports

**Task:** M04-106
**Produced:** 2026-05-27
**Status:** Complete

## Summary

This artifact documents the bounded implementation of migration rollback checkpoints, diff review, and retained diagnostics for failed imports. It hardens the migration lane so that every destructive import is previewable or checkpointed, and failed imports retain actionable diagnostics.

## Deliverables

1. **Rust module** — `crates/aureline-review/src/finalize_migration_rollback_checkpoints_diff_review_and_retained/mod.rs`
   - Defines `MigrationRollbackDiffReviewPacket` and its constituent records.
   - Validates cross-record invariants (diff approval ↔ checkpoint, validation_failed ↔ retained diagnostics, rolled_back ↔ checkpoint restored).
   - Projects into `MigrationRollbackDiffReviewProjection` for CLI/inspector surfaces.

2. **JSON Schema** — `schemas/review/migration_rollback_diff_review.schema.json`
   - Validates packet structure, closed vocabularies, and support-export safety.

3. **Fixtures** — `fixtures/review/m4/finalize-migration-rollback-checkpoints-diff-review-and-retained/`
   - 4 JSON fixtures covering settings import (diff-approved, checkpoint-ready), keymap import (validation-failed with diagnostics), snippet import (rolled-back), and theme import (aborted).

4. **Tests** — `crates/aureline-review/tests/finalize_migration_rollback_checkpoints_diff_review_and_retained_alpha.rs`
   - 8 fixture-driven tests covering projection, round-trip, checkpoint invariants, validation-failed diagnostic retention, rollback non-actionability, aborted non-actionability, support-export snapshot fidelity, and diff-rejection blocking.

## Acceptance evidence

- All 8 migration rollback diff-review tests pass.
- All `aureline-review` tests pass (including existing landing, workspace, diff, and review-pack suites).
- Workspace compiles with `cargo check`.

## Risks and limits

- This module is **contract and validation only**; it does not implement live importer adapters or a live apply service. The live service will be built on top of the existing `aureline-workspace` migration-wizard import-fidelity contract.
- Provider-hosted migration execution remains out of scope; this module covers local review, checkpoint truth, and diagnostic retention only.
- Cross-editor auto-translation and shim resolution are explicitly out of scope.

## Stable claims

- Diff-first review gate is observable on all eight supported migration operation kinds.
- Rollback checkpoint summary is required before destructive apply and enforced at validation time.
- Validation-failed flows retain at least one diagnostic record.
- Restart snapshots in support exports mirror current packet truth.
- All `raw_*_export_allowed` flags in support exports are `false`.
