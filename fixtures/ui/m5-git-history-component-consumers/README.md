# M5 Git-history component consumer fixtures

Protected fixtures for the M05-961 shared Git-history component consumer lane
(`add_shared_history_sidebar_review_workspace_command_help_support_and_export_consumers_so_git_history_components_keep_ref_worktree_recovery_language_aligned`).

Each fixture is a full `GitHistoryComponentConsumerPacket` that validates against
`schemas/ui/m5-git-history-component-consumer.schema.json` and proves that the same
Git-history object keeps identical ref/worktree/recovery/verb language across the six
consumer surfaces even as some objects narrow.

- `detached_ref_and_dirty_worktree_identity_narrowed.json` — commit-graph header on a
  detached/missing ref and a history row on a dirty/conflicted worktree, both
  `identity_narrowed`, with the exact target ref spelled out.
- `reflog_only_and_offline_recovery_narrowed.json` — a branch-comparison chip on a
  reflog-only recovery fallback and a sequence-editor header on an offline/local-only
  continuation, with the recovery destination and local continuation kept explicit.

Regenerate with
`GEN_GIT_HISTORY_COMPONENT_CONSUMER_ARTIFACTS=1 cargo test -p aureline-git --lib regenerate_git_history_component_consumer_artifacts`.
