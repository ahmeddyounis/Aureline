# History-edit, sequence-step, cherry-pick / revert, and recovery-object worked fixtures

These YAML fixtures exercise the history-edit-review,
sequence-step, and recovery-object contract frozen in
[`/docs/vcs/history_edit_and_recovery_contract.md`](../../../docs/vcs/history_edit_and_recovery_contract.md)
and the boundary schemas at
[`/schemas/vcs/recovery_object.schema.json`](../../../schemas/vcs/recovery_object.schema.json),
[`/schemas/vcs/sequence_step.schema.json`](../../../schemas/vcs/sequence_step.schema.json),
and
[`/schemas/vcs/history_edit_review.schema.json`](../../../schemas/vcs/history_edit_review.schema.json).

Every fixture is one record validated against `oneOf` in the
appropriate schema. Each carries only opaque workspace / branch /
worktree / revision / change-object / patch-stack / sequence-editor
/ recovery-object / approval-ticket / command / actor / policy-epoch
handles plus monotonic placeholder timestamps and redaction-aware
labels — no raw absolute paths, no raw branch / commit / remote URLs,
no raw author identity strings, no raw commit message bodies, no
raw patch bodies, no raw run-log bodies, no raw approval-ticket
bodies, no raw notebook cell text, no raw terminal bytes, and no
raw URLs.

## Recovery-object fixtures

| Fixture | Class / acceptance bullet |
|---|---|
| `recovery_object_combined_packet.yaml` | `recovery_object_record` with `recovery_object_payload_class = combined_recovery_packet` citing three sibling recovery objects (reflog anchor, patch-stack snapshot, local-history checkpoint). Acceptance bullet 1 — recovery object exists before confirm. |

## History-edit review fixtures

| Fixture | Class / lifecycle / acceptance bullet |
|---|---|
| `interactive_rebase_sequence_review.yaml` | `interactive_rebase_sequence_review` / `review_admitted_pending_run` with three steps and a combined recovery packet pinned. Publish impact is force-push-to-existing-remote, merge-queue impact is pipeline-invalidate, review-id impact is replace-review-id. Acceptance bullet 1 + bullet 3 — interactive rebase sequence fixture coverage. |
| `cherry_pick_onto_alternate_target_review.yaml` | `cherry_pick_onto_alternate_target_review` / `review_admitted_pending_run` replaying one hotfix change onto an alternate release-lane base; publish impact is new-branch-publish-only and there is no open review. Acceptance bullet 3 — cherry-pick onto alternate target fixture coverage. |
| `revert_commit_preview_review.yaml` | `revert_commit_preview_review` / `review_admitted_pending_run` appending a new revert commit on main; review-id is preserved and remote is unaffected. Acceptance bullet 1 + bullet 3 — revert commit preview fixture coverage; rules separating new-revert-commit, rewrite-history, and replay-onto-new-target. |
| `patch_series_restack_review.yaml` | `patch_series_restack_review` / `review_admitted_pending_run` restacking three patch-stack members onto an advanced base; publish / merge-queue / review-id impact classes pin force-push-to-existing-remote, pipeline-invalidate, and replace-review-id. Acceptance bullet 3 — stacked-series restack impact fixture coverage. |
| `conflict_resume_with_recovery_card.yaml` | `interactive_rebase_sequence_review` / `review_paused_pending_conflict` paired with `auto_continue_blocked_pending_conflict` and `conflict_resolution_packet_required_pending_decision`; recovery card cites the combined recovery packet pinned at admission. Acceptance bullet 2 + bullet 3 — auto-continue is structurally blocked while conflict remains unresolved; conflict-resume with recovery card fixture coverage. |

## Sequence-step fixtures

| Fixture | Status / acceptance bullet |
|---|---|
| `sequence_step_squash_paused_pending_signoff.yaml` | `sequence_step_record` for a squash step paused pending an owner-signoff blocker; pairs `step_paused_pending_signoff` with `auto_continue_blocked_pending_signoff` and `signoff_required_owner_signoff_missing`. Acceptance bullet 2 — auto-continue is structurally blocked while missing signoff remains unresolved. |

## Audit-denial fixtures

| Fixture | Denial reason / acceptance bullet |
|---|---|
| `audit_history_edit_review_admission_without_recovery_denied.yaml` | `history_edit_review_audit_event_record` carrying the `history_edit_review_recovery_object_required_before_admission` denial when a downstream surface tried to admit a review without a recovery object. Acceptance bullet 1 + bullet 2 — recovery object exists before confirm; auto-continue is structurally blocked while the recovery object is missing. |

## Cross-walk to the spec

- The recovery-object fixture set covers the combined-packet payload
  class (the most general case spanning multiple recovery surfaces);
  the remaining five payload classes
  (`local_history_checkpoint`, `backup_branch_or_tag_ref`,
  `reflog_anchor`, `export_patch_bundle`, `safe_worktree_clone`)
  are exercised through the schema's allOf gates which enforce the
  per-class non-null ref pairing whenever a row is admitted, and
  through the `recovery_object_record_id_ref` cited by the review
  fixtures.
- The history-edit review fixture set covers all four object-model
  change classes
  (`object_model_rewrite_history`,
  `object_model_replay_onto_new_target`,
  `object_model_append_new_revert_commit`,
  `object_model_pick_in_place_no_change` exercised at the
  per-step row level), all five history-edit review classes,
  and the four review-paused / auto-continue-blocked pairings.
- The sequence-step fixture covers a paused-pending-signoff status;
  the schema's allOf gates pair the remaining three blocker classes
  (`auto_continue_blocked_pending_conflict`,
  `auto_continue_blocked_protected_path_modification`,
  `auto_continue_blocked_base_or_head_identity_changed`) with their
  matching `step_paused_*` status and blocker class whenever a row is
  admitted.
- The audit-denial fixture covers the rule that admission past
  `review_drafted_pending_confirm` requires a recovery object.
- Forward-dependency slots on upstream contracts
  (`history_edit_recovery_record_id_ref` on the change-stack and
  branch / worktree / history / stash contracts) are now consumable
  through the records published in this contract; until those
  upstream contracts begin citing the recovery-object id, those
  slots remain nullable on existing rows.
