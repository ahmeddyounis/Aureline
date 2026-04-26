# Git local-vs-provider truth, branch / worktree / history / stash worked fixtures

These YAML fixtures exercise the branch-row, worktree-row, history-
operation-state, and stash-row contract frozen in
[`/docs/vcs/git_state_and_worktree_contract.md`](../../../docs/vcs/git_state_and_worktree_contract.md)
and the boundary schemas at
[`/schemas/vcs/branch_row.schema.json`](../../../schemas/vcs/branch_row.schema.json),
[`/schemas/vcs/worktree_row.schema.json`](../../../schemas/vcs/worktree_row.schema.json),
and
[`/schemas/vcs/history_operation_state.schema.json`](../../../schemas/vcs/history_operation_state.schema.json).

Every fixture is one record validated against `oneOf` in the
appropriate schema. Each carries only opaque workspace / branch /
worktree / revision / change-object / history-operation /
recovery-object / approval-ticket / command / actor / patch-stack-
snapshot / local-history / review-workspace / policy-epoch handles
plus monotonic placeholder timestamps and redaction-aware labels —
no raw absolute paths, no raw branch / commit / remote URLs, no raw
author identity strings, no raw commit message bodies, no raw patch
bodies, no raw stash payload bytes, no raw provider rule bodies, and
no raw approval-ticket bodies.

## Branch-row fixtures

| Fixture | Class / risk / acceptance bullet |
|---|---|
| `branch_row_local_only_with_unpublished_commits.yaml` | `local_branch_no_remote_tracking` / `unpublished_commits_present_local_only` with a `recovery_local_history_checkpoint_minted_before_switch` recovery action. Acceptance bullet 1 — reviewers can answer where the change lives, owns the directory, has unpublished commits, and how to recover before switching. |
| `branch_row_tracking_remote_with_provider_overlay_layered.yaml` | `local_branch_tracking_remote_in_sync` with a fresh remote-tracking envelope and a `provider_overlay_visual_secondary` block resolving `overlay_visual_secondary_provider_authoritative_for_provider_owned_rows`. Acceptance bullet 3 — provider review chip layered over local branch truth without replacing the local label. |
| `branch_row_detached_head_state.yaml` | `detached_head_state_no_branch_label` pinned in a linked worktree; remote-tracking envelope and provider overlay both null. Acceptance bullet 3 — detached-head fixture coverage. |

## Worktree-row fixtures

| Fixture | Kind / lifecycle / acceptance bullet |
|---|---|
| `worktree_row_linked_secondary_checkout.yaml` | `worktree_linked_secondary_checkout` / `worktree_registered_active_clean` bound to a feature branch. Acceptance bullet 3 — linked worktree fixture coverage. |
| `worktree_row_sparse_workset_limited_cleanliness.yaml` | `worktree_linked_secondary_checkout` / `worktree_registered_active_clean` with `worktree_workset_cleanliness_class = sparse_worktree_workset_limited_cleanliness`. Acceptance bullet 3 — sparse worktree with limited cleanliness fixture coverage. |
| `worktree_row_pending_removal_metadata_vs_filesystem_review_sheet.yaml` | `worktree_pending_removal_user_review_required` paired with `worktree_removal_class = worktree_removal_metadata_only_admin_directory_pruned` and `review_sheet_metadata_only_vs_filesystem_deletion_distinction_required`. Acceptance bullet 2 — metadata-only / filesystem-deletion / path-preserve distinction surfaced. |
| `worktree_row_generic_removal_denied.yaml` | `worktree_row_audit_event_record` carrying `worktree_removal_generic_action_forbidden` denial. Acceptance bullet 2 — single generic remove action is mechanically forbidden. |

## History-operation-state fixtures

| Fixture | Kind / state / acceptance bullet |
|---|---|
| `history_operation_state_rebase_paused_pending_conflict.yaml` | `rebase_interactive_in_progress` / `paused_pending_conflict_resolution` paired with `conflict_resolution_packet_required_pending_decision`, `banner_in_progress_operation_resume_or_abort`, and `review_sheet_in_progress_operation_resume_or_abort`. Acceptance bullet 1 — reviewers can answer how to recover before rebasing. |
| `history_operation_state_bisect_in_progress.yaml` | `bisect_in_progress` / `running_no_user_action_required` on a detached head (no target branch) with `banner_bisect_next_step_required`. Acceptance bullet 1 — branch-switch / worktree-removal review sheets know the worktree is held by the bisect. |

## Stash-row fixtures

| Fixture | Class / lifecycle / acceptance bullet |
|---|---|
| `stash_row_pinned_to_base_ref.yaml` | `stash_authored_locally` / `stash_active_apply_admissible` with both `base_branch_or_worktree_ref` and `base_revision_id_ref` non-null so apply against a drifted base is mechanically forbidden. Acceptance bullet 3 — stash with base ref fixture coverage. |

## Cross-walk to the spec

- The branch-row fixture set covers a local-only branch with
  unpublished commits and a recovery prompt before switch
  (`branch_row_local_only_with_unpublished_commits.yaml`), a tracking
  branch with a fresh provider overlay layered visually secondary
  (`branch_row_tracking_remote_with_provider_overlay_layered.yaml`),
  and the detached-head state required by the third acceptance
  bullet (`branch_row_detached_head_state.yaml`).
- The worktree-row fixture set covers a linked secondary checkout
  (`worktree_row_linked_secondary_checkout.yaml`), a sparse worktree
  with workset-limited cleanliness explicitly labelled
  (`worktree_row_sparse_workset_limited_cleanliness.yaml`), the
  pending-removal review sheet that distinguishes metadata-only from
  filesystem-deletion
  (`worktree_row_pending_removal_metadata_vs_filesystem_review_sheet.yaml`),
  and the audit denial when a downstream surface tried a single
  generic remove (`worktree_row_generic_removal_denied.yaml`).
- The history-operation-state fixture set walks an in-progress
  rebase paused on a conflict
  (`history_operation_state_rebase_paused_pending_conflict.yaml`)
  and an in-progress bisect on a detached head
  (`history_operation_state_bisect_in_progress.yaml`); both pin
  recovery objects so abort is deterministic and downstream
  branch-switch / worktree-removal review sheets know the worktree
  is held by the operation.
- The stash-row fixture
  (`stash_row_pinned_to_base_ref.yaml`) shows the base-ref invariant
  that mechanically prevents silent apply against a drifted base.
- Forward dependency slots (`merge_conflict_class_record_id_ref`
  and `history_edit_recovery_record_id_ref`) are set to `null` on
  every fixture; they will become non-null when the merge /
  conflict-class contract and the dedicated history-edit recovery
  contract land.
