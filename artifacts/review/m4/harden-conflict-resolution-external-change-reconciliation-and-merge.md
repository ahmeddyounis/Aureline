# Artifact: Hardened conflict resolution, external-change reconciliation, and merge-editor recovery on stable rows

**Task:** M04-100
**Produced:** 2026-05-27
**Status:** Complete

## Summary

This artifact documents the bounded implementation of durable conflict-session contracts that harden merge, rebase, cherry-pick, revert, and external-change reconciliation on stable rows. The stable conflict session survives restart, preserves provenance of competing inputs, and supports honest downgrade between structured and raw resolution modes without implying the conflict is resolved.

## Deliverables

1. **Rust module** — `crates/aureline-git/src/harden_conflict_resolution_external_change_reconciliation_and_merge/mod.rs`
   - Defines `StableConflictSessionRecord`, `StableConflictSessionPacket`, and constituent records.
   - Tracks `repo_ref`, `worktree_ref`, `operation_kind`, `base_ref`, `ours_ref`, `theirs_ref`, `affected_path_tokens`, `unresolved_count`, `resolution_mode`, and `started_at`/`updated_at` lineage.
   - Validates cross-record invariants (downgrade honesty, checkpoint requirements, provenance preservation, restart survivability).
   - Projects into `StableConflictSessionProjection` for CLI/inspector surfaces.
   - Builds command-graph records with actionability computed from session state.

2. **JSON Schema** — `schemas/git/stable_conflict_session.schema.json`
   - Validates record structure, closed vocabularies, and support-export safety.
   - Enforces that `downgraded_structured_to_raw` cannot coexist with completed lifecycle states.
   - Enforces that `continuing_after_resolution` requires a non-null `recovery_checkpoint_ref`.

3. **Fixtures** — `fixtures/git/m4/harden_conflict_resolution_external_change_reconciliation_and_merge/`
   - 5 JSON fixtures covering:
     - Merge conflict in structured mode.
     - Merge conflict honestly downgraded from structured to raw.
     - External-change reconciliation session.
     - Rebase paused with recovery checkpoint captured.
     - Session resumed after restart with `previous_session_ref` lineage.

4. **Tests** — `crates/aureline-git/tests/harden_conflict_resolution_external_change_reconciliation_and_merge_alpha.rs`
   - 11 fixture-driven and unit tests covering projection, round-trip, validation, restart survivability, provenance preservation, downgrade honesty, checkpoint invariants, consumer-surface vocabulary, support-export reopenability, and command actionability.

5. **CLI mirror** — `crates/aureline-git/src/bin/aureline_git_stable_conflict_session.rs`
   - Headless entry point for projecting, validating, and building stable conflict-session packets.

## Acceptance evidence

- All 11 stable-conflict-session tests pass.
- All 68 `aureline-git` tests pass (including existing status, history-rewrite, conflict-handoff, branch, commit, mutation, publish, daily-loop, and change-object suites).
- Workspace compiles with `cargo check`.

## Risks and limits

- This module is **contract and validation only**; it does not shell out to Git or implement a live merge-editor service. The live service will be built on top of the existing `conflicts` alpha contract and `history_rewrite` beta contract.
- Auto-resolution and provider-hosted merge-queue execution remain out of scope.
- Cross-repo operations and three-way merge content rendering are explicitly out of scope.
- The module currently focuses on local worktree conflicts; submodule conflict sessions are not yet modeled.

## Stable claims

- Conflict sessions survive restart (`session_id`, `repo_ref`, `worktree_ref`, `started_at` required).
- Provenance of competing inputs is preserved (`base_ref`, `ours_ref`, `theirs_ref` + source classes required).
- Honest downgrade from structured to raw mode is tracked and does not imply resolution.
- Recovery checkpoint is required for `continuing_after_resolution` and enforced at validation time.
- Support export includes a `restart_snapshot` so the same structured session truth can be reopened after restart.
- Consumer surfaces include both `support_export` and `audit_lane` by contract.
- All `raw_*_export_allowed` flags are `false` by contract.
