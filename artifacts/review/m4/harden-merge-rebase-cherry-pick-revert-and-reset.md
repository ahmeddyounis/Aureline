# Artifact: Diff-first review and recovery checkpoints for merge, rebase, cherry-pick, revert, and reset

**Task:** M04-098
**Produced:** 2026-05-27
**Status:** Complete

## Summary

This artifact documents the bounded implementation of diff-first review and recovery-checkpoint contracts for the daily-driver Git rewrite lane (merge, rebase, cherry-pick, revert, reset).

## Deliverables

1. **Rust module** — `crates/aureline-review/src/harden_merge_rebase_cherry_pick_revert_and_reset/mod.rs`
   - Defines `DiffFirstRewriteFlowPacket` and its constituent records.
   - Validates cross-record invariants (diff approval ↔ checkpoint, protected branch ↔ actionable, sequence-edit uniqueness).
   - Projects into `DiffFirstRewriteFlowProjection` for CLI/inspector surfaces.

2. **JSON Schema** — `schemas/review/diff_first_rewrite_flow.schema.json`
   - Validates packet structure, closed vocabularies, and support-export safety.

3. **Fixtures** — `fixtures/review/m4/harden_merge_rebase_cherry_pick_revert_and_reset/`
   - 6 JSON fixtures covering merge, rebase (paused conflict), cherry-pick (protected branch blocked), revert (reflog-only), reset (hard with checkpoint), and interactive rebase (sequence running).

4. **Tests** — `crates/aureline-review/tests/harden_merge_rebase_cherry_pick_revert_and_reset_alpha.rs`
   - 11 fixture-driven tests covering projection, round-trip, checkpoint invariants, protected-branch blocking, restartability, sequence-edit proposals, and support-export snapshot fidelity.

## Acceptance evidence

- All 11 rewrite-flow tests pass.
- All 104 `aureline-review` tests pass (including existing landing, workspace, diff, and review-pack suites).
- Workspace compiles with `cargo check`.

## Risks and limits

- This module is **contract and validation only**; it does not shell out to Git or implement a live preview/apply service. The live service will be built in `crates/aureline-git` on top of the existing `history_rewrite` beta contract.
- Provider-hosted merge-queue execution remains out of scope; this module covers local review and checkpoint truth only.
- Cross-repo operations and auto-resolution are explicitly out of scope (see `docs/source_control/m3/history_rewrite_known_limits.md`).

## Stable claims

- Diff-first review gate is observable on all six supported operation kinds.
- Recovery checkpoint summary is required for `reset` and enforced at validation time.
- Restart snapshots in support exports mirror current packet truth.
- Protected-branch blocked flows are non-actionable and surface next-safe paths via command-graph records.
- Sequence-edit proposals carry unique ordinals and remaining-step counts.
