# Change-object, worktree, patch-stack, and sequence-editor worked fixtures

These YAML fixtures exercise the change-object, worktree-lifecycle,
patch-stack, and sequence-editor contract frozen in
[`/docs/vcs/change_stack_contract.md`](../../../docs/vcs/change_stack_contract.md)
and the boundary schemas at
[`/schemas/vcs/change_object.schema.json`](../../../schemas/vcs/change_object.schema.json)
and
[`/schemas/vcs/patch_stack.schema.json`](../../../schemas/vcs/patch_stack.schema.json).

Every fixture is one record validated against `oneOf` in the
appropriate schema. Each carries only opaque workspace / branch /
revision / change-object / patch-stack / patch-stack-member /
sequence-editor-plan / sequence-editor-operation / recovery-object /
approval-ticket / command / actor / run-record / patch-bundle /
patch-stack-snapshot / policy-epoch / build-identity handles plus
monotonic placeholder timestamps and redaction-aware labels — no raw
absolute paths, no raw branch / commit URLs, no raw author identity
strings, no raw commit message bodies, no raw patch bodies, no raw
run-log bodies, no raw approval-ticket bodies, no raw notebook cell
text, no raw terminal bytes, and no raw URLs.

## Change-object fixtures

| Fixture | Class / authorship / landing state | Acceptance bullet |
|---|---|---|
| `change_object_local_solo_with_recovery.yaml` | `commit_authored_locally` / `human_user_authored` / `unsubmitted_local_only` with a `pre_mutation_local_history_checkpoint_minted` recovery object. | Acceptance bullet 1 — every topology-changing flow names a recovery object before mutation; this baseline shows the local commit row pinning a checkpoint. |
| `change_object_ai_branch_agent_authored.yaml` | `change_authored_by_ai_branch_agent` / `ai_branch_agent_authored` / `submitted_review_in_progress` with a non-null `ai_branch_agent_run_ref` and a `pre_mutation_branch_label_anchor_minted` recovery object. | Acceptance bullet 4 — AI branch-agent outputs are attributable through the run-record envelope; the change object cites the run rather than embedding raw run logs. |
| `change_object_orphan_worktree_pending_cleanup.yaml` | `commit_authored_locally` / `human_user_authored` / `abandoned_orphan_pending_cleanup` with `worktree_orphan_cleanup_class = cleanup_export_then_archive` and a non-null `exported_patch_stack_snapshot_ref`. | Acceptance bullet 3 — orphan worktrees export before reaping; silent reaping denies. |
| `change_object_synthesized_during_history_edit.yaml` | `change_synthesized_during_history_edit` / `synthesized_by_history_edit_operation` / `unsubmitted_local_only` with a non-empty `prior_change_object_id_refs` chain and a `pre_mutation_combined_recovery_packet_minted` recovery object. | Acceptance bullet 1 + bullet 4 — sequence-editor synthesized output cites the rollback target so the rewrite is reversible. |
| `change_object_cross_worktree_write_denied.yaml` | `change_object_audit_event_record` carrying `cross_worktree_write_forbidden` denial. | Acceptance bullet 3 — no contract path admits hidden cross-worktree writes. |

## Patch-stack fixtures

| Fixture | Lifecycle / acceptance bullet |
|---|---|
| `patch_stack_multi_member_with_validation_staleness.yaml` | `patch_stack_active_review_open` with three ordered members; immutable base identity; a `pre_mutation_patch_stack_snapshot_minted` recovery object pinned. Acceptance bullet 2 — base identity, landing order, and validation staleness preserved. |
| `patch_stack_member_validation_stale_base_advanced.yaml` | `patch_stack_member_record` whose `validation_staleness_class` is `validation_stale_base_advanced`; chip surfaces honest staleness rather than claiming a fresh run. Acceptance bullet 2 — validation staleness preserved per member. |
| `patch_stack_imported_from_bundle.yaml` | `patch_stack_drafting` lifecycle with `imported_from_patch_stack_snapshot_ref` non-null; provenance attributable through the snapshot ref. Acceptance bullet 2 — rollback / export determinism + import provenance. |
| `patch_stack_member_silent_reorder_denied.yaml` | `patch_stack_audit_event_record` carrying `patch_stack_member_silent_reorder_forbidden` denial. Acceptance bullet 3 — no silent reorder of stack members. |

## Sequence-editor fixtures

| Fixture | Lifecycle / operation / acceptance bullet |
|---|---|
| `sequence_editor_plan_admitted_with_recovery.yaml` | `sequence_plan_admitted` with a non-null `recovery_object_ref`, `actor_ref`, `command_id_ref`, and `approval_ticket_ref`. Acceptance bullet 1 — admission requires a recovery object and full attribution. |
| `sequence_editor_operation_squash_completed.yaml` | `sequence_op_squash` / `op_completed` with a non-null `output_change_object_id_ref` (paired with the synthesized change object fixture). Acceptance bullet 4 — sequence state walks cleanly into the synthesized output. |
| `sequence_editor_plan_paused_pending_conflict.yaml` | `sequence_plan_paused_pending_conflict` with a paused operation citing `op_blocked_pending_conflict` and `conflict_resolution_packet_required_pending_decision`. Acceptance bullet 4 — sequence state escalates into the conflict-resolution path; the recovery object remains pinned so the user can roll back at any time. |
| `sequence_editor_plan_aborted_rolled_back.yaml` | `sequence_plan_aborted_rolled_back_to_recovery_object` citing the same recovery object the plan minted at admission. Acceptance bullet 1 + bullet 4 — abort path is deterministic and resolves to the pre-mutation recovery object. |
| `sequence_editor_plan_admission_without_recovery_denied.yaml` | `sequence_editor_audit_event_record` carrying `sequence_editor_recovery_object_required_before_admission` denial. Acceptance bullet 1 — admission without a recovery object denies. |

## Cross-walk to the spec

- The change-object fixture set covers each `change_object_class`
  reachable today (locally authored, AI branch-agent authored,
  abandoned orphan, history-edit synthesized) plus the explicit
  cross-worktree-write denial path required by the plan's third
  acceptance bullet.
- The patch-stack fixture set covers a multi-member stack with
  preserved base identity and ordered members, a per-member
  validation-staleness row, an imported-from-bundle stack, and the
  silent-reorder denial path so the third acceptance bullet is
  exercised end-to-end.
- The sequence-editor fixture set walks the plan from admission
  (with recovery + attribution) through a completed squash,
  through a paused-pending-conflict pause, through an
  aborted-rolled-back resolution, and ends with the admission-
  without-recovery denial — covering acceptance bullets 1, 2, 4 and
  the conflict / history-edit recovery handoff named in the plan.
- Forward dependency slots (`merge_conflict_class_record_id_ref`
  and `history_edit_recovery_record_id_ref`) are set to `null` on
  every fixture; they will become non-null when the merge /
  conflict-class contract and the dedicated history-edit recovery
  contract land.
