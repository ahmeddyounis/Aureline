# Shared Git-History Component Consumers: Ref, Worktree, and Recovery Parity

- Packet: `git-history-component-consumer:stable:0001`
- Surface: `Shared Git-history component consumers`
- Consumer bindings: 24 (18 narrowed)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Consumer bindings

- **Commit graph: main** [`bind:cgh-1:sidebar`]: component `commit_graph_header` on `history_sidebar`, mode `full_parity`
- **Commit graph: main** [`bind:cgh-1:help`]: component `commit_graph_header` on `command_help`, mode `full_parity`
- **History row: 4f2a9c1** [`bind:hgr-1:sidebar`]: component `history_graph_row` on `history_sidebar`, mode `full_parity`
- **History row: 4f2a9c1** [`bind:hgr-1:export`]: component `history_graph_row` on `exported_recovery_packet`, mode `full_parity`
- **Compare: feature/queue vs main** [`bind:bcc-1:sidebar`]: component `branch_comparison_chip` on `history_sidebar`, mode `identity_narrowed`
- **Compare: feature/queue vs main** [`bind:bcc-1:review`]: component `branch_comparison_chip` on `review_workspace_banner`, mode `identity_narrowed`
- **Worktree: ../hotfix** [`bind:wtr-1:sidebar`]: component `worktree_row` on `history_sidebar`, mode `identity_narrowed`
- **Worktree: ../hotfix** [`bind:wtr-1:support`]: component `worktree_row` on `support_bundle`, mode `identity_narrowed`
- **Stash: stash@{0}** [`bind:se-1:sheet`]: component `stash_entry` on `risky_mutation_sheet`, mode `identity_narrowed`
- **Stash: stash@{0}** [`bind:se-1:sidebar`]: component `stash_entry` on `history_sidebar`, mode `identity_narrowed`
- **Reflog recovery: HEAD@{2}** [`bind:rrb-1:sheet`]: component `reflog_recovery_banner` on `risky_mutation_sheet`, mode `recovery_narrowed`
- **Reflog recovery: HEAD@{2}** [`bind:rrb-1:support`]: component `reflog_recovery_banner` on `support_bundle`, mode `recovery_narrowed`
- **Rebase step: pick 4f2a9c1** [`bind:rtr-1:sheet`]: component `rebase_todo_row` on `risky_mutation_sheet`, mode `identity_narrowed`
- **Rebase step: pick 4f2a9c1** [`bind:rtr-1:review`]: component `rebase_todo_row` on `review_workspace_banner`, mode `identity_narrowed`
- **Rebase sequence onto main** [`bind:seh-1:sheet`]: component `sequence_editor_header` on `risky_mutation_sheet`, mode `full_parity`
- **Rebase sequence onto main** [`bind:seh-1:help`]: component `sequence_editor_header` on `command_help`, mode `full_parity`
- **Cherry-pick 91ba0de** [`bind:cprs-1:sheet`]: component `cherry_pick_revert_review_sheet` on `risky_mutation_sheet`, mode `recovery_narrowed`
- **Cherry-pick 91ba0de** [`bind:cprs-1:review`]: component `cherry_pick_revert_review_sheet` on `review_workspace_banner`, mode `recovery_narrowed`
- **Apply 0001-add-retry.patch** [`bind:pars-1:sheet`]: component `patch_apply_review_sheet` on `risky_mutation_sheet`, mode `identity_narrowed`
- **Apply 0001-add-retry.patch** [`bind:pars-1:support`]: component `patch_apply_review_sheet` on `support_bundle`, mode `identity_narrowed`
- **Conflict checkpoint cp-771** [`bind:ccc-1:sheet`]: component `conflict_checkpoint_card` on `risky_mutation_sheet`, mode `recovery_narrowed`
- **Conflict checkpoint cp-771** [`bind:ccc-1:export`]: component `conflict_checkpoint_card` on `exported_recovery_packet`, mode `recovery_narrowed`
- **Force-push feature/queue** [`bind:fprd-1:sheet`]: component `force_push_review_dialog` on `risky_mutation_sheet`, mode `local_continue_fallback`
- **Force-push feature/queue** [`bind:fprd-1:export`]: component `force_push_review_dialog` on `exported_recovery_packet`, mode `local_continue_fallback`
