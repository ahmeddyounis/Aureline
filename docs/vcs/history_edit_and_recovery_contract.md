# History-edit, sequence-editor, cherry-pick / revert, and recovery-path contract

This document freezes the cross-tool record family every Aureline
surface reads when it presents a high-trust history-edit operation —
an interactive rebase sequence, a cherry-pick onto an alternate
target, a revert commit preview, a stacked-series restack, or any
single-branch history rewrite — before the surface is free to
collapse rewrite, replay, and revert into one generic "edit history"
button. Every history-edit surface, change-stack panel, AI
branch-agent panel, support / export bundle's git section, and audit
lane reads exactly one record from this family and never invents its
own "rewrite", "replay", "revert", "auto-continue", or "recovery
object" vocabulary.

The machine-readable boundaries are:

- [`/schemas/vcs/recovery_object.schema.json`](../../schemas/vcs/recovery_object.schema.json)
  — the `recovery_object_record` and
  `recovery_object_audit_event_record` shapes plus the closed
  `recovery_object_payload_class`,
  `recovery_object_lifecycle_state`, and
  `recovery_object_topology_change_class` vocabularies. Re-exports
  the `recovery_object_class` upstream vocabulary verbatim from
  [`/schemas/vcs/change_object.schema.json`](../../schemas/vcs/change_object.schema.json).
- [`/schemas/vcs/sequence_step.schema.json`](../../schemas/vcs/sequence_step.schema.json)
  — the `sequence_step_record` and
  `sequence_step_audit_event_record` shapes plus the closed
  `sequence_step_kind_class` (re-exported from the patch-stack
  contract's `sequence_editor_operation_class`),
  `sequence_step_status_class`, `auto_continue_state_class`,
  `object_model_change_class`, `protected_path_blocker_class`,
  `signoff_blocker_class`, and
  `base_or_head_identity_drift_class` vocabularies.
- [`/schemas/vcs/history_edit_review.schema.json`](../../schemas/vcs/history_edit_review.schema.json)
  — the `history_edit_review_record` and
  `history_edit_review_audit_event_record` shapes plus the closed
  `history_edit_review_class`,
  `history_edit_review_lifecycle_state`,
  `publish_impact_class`, `merge_queue_impact_class`, and
  `review_id_impact_class` vocabularies.

Worked cases (an interactive rebase sequence with reword / squash /
drop steps and a pre-mutation combined recovery packet pinned; a
cherry-pick of one commit onto an alternate target with auto-continue
admissible; a revert commit preview that appends a new commit and
keeps the open review thread; a stacked-series restack onto a new
base with publish impact `force_push_required_to_existing_remote`
and merge-queue impact
`review_open_will_invalidate_pipeline`; a conflict-resume case where
the review paused on a conflict, the recovery card cites the
combined recovery packet, and auto-continue is structurally blocked;
a denial when a history-edit review tried to admit without a
recovery object; and a denial when an auto-continue advance was
attempted from a paused state) live under
[`/fixtures/vcs/history_edit_cases/`](../../fixtures/vcs/history_edit_cases/).

The eventual git-service crate's Rust types are the schema of record.
This document and the JSON Schema exports are the cross-tool
boundary every non-owning surface reads. The change-object /
patch-stack / sequence-editor contract
([`/docs/vcs/change_stack_contract.md`](change_stack_contract.md))
and the branch / worktree / history / stash contract
([`/docs/vcs/git_state_and_worktree_contract.md`](git_state_and_worktree_contract.md))
are upstream: every paused-pending-history-edit-recovery row from
those contracts resolves through one `recovery_object_record` plus
one `history_edit_review_record`, and the
`history_edit_recovery_record_id_ref` slots they reserved become
required non-null on rows whose `conflict_handoff_class` is one of
the `history_edit_recovery_packet_*` values. The merge / conflict
contract
([`/docs/vcs/merge_and_conflict_contract.md`](merge_and_conflict_contract.md))
is upstream for any conflict-resume escalation that flows through a
conflict-resolution session; this contract pins the history-edit
review and per-step rows the conflict session pauses against.

Companion artifacts:

- [`/schemas/vcs/change_object.schema.json`](../../schemas/vcs/change_object.schema.json)
  and
  [`/docs/vcs/change_stack_contract.md`](change_stack_contract.md)
  — the change-object family this contract's recovery objects pin
  the pre-mutation state of. A `change_object_record` whose
  `change_object_class` is `change_synthesized_during_history_edit`
  resolves through a `sequence_step_record` whose
  `output_change_object_id_ref` cites the synthesized change.
- [`/schemas/vcs/patch_stack.schema.json`](../../schemas/vcs/patch_stack.schema.json)
  — the patch-stack and sequence-editor family this contract's
  per-step row pairs with through `sequence_editor_plan_id_ref` and
  `sequence_editor_operation_id_ref`. A `sequence_editor_plan_record`
  whose `sequence_editor_lifecycle_state` is past
  `sequence_plan_drafted` cites a recovery object through the
  patch-stack schema's `recovery_object_ref`; this contract resolves
  that ref to one `recovery_object_record`.
- [`/schemas/vcs/conflict_resolution_session.schema.json`](../../schemas/vcs/conflict_resolution_session.schema.json)
  and
  [`/docs/vcs/merge_and_conflict_contract.md`](merge_and_conflict_contract.md)
  — the conflict-resolution session family a paused
  `history_edit_review_record` cites through the
  `conflict_handoff_class` cross-link. The session row pins the
  conflict surface; this contract pins the history-edit review and
  per-step rows the session paused against.
- [`/schemas/vcs/history_operation_state.schema.json`](../../schemas/vcs/history_operation_state.schema.json)
  and
  [`/docs/vcs/git_state_and_worktree_contract.md`](git_state_and_worktree_contract.md)
  — the in-progress history-operation family every active
  history-edit review surfaces against. The branch / worktree /
  history / stash contract reserved
  `history_edit_recovery_record_id_ref` slots that this contract
  fills.
- [`/schemas/recovery/local_history_entry.schema.json`](../../schemas/recovery/local_history_entry.schema.json)
  and
  [`/docs/reliability/local_history_contract.md`](../reliability/local_history_contract.md)
  — the local-history entry that a `recovery_object_record` whose
  `recovery_object_payload_class` is `local_history_checkpoint`
  resolves through.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  — the approval-ticket model every history-edit review and
  recovery-rollback action cites. A history-edit review that admits
  without an approval ticket denies with
  `history_edit_review_attribution_missing`.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — re-exported `freshness_class`, `client_scope`, and
  `redaction_class` vocabularies (ADR-0011). This contract never
  redefines them.
- [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json)
  — re-exported `workspace_trust_state_class` and policy / trust
  envelope (ADR-0001 / ADR-0018). A history-edit confirm never
  appears available under an unset trust decision.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — git
  service architecture, sequence-editor architecture, and
  history-edit recovery architecture.
- `.t2/docs/Aureline_PRD.md` — explicit-recovery,
  no-silent-rewrite, and auto-continue gating MUST / SHOULD
  language.

If this contract disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same
change.

## Why freeze this now

Until this contract lands, every surface that touches a history-edit
operation would be free to invent its own rewrite vocabulary:

- A "rewrite history" button could appear without naming the recovery
  object. The user would press it, find a conflict mid-rebase, and
  discover only after the rebase that no rollback target exists.
- A "cherry-pick" affordance could appear without naming the
  alternate target. The replay would land on whichever base the
  surface happened to read at confirm time, which may not be the
  base the reviewer believed they were replaying onto.
- A "revert" button could rewrite history (a one-shot reset) instead
  of appending a new commit; the open provider review thread would
  silently break.
- An auto-continue executor could advance past a conflict, a
  protected-path edit, a missing signoff, or a changed base / head
  identity without re-confirming with the user. The downstream
  surface would learn the gate was bypassed only through a later
  audit replay.
- A patch-series restack could mutate `base_identity_ref` in place
  on the upstream patch-stack record (see the change-stack
  contract's `patch_stack_base_identity_must_not_mutate_in_place`
  denial), or could leave the prior stack reachable only through a
  reflog the user has no way to navigate.
- A conflict-resume could land on a worktree whose base or head
  identity had drifted since the review was admitted; the resume
  would silently apply against the drifted state and the user would
  discover the drift only when the merge tool produced unexpected
  output.

Freezing one record family
(`recovery_object_record`, `sequence_step_record`,
`history_edit_review_record`) and the closed
recovery-payload, sequence-step-status, auto-continue,
object-model-change, history-edit-review-class, lifecycle, publish-
impact, merge-queue-impact, and review-id-impact vocabularies they
read solves all six problems in one shape.

## Scope

Frozen at this revision:

1. The `recovery_object_record` shape every recovery object reads,
   including:
   - one `recovery_object_payload_class` from the closed six-value
     vocabulary (`local_history_checkpoint`,
     `backup_branch_or_tag_ref`, `reflog_anchor`,
     `export_patch_bundle`, `safe_worktree_clone`,
     `combined_recovery_packet`);
   - the upstream `recovery_object_class` re-exported verbatim from
     the change-object schema, paired through allOf with the payload
     class so the row cannot claim a class it cannot back;
   - the `recovery_object_topology_change_class` from the closed
     six-value vocabulary
     (`topology_change_history_rewrite`,
     `topology_change_revert_commit_append`,
     `topology_change_replay_onto_new_target`,
     `topology_change_patch_series_restack`,
     `topology_change_orphan_worktree_export`,
     `topology_change_branch_label_move`);
   - the `recovery_object_lifecycle_state` from the closed four-value
     vocabulary (`recovery_object_minted_pre_mutation_pinned`,
     `recovery_object_consumed_rolled_back`,
     `recovery_object_superseded_by_newer_recovery_object`,
     `recovery_object_archived_tombstone`);
   - the per-payload-class non-null ref slots
     (`local_history_entry_ref`, `backup_branch_or_tag_ref`,
     `reflog_anchor_ref`, `export_patch_bundle_ref`,
     `safe_worktree_clone_ref`,
     `combined_recovery_packet_member_refs`);
   - the workspace / target-branch-or-worktree / pre-mutation-revision
     locator triple; and
   - the audit-event row shape on the `recovery_object` stream.

2. The `sequence_step_record` shape every per-step review row reads,
   including:
   - the `sequence_step_kind_class` re-exported from the patch-stack
     contract's `sequence_editor_operation_class` so the eleven kinds
     (`sequence_op_pick`, `sequence_op_reword`, `sequence_op_edit`,
     `sequence_op_squash`, `sequence_op_fixup`, `sequence_op_drop`,
     `sequence_op_exec`, `sequence_op_break`, `sequence_op_label`,
     `sequence_op_reset`, `sequence_op_merge_replay`) are read
     mechanically without re-resolving the upstream operation row;
   - the `sequence_step_status_class` from the closed nine-value
     vocabulary (`step_drafted_pending_review`,
     `step_admitted_pending_run`, `step_in_progress_running`,
     `step_paused_pending_conflict`,
     `step_paused_pending_protected_path_review`,
     `step_paused_pending_signoff`,
     `step_paused_pending_base_or_head_identity_changed`,
     `step_completed`, `step_aborted_rolled_back`);
   - the `auto_continue_state_class` from the closed five-value
     vocabulary (`auto_continue_admissible`,
     `auto_continue_blocked_pending_conflict`,
     `auto_continue_blocked_protected_path_modification`,
     `auto_continue_blocked_pending_signoff`,
     `auto_continue_blocked_base_or_head_identity_changed`);
   - the `object_model_change_class` from the closed four-value
     vocabulary (`object_model_pick_in_place_no_change`,
     `object_model_rewrite_history`,
     `object_model_append_new_revert_commit`,
     `object_model_replay_onto_new_target`);
   - the per-blocker class fields
     (`protected_path_blocker_class`, `signoff_blocker_class`,
     `base_or_head_identity_drift_class`) so each auto-continue
     blocker dimension is reviewable independently;
   - the `target_change_object_id_ref` and
     `output_change_object_id_ref` paired through allOf with the
     kind class (matching the patch-stack contract's
     pick/reword/edit/squash/fixup/drop/exec/break/label/reset/
     merge-replay table);
   - the `replay_target_base_identity_ref` (required when
     `object_model_change_class` is
     `object_model_replay_onto_new_target`);
   - the `recovery_object_record_id_ref` (required for any non-pick
     and non-label step past `step_drafted_pending_review`); and
   - the audit-event row shape on the `sequence_step` stream.

3. The `history_edit_review_record` shape every history-edit review
   reads, including:
   - the `history_edit_review_class` from the closed five-value
     vocabulary (`interactive_rebase_sequence_review`,
     `cherry_pick_onto_alternate_target_review`,
     `revert_commit_preview_review`,
     `patch_series_restack_review`,
     `single_branch_history_edit_review`);
   - the dominant `object_model_change_class` for the whole review
     (paired through allOf with the review class so cherry-pick
     paris with replay, revert pairs with append-new-revert-commit,
     interactive rebase / single-branch pair with rewrite-history,
     and patch-series restack pairs with replay);
   - the `history_edit_review_lifecycle_state` from the closed
     ten-value vocabulary (`review_drafted_pending_confirm`,
     `review_admitted_pending_run`, `review_in_progress_running`,
     four `review_paused_*` states, `review_completed_recovery_object_minted`,
     `review_aborted_rolled_back_to_recovery_object`,
     `review_archived_tombstone`);
   - the aggregate `auto_continue_state_class` paired through allOf
     with the lifecycle so a reviewer can read the gate from the
     review row alone;
   - the `publish_impact_class` from the closed five-value vocabulary
     (`publish_impact_local_only_no_remote_publish`,
     `publish_impact_force_push_required_to_existing_remote`,
     `publish_impact_new_branch_publish_only`,
     `publish_impact_remote_unaffected_revert_only`,
     `publish_impact_blocked_by_protected_branch_policy`);
   - the `merge_queue_impact_class` from the closed four-value
     vocabulary (`merge_queue_impact_no_review_open`,
     `merge_queue_impact_review_open_will_invalidate_pipeline`,
     `merge_queue_impact_review_open_will_drift_base_only`,
     `merge_queue_impact_review_open_blocked_by_queue_freeze`);
   - the `review_id_impact_class` from the closed four-value
     vocabulary (`review_id_impact_no_review_open`,
     `review_id_impact_will_preserve_review_id`,
     `review_id_impact_will_invalidate_review_id_new_id_required`,
     `review_id_impact_will_orphan_existing_review_thread`);
   - the target locator fields (target workspace / branch-or-worktree
     / base-identity / replay-target-base-identity / change-object-id
     refs / patch-stack-id ref / sequence-editor-plan-id ref);
   - the `ordered_sequence_step_id_refs` chain into the
     `sequence_step_record` rows;
   - the `recovery_object_record_id_ref` (required past
     `review_drafted_pending_confirm`); and
   - the audit-event row shape on the `history_edit_review` stream.

4. The acceptance invariants this contract enforces:

   - A reviewer can tell what object-model change the operation will
     produce and what recovery object exists before confirm. Every
     `history_edit_review_record` past `review_drafted_pending_confirm`
     resolves to one `object_model_change_class` and cites a non-null
     `recovery_object_record_id_ref`; a missing ref denies with
     `history_edit_review_recovery_object_required_before_admission`.
     Every per-step row past `step_drafted_pending_review` (other
     than pick / label) cites a non-null
     `recovery_object_record_id_ref`; a missing ref denies with
     `sequence_step_recovery_object_required_before_non_pick_admission`.
   - Auto-continue is structurally blocked while conflicts, protected-
     path blockers, missing signoff, or changed base / head identity
     remain unresolved. The schema's allOf gates pair every
     `review_paused_*` lifecycle and every `step_paused_*` status
     with the matching `auto_continue_blocked_*` state, force the
     three blocker-class fields on the per-step row to a
     non-default value when the matching lifecycle is set, and force
     `auto_continue_admissible` to require every blocker class at its
     default plus a `conflict_handoff_class` that is either
     `no_conflict_path_required` or one of the
     `*_admitted_user_resolved` values; a row that mismatches denies
     with `sequence_step_auto_continue_blocked_resolution_required`.
   - Rules separate rewrite-history, new-revert-commit, and
     replay-onto-new-target actions. The
     `object_model_change_class` vocabulary is closed and the
     `history_edit_review_class` ↔ `object_model_change_class`
     pairing is enforced through allOf so a revert preview cannot
     pose as a rewrite, a cherry-pick cannot pose as a rewrite, and
     an interactive rebase cannot pose as a revert preview. The
     publish / merge-queue / review-id impact classes are pinned per
     row so the downstream consequences of the chosen action class
     are mechanical to read.
   - Conflict / resume state is explicit. A
     `review_paused_pending_conflict` lifecycle pairs with
     `conflict_resolution_packet_required_pending_decision`
     `conflict_handoff_class` and at least one
     `step_paused_pending_conflict` step; a paused review that
     resumes flips both the review row's lifecycle and at least one
     step row's status in lockstep. Silent resumption denies with
     `history_edit_review_pause_resolution_required_before_resume`.
   - Abort rolls back to the recovery object the review pinned at
     admission. A `review_aborted_rolled_back_to_recovery_object`
     row MUST cite the same `recovery_object_record_id_ref` it
     pinned at admission and a non-null `aborted_at`; a missing
     recovery object denies with
     `history_edit_review_abort_recovery_object_required`.

Out of scope until a superseding decision row opens:

- Implementing Git plumbing (real refs, real pack files, real
  rebase / cherry-pick / revert plumbing). The contract reserves
  the row shape; the engine is a later lane.
- Implementing a real sequence editor or a real cherry-pick / revert
  / restack tool. The contract reserves the per-step and per-review
  rows the eventual editor binds to.
- Final provider integration behaviour (force-push admission,
  merge-queue restart, review-id replacement). The publish /
  merge-queue / review-id impact classes are reviewable but the
  actual provider interaction stays in the integration lane.
- Cross-repo, multi-base, or distributed history-edit federation.

## 1. The recovery-object record

A recovery object is the explicit pre-mutation handle every
topology-changing operation pins before mutation. The
`recovery_object_record` row is the upstream every change-object,
patch-stack, sequence-editor, history-edit-review, and sequence-step
record cites through `history_edit_recovery_record_id_ref` (or, on
the change-stack contract's per-row `recovery_object_ref`, through
the recovery object id directly).

### 1.1 Payload classes

`recovery_object_payload_class` is closed and exhaustive:

| `recovery_object_payload_class` | Cited ref | Upstream `recovery_object_class` |
|---|---|---|
| `local_history_checkpoint` | `local_history_entry_ref` | `pre_mutation_local_history_checkpoint_minted` |
| `backup_branch_or_tag_ref` | `backup_branch_or_tag_ref` | `pre_mutation_branch_label_anchor_minted` |
| `reflog_anchor` | `reflog_anchor_ref` | `pre_mutation_reflog_anchor_minted` |
| `export_patch_bundle` | `export_patch_bundle_ref` | `pre_mutation_patch_stack_snapshot_minted` |
| `safe_worktree_clone` | `safe_worktree_clone_ref` | `pre_mutation_combined_recovery_packet_minted` |
| `combined_recovery_packet` | `combined_recovery_packet_member_refs` (>= 2) | `pre_mutation_combined_recovery_packet_minted` |

The schema's allOf gates pair every payload class with the matching
upstream class and the matching ref field; a row that mismatches
denies with
`recovery_object_payload_class_must_match_recovery_object_class`.

### 1.2 Lifecycle

`recovery_object_lifecycle_state` is closed:

- `recovery_object_minted_pre_mutation_pinned` — the only state in
  which auto-continue may advance. The row was minted before the
  topology-changing operation and pins a reachable rollback target.
- `recovery_object_consumed_rolled_back` — set after the user
  invoked `rollback_to_pre_mutation_recovery_object` (see the
  change-stack contract's rollback / export class). The row remains
  as a tombstone for audit; cites a non-null `consumed_at`.
- `recovery_object_superseded_by_newer_recovery_object` — set when a
  multi-step plan refreshes the recovery object between steps. The
  row cites the successor through `superseded_by_recovery_object_id_ref`.
- `recovery_object_archived_tombstone` — the long-tail tombstone
  state when retention expires.

### 1.3 Topology-change class

`recovery_object_topology_change_class` is closed and pins which
class of topology-changing operation the recovery object is
attached to. The six values map cleanly to the four
`object_model_change_class` values plus two long-tail cases
(orphan-worktree export, branch-label move) that the change-stack
and branch-row contracts already reserved vocabulary for.

## 2. The sequence-step record

A sequence-step row is the per-step review boundary every
history-edit operation pins. The `sequence_step_record` is the
review-side mirror of the patch-stack contract's
`sequence_editor_operation_record`; it carries the additional
fields the reviewer needs (per-step status, auto-continue state,
object-model change class, blocker classes per dimension) that the
upstream operation row does not.

### 2.1 Status and auto-continue gating

`sequence_step_status_class` and `auto_continue_state_class` pair
through allOf:

| `sequence_step_status_class` | Required `auto_continue_state_class` |
|---|---|
| `step_drafted_pending_review` | (any; review has not yet confirmed) |
| `step_admitted_pending_run` | `auto_continue_admissible` |
| `step_in_progress_running` | `auto_continue_admissible` |
| `step_paused_pending_conflict` | `auto_continue_blocked_pending_conflict` |
| `step_paused_pending_protected_path_review` | `auto_continue_blocked_protected_path_modification` |
| `step_paused_pending_signoff` | `auto_continue_blocked_pending_signoff` |
| `step_paused_pending_base_or_head_identity_changed` | `auto_continue_blocked_base_or_head_identity_changed` |
| `step_completed` | (any; terminal) |
| `step_aborted_rolled_back` | (any; terminal) |

`auto_continue_admissible` further requires every blocker-class
field at its default value
(`no_protected_path_blocker`, `no_signoff_blocker`,
`base_and_head_identity_unchanged_since_admission`) and a
`conflict_handoff_class` that is either `no_conflict_path_required`
or one of the `*_admitted_user_resolved` values.

### 2.2 Object-model change class

`object_model_change_class` is closed:

- `object_model_pick_in_place_no_change` — admissible only on
  `sequence_op_pick`. The row preserves the parent and history.
- `object_model_rewrite_history` — admissible on
  `sequence_op_reword`, `sequence_op_edit`, `sequence_op_squash`,
  `sequence_op_fixup`, `sequence_op_drop`, `sequence_op_merge_replay`.
  The row's confirm rewrites history; the recovery object is the
  rollback target.
- `object_model_append_new_revert_commit` — admissible on
  revert-commit-preview reviews whose underlying step is a
  `sequence_op_pick` of a synthesized revert change; the row's
  confirm appends a new commit and does not rewrite prior history.
- `object_model_replay_onto_new_target` — admissible on
  `sequence_op_pick` and `sequence_op_merge_replay` whose
  `replay_target_base_identity_ref` is non-null. The row's confirm
  replays the cited change onto the alternate target.

### 2.3 Pre-non-pick recovery requirement

A step whose kind class is anything other than `sequence_op_pick` or
`sequence_op_label` and whose status is past
`step_drafted_pending_review` MUST cite a non-null
`recovery_object_record_id_ref`. A missing ref denies with
`sequence_step_recovery_object_required_before_non_pick_admission`.
`sequence_op_pick` and `sequence_op_label` are exempt because a
pick-in-place / label step does not, by itself, change the object
model; the parent review row's `recovery_object_record_id_ref`
remains the upstream rollback target.

## 3. The history-edit review record

A history-edit review row is the explicit confirm-time boundary the
user reads before any topology-changing operation runs. The record
is the answer to seven questions the reviewer must be able to
answer without opening any other object:

1. *Which class of history edit is this?* —
   `history_edit_review_class`.
2. *What object-model change will the confirm produce?* —
   `object_model_change_class`.
3. *Which steps will execute and in which order?* —
   `ordered_sequence_step_id_refs` (each one a `sequence_step_record`).
4. *What is the publish / merge-queue / review-id impact?* —
   `publish_impact_class`, `merge_queue_impact_class`,
   `review_id_impact_class`.
5. *Where is the review in its lifecycle and what is the
   auto-continue gate?* — `history_edit_review_lifecycle_state`
   plus `auto_continue_state_class`.
6. *Is the review clean of conflict, or escalating into the conflict
   / history-edit recovery contracts?* — `conflict_handoff_class`.
7. *What recovery object will the abort roll back to?* —
   `recovery_object_record_id_ref`.

### 3.1 Review-class ↔ object-model pairing

| `history_edit_review_class` | Required `object_model_change_class` |
|---|---|
| `interactive_rebase_sequence_review` | `object_model_rewrite_history` |
| `cherry_pick_onto_alternate_target_review` | `object_model_replay_onto_new_target` |
| `revert_commit_preview_review` | `object_model_append_new_revert_commit` |
| `patch_series_restack_review` | `object_model_replay_onto_new_target` |
| `single_branch_history_edit_review` | `object_model_rewrite_history` |

The schema's allOf gates pin every review-class to its required
object-model class; a row that disagrees denies with
`history_edit_review_class_must_match_object_model_change_class`.

### 3.2 Auto-continue gate at the review level

`auto_continue_state_class` on the review row is the aggregate gate
across every step in the review:

- `auto_continue_admissible` is admissible only when the lifecycle
  is `review_admitted_pending_run` or `review_in_progress_running`.
- The four `auto_continue_blocked_*` values pair through allOf with
  the matching `review_paused_*` lifecycle.
- `review_drafted_pending_confirm` rows MAY carry any aggregate
  auto-continue value (the user has not yet confirmed).
- Terminal lifecycles (`review_completed_recovery_object_minted`,
  `review_aborted_rolled_back_to_recovery_object`,
  `review_archived_tombstone`) are not subject to the gate.

A row that mismatches denies with
`history_edit_review_auto_continue_state_must_match_lifecycle`.

### 3.3 Publish, merge-queue, and review-id impact

`publish_impact_class`, `merge_queue_impact_class`, and
`review_id_impact_class` are reviewable per row. Two impact classes
are admission-blocking on confirm:

- `publish_impact_blocked_by_protected_branch_policy` — the review
  cannot move past `review_drafted_pending_confirm` until the
  protected-branch policy clears (denies with
  `history_edit_review_publish_impact_blocked_by_protected_branch_policy`
  on `review_admitted_pending_run` /
  `review_in_progress_running`).
- `merge_queue_impact_review_open_blocked_by_queue_freeze` — the
  review cannot move past `review_drafted_pending_confirm` until the
  queue freeze clears (denies with
  `history_edit_review_merge_queue_impact_blocked_by_queue_freeze`).

The remaining values are reviewable but not admission-blocking; the
reviewer reads them to understand the downstream consequence.

### 3.4 Pause / resume escalation

A `review_paused_pending_conflict` row MUST cite at least one
`sequence_step_record` whose status is `step_paused_pending_conflict`
and whose `conflict_handoff_class` is
`conflict_resolution_packet_required_pending_decision`. Resuming
the review flips the review row's lifecycle and at least one step
row's status in lockstep; silent resumption (a review that flips
out of pause without resolving the escalation) denies with
`history_edit_review_pause_resolution_required_before_resume`.

A `review_paused_pending_protected_path_review`,
`review_paused_pending_signoff`, or
`review_paused_pending_base_or_head_identity_changed` row pairs
analogously with the matching step paused state and blocker class.

### 3.5 Abort path

A `review_aborted_rolled_back_to_recovery_object` row MUST cite the
recovery object the abort rolled back to and a non-null `aborted_at`.
A row that claims an abort without resolving to the recovery object
denies with `history_edit_review_abort_recovery_object_required`.

## 4. Cherry-pick / revert review fields

`history_edit_review_class` covers cherry-pick and revert through
two distinct values so the reviewer can read the action class
without re-resolving the object-model change class:

- `cherry_pick_onto_alternate_target_review` pins
  `object_model_change_class = object_model_replay_onto_new_target`,
  cites a non-null `replay_target_base_identity_ref`, and may cite
  any `target_change_object_id_refs` count >= 1 (single
  cherry-pick or batch). A missing replay target denies with
  `history_edit_review_replay_target_base_identity_required_for_replay_class`.
- `revert_commit_preview_review` pins
  `object_model_change_class = object_model_append_new_revert_commit`,
  cites the change(s) being reverted in `target_change_object_id_refs`,
  and never rewrites prior history. A row that pairs revert with
  any other object-model class denies with
  `history_edit_review_revert_commit_must_pair_with_append_new_revert_commit`.

The per-step rows the review cites carry the same constraint at the
step level (a sequence step that pairs a revert review with
`object_model_rewrite_history` denies through the step's
`sequence_step_object_model_change_class_must_match_kind` rule
since revert is encoded as a `sequence_op_pick` of a synthesized
revert change_object whose class on the change-object schema is
`change_synthesized_during_history_edit`).

## 5. Patch-series / stack review fields

`patch_series_restack_review` is the review class for stack-wide
restacks. The row MUST cite:

- a non-null `target_patch_stack_id_ref` (the upstream
  `patch_stack_record` the restack operates on);
- a non-null `replay_target_base_identity_ref` (the new base);
- `object_model_change_class = object_model_replay_onto_new_target`;
- a non-null `recovery_object_record_id_ref` past
  `review_drafted_pending_confirm` (the patch-stack contract's
  rule that base identity must not mutate in place is enforced
  upstream — see
  `patch_stack_base_identity_must_not_mutate_in_place`);
- a non-empty `ordered_sequence_step_id_refs` list (one step per
  member; the per-step row's `object_model_change_class` may be
  `object_model_replay_onto_new_target` for each member).

The publish / merge-queue / review-id impact classes are read at the
review level: a typical restack against an already-pushed branch
pins
`publish_impact_force_push_required_to_existing_remote`,
`merge_queue_impact_review_open_will_invalidate_pipeline`, and
`review_id_impact_will_invalidate_review_id_new_id_required` (or
`will_orphan_existing_review_thread` when the provider does not
recreate the review on force-push).

## 6. Audit streams

Three audit streams are reserved by this contract:

- `recovery_object_audit_event` — closed event-id vocabulary
  including `recovery_object_minted`,
  `recovery_object_pinned_to_topology_change`,
  `recovery_object_rollback_invoked`,
  `recovery_object_consumed`, `recovery_object_superseded`,
  `recovery_object_archived`,
  `recovery_object_audit_denial_emitted`. Denial events MUST cite
  one denial reason from the `recovery_object_denial_reason`
  vocabulary.
- `sequence_step_audit_event` — closed event-id vocabulary
  including `sequence_step_drafted`, `sequence_step_admitted`,
  `sequence_step_started`, `sequence_step_paused`,
  `sequence_step_resumed`, `sequence_step_completed`,
  `sequence_step_aborted_rolled_back`,
  `sequence_step_recovery_object_pinned`,
  `sequence_step_audit_denial_emitted`. Denial events MUST cite one
  denial reason from the `sequence_step_denial_reason` vocabulary.
- `history_edit_review_audit_event` — closed event-id vocabulary
  including `history_edit_review_drafted`,
  `history_edit_review_admitted`, `history_edit_review_started`,
  `history_edit_review_paused`, `history_edit_review_resumed`,
  `history_edit_review_completed`,
  `history_edit_review_aborted_rolled_back`,
  `history_edit_review_recovery_object_pinned`,
  `history_edit_review_archived`,
  `history_edit_review_audit_denial_emitted`. Denial events MUST
  cite one denial reason from the
  `history_edit_review_denial_reason` vocabulary.

Adding a new denial reason or a new audit-event id is additive-minor
and bumps the per-record schema-version const; repurposing an
existing value is breaking and requires a new decision row.

## 7. Redaction posture

Every record published against this contract carries one
`redaction_class` from the re-exported capability-lifecycle
vocabulary. Defaults:

- Local-only recovery objects, sequence-step rows, and history-edit
  review rows default to `metadata_safe_default`.
- Rows whose actor or approval ticket touches a credentialed flow
  MUST raise to `operator_only_restricted`.
- Support exports of any row MUST raise to
  `internal_support_restricted`.
- Recovery objects whose payload class is `safe_worktree_clone` and
  whose worktree touches operator-only state MUST raise to
  `operator_only_restricted`.

Raw absolute paths, raw branch / commit URLs, raw author identity
strings, raw commit message bodies, raw patch bodies, raw run-log
bodies, raw approval-ticket bodies, raw notebook cell text, raw
terminal bytes, and raw URLs never appear on any record published
against this contract.

## 8. Acceptance cross-walk

| Acceptance bullet from the plan | Where it lands |
|---|---|
| A reviewer can tell what object model change the operation will produce and what recovery object exists before confirm. | §3 review record + §1 recovery-object record + the `history_edit_review_recovery_object_required_before_admission`, `sequence_step_recovery_object_required_before_non_pick_admission`, and `history_edit_review_class_must_match_object_model_change_class` denials. Fixtures `interactive_rebase_sequence_review.yaml`, `cherry_pick_onto_alternate_target_review.yaml`, `revert_commit_preview_review.yaml`, and `recovery_object_combined_packet.yaml`. |
| Auto-continue is structurally blocked while conflicts, protected-path blockers, missing signoff, or changed base/head identity remain unresolved. | §2.1 status / auto-continue gate + §3.2 review-level auto-continue gate + the `sequence_step_auto_continue_blocked_resolution_required` and `history_edit_review_auto_continue_state_must_match_lifecycle` denials. Fixtures `conflict_resume_with_recovery_card.yaml` and `audit_history_edit_review_admission_without_recovery_denied.yaml`. |
| Fixtures cover at least: interactive rebase sequence, cherry-pick onto alternate target, revert commit preview, stacked-series restack impact, and conflict-resume with recovery card. | Fixtures `interactive_rebase_sequence_review.yaml`, `cherry_pick_onto_alternate_target_review.yaml`, `revert_commit_preview_review.yaml`, `patch_series_restack_review.yaml`, and `conflict_resume_with_recovery_card.yaml`. |

## 9. Versioning

Each schema in this family carries a document-level
`*_schema_version` const. Adding a new enum value, a new optional
property, or a new additive sub-record is additive-minor and bumps
the relevant `*_schema_version` const. Repurposing an existing value
is breaking and requires a new decision row. The schemas join the
`vcs` family row in
[`artifacts/governance/schema_families.yaml`](../../artifacts/governance/schema_families.yaml)
and each artifact joins
[`artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
in the same change.
