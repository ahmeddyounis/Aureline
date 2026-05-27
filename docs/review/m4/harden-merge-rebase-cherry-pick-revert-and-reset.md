# Diff-first review and recovery checkpoints for merge, rebase, cherry-pick, revert, and reset

**Scope:** M04-098 — Harden merge, rebase, cherry-pick, revert, and reset flows with diff-first review and recovery checkpoints.

**Status:** Stable review lane — implemented in `crates/aureline-review`.

## Goal

Every destructive Git rewrite flow must pass through a diff review gate and carry an explicit recovery checkpoint before any ref update is applied. The review surface never collapses diff state, checkpoint state, protected-branch posture, and approval/check freshness into one ambiguous status.

## Design principles

1. **Diff-first review** — No merge, rebase, cherry-pick, revert, or reset may execute until the user has reviewed the diff preview and explicitly approved it. The approval is recorded as a durable [`DiffFirstReviewRecord`].
2. **Recovery checkpoint required** — Destructive operations (especially `reset`) must capture a rollback-safe checkpoint or require an explicit reflog-only disclosure acknowledgment before apply.
3. **Restart-resilient session truth** — Every flow carries a `restart_session_ref` and the support-export packet embeds a `RestartSnapshot` so the same structured session truth can be reopened after an IDE or CLI restart.
4. **Durable sequence-edit proposals** — Interactive rebase and cherry-pick sequences are modeled as [`SequenceEditProposalRecord`] objects with stable IDs, ordered operations, protected-branch blocks, and invalidation state.
5. **Separable inspectable truths** — Flow state, diff review state, checkpoint state, divergence class, protected-branch posture, approval state, and checks-freshness state are all independent fields. No single "status" column hides the underlying truth.
6. **Redaction-safe support export** — Raw paths, branch names, patch bodies, reflog bodies, and URLs are explicitly forbidden from crossing the support boundary.

## Record kinds

| Record kind | Purpose |
|---|---|
| `review_diff_first_rewrite_flow_packet` | Top-level packet consumed by review surfaces and support exports. |
| `review_rewrite_flow_record` | Stable identity, operation provenance, divergence summary, protected-branch posture. |
| `review_diff_first_review_record` | The review gate: diff preview ref, suspicious-content flags, checkpoint requirement. |
| `review_sequence_edit_proposal_record` | Durable ordered operations for interactive rebase / cherry-pick sequences. |
| `review_recovery_checkpoint_summary_record` | Checkpoint state, restore command, disclosure label, offline restorability. |
| `review_rewrite_flow_command_record` | Command-graph operations (preview, approve, capture, apply, abort, continue, skip). |
| `review_rewrite_flow_support_export_packet` | Redaction-safe export with restart snapshot. |
| `review_rewrite_flow_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Operation kinds
- `merge`, `rebase`, `interactive_rebase`, `cherry_pick`, `revert`, `reset`

### Flow states
- `diff_pending_review`, `diff_review_approved`, `diff_review_rejected`, `checkpoint_pending`, `checkpoint_captured`, `executing`, `paused_conflict`, `completed`, `aborted_rolled_back`, `failed_no_changes_made`

### Diff review states
- `pending`, `approved_with_checkpoints`, `rejected`, `requires_manual_review`

### Checkpoint summary states
- `none_required`, `captured_ready`, `captured_pending`, `restored`, `expired`, `reflog_only_acknowledged`, `missing_blocks_apply`

### Protected-branch postures
- `no_protected_refs`, `protected_branch_readonly`, `protected_branch_blocked`, `policy_lock_active`

### Divergence classes
- `no_divergence`, `local_ahead`, `remote_ahead`, `diverged_requires_rebase`, `diverged_requires_merge`

## Key invariants

- `diff_review_state = approved_with_checkpoints` and `checkpoint_required_before_apply = true` requires `checkpoint_state` to be `captured_ready`, `captured_pending`, or `restored`.
- `flow_state` in `executing`, `completed`, or `paused_conflict` requires `diff_review_state = approved_with_checkpoints`.
- `protected_branch_posture = protected_branch_blocked` implies `actionable = false`.
- `reset` operation requires `checkpoint_required_before_apply = true`.
- `interactive_rebase` and `cherry_pick` require a `sequence_edit_proposal`.
- Sequence-edit proposal ordinals must be unique.
- Support export must include `support_export` and `audit_lane` consumer surfaces.
- All `raw_*_export_allowed` flags in support export must be `false`.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-review/src/harden_merge_rebase_cherry_pick_revert_and_reset/mod.rs` |
| Schema | `schemas/review/diff_first_rewrite_flow.schema.json` |
| Fixtures | `fixtures/review/m4/harden_merge_rebase_cherry_pick_revert_and_reset/` |
| Tests | `crates/aureline-review/tests/harden_merge_rebase_cherry_pick_revert_and_reset_alpha.rs` |

## Integration with existing lanes

- Consumes [`ReviewWorkspaceBetaPacket`] from the `workspace` module.
- References [`aureline_git::history_rewrite`] records (recovery checkpoints, sequence-edit sessions, conflict sessions, ref-update proposals) via opaque refs.
- Projects into the same inspector/CLI/support-export surfaces as the `landing` module.

## Verification

```bash
cargo test -p aureline-review --test harden_merge_rebase_cherry_pick_revert_and_reset_alpha
```
