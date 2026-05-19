# History-rewrite, stash, reflog, and conflict-session conformance evidence

This artifact is the release-consumable conformance evidence for the
risky Git mutation lane. Every claimed beta history-rewrite, stash,
reflog, or conflict-session flow has at least one drill in
`fixtures/git/m3/history_rewrite_corpus/`; the drills are executed by
`crates/aureline-qe/src/git_history_rewrite/` and replayed by
`cargo test -p aureline-qe --test git_history_rewrite_conformance`.

The corpus is owned by the QE crate so the same fixture matrix can
gate desktop projections, CLI / headless mirrors, and support-export
parity reviews from one shared truth.

## Coverage matrix

| Axis (Spec §) | Drill ids | Outcome anchored |
| --- | --- | --- |
| Continue / skip / abort semantics across operations | `cherry_pick.conflict.skip`, `cherry_pick.conflict.abort`, `revert.conflict.continue`, `revert.conflict.abort` (existing fixture), `cherry_pick.conflict.continue_after_resolve` (existing fixture) | Conflict-session lifecycle is one of `continuing_after_resolution`, `skipped_conflicted_step`, `aborted_rolled_back`; destructive-gate flag matches the lifecycle. |
| Interactive rebase + autosquash sequence edit | `interactive_rebase.autosquash.paused`, `interactive_rebase.completed_admitted`, existing `interactive_rebase_sequence_running` | Sequence-edit lifecycle pins `paused_for_conflict`, `running`, `completed_admitted`; verbs include `fixup`, `squash`, `drop`. |
| Stash apply / pop / drop / branch-from | `stash.captured_unapplied`, `stash.applied_popped`, `stash.dropped`, existing `stash_apply_with_conflict` and `branch_from_stash_promoted` | Stash lifecycle covers every state except the dropped-without-acknowledgement negative; reflog-only acknowledgement gates the `dropped` drill. |
| Captured-checkpoint vs reflog-only recovery | `ref_update.applied.reflog_only_non_force`, `ref_update.applied.force_move_with_checkpoint`, existing `reset_hard_reflog_only_acknowledged` and `recovery_checkpoint_captured` | Force-moves require `recovery_checkpoint_captured`; non-force reset may use `reflog_only_disclosure_acknowledged`. |
| Recovery-checkpoint restored | `recovery_checkpoint.restored` | Restored lifecycle keeps a deterministic offline rollback path. |
| Protected-branch / policy / collaboration blocks | `ref_update.policy_admin_lock.blocked`, `ref_update.collaboration.blocked`, `ref_update.branch_delete.protected_blocked`, existing `ref_update_protected_branch_blocked` | Every block class is exercised; every blocked proposal publishes at least one `next_safe_path`. |
| Alternate-worktree fallback acceptance | `ref_update.alternate_worktree.accepted` | `next_safe_path_accepted` audit event records the user's chosen alternate-worktree route. |
| Session restart / restore behavior | `rebase.conflict.resumed_after_restart` | `session_resumed_after_restart` audit event present; checkpoint pin survived the restart; alternate worktree honored. |
| External-tool handoff (gate denied) | `rebase.conflict.external_handoff` | `external_handoff_pending` posture denies the destructive gate. |
| Negative — scope widen via posture mislabel | `negative.reflog_relabeled_as_checkpoint` | Validation rejects with `recovery_checkpoint_captured posture requires …`. |
| Negative — stash provenance lost | `negative.lost_stash_provenance` | Validation rejects `promoted_to_branch` without `promoted_branch_ref`. |
| Negative — force-move applied without captured checkpoint | `negative.force_move_applied_with_reflog_only` | Validation rejects with `force-move` substring. |
| Negative — blocked proposal without next-safe path | `negative.blocked_proposal_without_next_safe_path` | Validation rejects (no `next-safe` path published). |
| Negative — raw body export | `negative.raw_patch_body_exported` | Validation rejects raw-body export flag. |
| Negative — silent scope widen on apply | `negative.silent_widen_scope_apply_still_blocked` | Validation rejects `applied` when a non-`no_block` block is still active. |

## Cross-surface projection check

The drills are read by every surface that quotes a history-rewrite
record:

- Desktop history / conflict / sequence / stash panels: read the
  projection's `display_label`, `summary`, `lifecycle_state`,
  `recovery_posture_class`, `blocks_summary`, and
  `next_safe_path_classes`.
- CLI / headless mirrors: print the same fields in plain text.
- Support exports: quote `support_export_refs`, `redaction_class`,
  and the `raw_*_export_allowed` flags (all `false`).
- Audit lane: reads `audit_event_ids` to confirm restart-safe
  session provenance, action lineage, and next-safe-path offers all
  survived restart.

The conformance harness asserts the projection truth for every
drill, so a drift in any surface's read path appears as a failure on
the corresponding drill rather than silently passing through.

## Replay

```
cargo test -p aureline-qe --test git_history_rewrite_conformance
```

The corpus manifest at
`fixtures/git/m3/history_rewrite_corpus/manifest.json` is the
canonical pass / fail input; CI consumers SHOULD treat any
`failures()` returned by `run_corpus_from_repo_root` as a beta
release blocker.

## Known limits

See `docs/source_control/m3/history_rewrite_known_limits.md` for
the explicit beta-out-of-scope items and the next-milestone
dependencies that must land before claimed beta surfaces grow past
the local / worktree boundary.
