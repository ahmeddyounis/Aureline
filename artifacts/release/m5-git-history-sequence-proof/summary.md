# M5 Git-History and Risky-Mutation Component Matrix

- Packet: `m5-git-history-sequence-component-matrix:frozen:0001`
- Label: `M5 Git-history and risky-mutation component family`
- Frozen: true (review SLO: 168 hours, last reviewed: 2026-07-08T00:00:00Z)
- Rows: 12 components / 7 downgrade states

## Components

- **commit_graph_header** (stable, `distinct-verb`): review `display_only_no_mutation`, binds `schemas/git/git_history_review.schema.json`
- **history_graph_row** (stable, `distinct-verb`): review `display_only_no_mutation`, binds `schemas/git/git_history_review.schema.json`
- **branch_comparison_chip** (stable, `distinct-verb`): review `display_only_no_mutation`, binds `schemas/review/repository-topology.schema.json`
- **worktree_row** (stable, `distinct-verb`): review `display_only_no_mutation`, binds `schemas/review/repository-topology.schema.json`
- **stash_entry** (stable, `distinct-verb`): review `stash_restore_confirm`, binds `schemas/git/stash_entry.schema.json`
- **reflog_recovery_banner** (stable, `distinct-verb`): review `display_only_no_mutation`, binds `schemas/git/recovery_checkpoint.schema.json`
- **rebase_todo_row** (stable, `distinct-verb`): review `sequence_rewrite_confirm`, binds `schemas/git/sequence_edit_session.schema.json`
- **sequence_editor_header** (stable, `distinct-verb`): review `sequence_rewrite_confirm`, binds `schemas/git/sequence_edit_session.schema.json`
- **cherry_pick_revert_review_sheet** (stable, `distinct-verb`): review `explicit_verb_confirm`, binds `schemas/git/history-surgery-review.schema.json`
- **patch_apply_review_sheet** (beta, `distinct-verb`): review `patch_apply_confirm`, binds `schemas/git/history-surgery-review.schema.json`
- **conflict_checkpoint_card** (stable, `distinct-verb`): review `display_only_no_mutation`, binds `schemas/git/conflict_session.schema.json`
- **force_push_review_dialog** (preview, `distinct-verb`): review `force_push_confirm`, binds `schemas/review/sequence-edit-conflict-session-stash-entry-ref-update.schema.json`

## Downgrade vocabulary

- **stale_provider_overlay**: A hosted provider overlay is older than local Git truth and must be labeled, not trusted.
- **detached_or_missing_ref**: The target ref is detached or missing, so exact identity must be spelled out before any action.
- **dirty_or_conflicted_worktree**: The worktree has uncommitted or conflicted changes at the operation target.
- **shallow_or_partial_topology**: History is shallow/partial/sparse here, so the shown graph is incomplete.
- **reflog_only_fallback**: No checkpoint exists; only a reflog-only recovery fallback is offered and must stay visible.
- **approval_invalidated**: A prior approval was invalidated by this change and must be recomputed, never hidden.
- **offline_local_only**: Operating offline / local-only; provider handoff is unavailable and the surface says so.
