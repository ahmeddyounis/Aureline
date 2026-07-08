# Git-History Component Accessibility, Headless, and Export Parity

- Packet: `git-history-component-accessibility:stable:0001`
- Surface: `Git-history component accessibility, headless, and export parity`
- Accessibility rows: 12 (9 claim-narrowed)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Accessibility rows

- **commit_graph_header** [`row:commit-graph-aligned`]: condition `local_truth_aligned`, claim `recoverable_in_product`
- **history_graph_row** [`row:history-graph-topology-partial`]: condition `repo_topology_partial`, claim `partial_history_only`
- **branch_comparison_chip** [`row:branch-comparison-provider-stale`]: condition `provider_review_state_stale`, claim `locally_recoverable`
- **worktree_row** [`row:worktree-aligned`]: condition `local_truth_aligned`, claim `recoverable_in_product`
- **stash_entry** [`row:stash-aligned`]: condition `local_truth_aligned`, claim `recoverable_in_product`
- **reflog_recovery_banner** [`row:reflog-checkpoint-unavailable`]: condition `checkpoint_recovery_unavailable`, claim `reflog_only_recovery`
- **rebase_todo_row** [`row:rebase-todo-topology-partial`]: condition `repo_topology_partial`, claim `partial_history_only`
- **sequence_editor_header** [`row:sequence-editor-provider-stale`]: condition `provider_review_state_stale`, claim `locally_recoverable`
- **cherry_pick_revert_review_sheet** [`row:cherry-pick-offline`]: condition `offline_local_only`, claim `local_continue_only`
- **patch_apply_review_sheet** [`row:patch-apply-checkpoint-unavailable`]: condition `checkpoint_recovery_unavailable`, claim `reflog_only_recovery`
- **conflict_checkpoint_card** [`row:conflict-checkpoint-offline`]: condition `offline_local_only`, claim `local_continue_only`
- **force_push_review_dialog** [`row:force-push-provider-stale`]: condition `provider_review_state_stale`, claim `locally_recoverable`
