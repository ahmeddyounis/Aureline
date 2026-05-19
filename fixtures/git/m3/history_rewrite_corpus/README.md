# History-rewrite, stash, reflog, and conflict-session conformance corpus

This corpus is the failure / recovery drill harness for the risky Git
mutation lane. It complements the smaller `history_rewrite_and_stash/`
case set with explicit drills for restart, alternate-worktree fallback,
protected-branch / policy / collaboration blocks, reflog-only recovery
disclosure, stash apply / pop / drop / branch-from semantics, and the
negative invariants that prove Aureline does not silently widen scope,
discard stash provenance, or mislabel a reflog-only path as a
checkpoint-backed recovery.

Every drill is loaded by the conformance harness at
`crates/aureline-qe/src/git_history_rewrite/`. Positive drills are
expected to validate, project, and satisfy the projection expectations
listed in `manifest.json`. Negative drills are expected to FAIL
validation with the recorded `expected_failure_substring`.

Schemas: `schemas/git/{conflict_session,sequence_edit_session,stash_entry,recovery_checkpoint}.schema.json`.
Reviewer guidance: `docs/source_control/m3/history_rewrite_beta.md`.
Known limits: `docs/source_control/m3/history_rewrite_known_limits.md`.
Conformance artifact: `artifacts/source_control/m3/history_rewrite_conformance.md`.

## Coverage axes

| Axis | Drills |
| --- | --- |
| Continue / skip / abort semantics | `cherry_pick.conflict.skip`, `cherry_pick.conflict.abort`, `revert.conflict.continue` |
| External-tool handoff (gate denied) | `rebase.conflict.external_handoff` |
| Restart + alternate-worktree fallback | `rebase.conflict.resumed_after_restart` |
| Interactive rebase / autosquash | `interactive_rebase.autosquash.paused`, `interactive_rebase.completed_admitted` |
| Stash capture / pop / drop / promote | `stash.captured_unapplied`, `stash.applied_popped`, `stash.dropped`, plus existing `branch_from_stash_promoted` |
| Reflog-only recovery (non-force) | `ref_update.applied.reflog_only_non_force`, `stash.dropped` |
| Captured-checkpoint required for force-move | `ref_update.applied.force_move_with_checkpoint` |
| Protected-branch blocks | `ref_update.branch_delete.protected_blocked` + existing protected-branch fixture |
| Policy / collaboration blocks | `ref_update.policy_admin_lock.blocked`, `ref_update.collaboration.blocked` |
| Recovery-checkpoint restored | `recovery_checkpoint.restored` |
| Negative — scope widen via mislabel | `negative.reflog_relabeled_as_checkpoint` |
| Negative — stash provenance loss | `negative.lost_stash_provenance` |
| Negative — force-move without checkpoint | `negative.force_move_applied_with_reflog_only` |
| Negative — blocked without safe path | `negative.blocked_proposal_without_next_safe_path` |
| Negative — raw body export | `negative.raw_patch_body_exported` |
| Negative — apply while still blocked | `negative.silent_widen_scope_apply_still_blocked` |

## Running the corpus

```
cargo test -p aureline-qe --test git_history_rewrite_conformance
```

The crate also exposes the corpus loader + projection assertions as a
library so other harnesses (UI checks, support-export parity reviews)
can quote the same drill matrix.

## Redaction guarantees

Every drill keeps `raw_path_export_allowed`,
`raw_branch_name_export_allowed`, `raw_patch_body_export_allowed`,
`raw_reflog_body_export_allowed`, and `raw_stash_body_export_allowed`
all false. Negative drill `negative.raw_patch_body_exported` is the
single exception used to prove the export-boundary check rejects raw
patch bodies; it must always fail validation.
