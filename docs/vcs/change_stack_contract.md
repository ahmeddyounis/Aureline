# Change-object, worktree, patch-stack, and sequence-editor contract

This document freezes the cross-tool change-object family every Aureline
surface reads when it presents a local change, an in-flight worktree, a
patch stack, an AI branch-agent run, an imported patch bundle, or a
history-rewrite plan. Every change a reviewer, automation surface, or
support exporter can name resolves to one `change_object_record`. Every
ordered stack of changes resolves to one `patch_stack_record` and a row
per member through `patch_stack_member_record`. Every history-edit plan
resolves to one `sequence_editor_plan_record` and a row per operation
through `sequence_editor_operation_record`. No surface mints a parallel
"change", "stack", "rebase plan", "branch agent run", or "orphan
worktree" vocabulary.

The machine-readable boundaries are:

- [`/schemas/vcs/change_object.schema.json`](../../schemas/vcs/change_object.schema.json)
  — the `change_object_record` and `change_object_audit_event_record`
  shapes, plus the closed worktree-lifecycle, change-class,
  authorship, landing-state, recovery-object, rollback/export,
  orphan-cleanup, and conflict-handoff vocabularies.
- [`/schemas/vcs/patch_stack.schema.json`](../../schemas/vcs/patch_stack.schema.json)
  — the `patch_stack_record`, `patch_stack_member_record`,
  `sequence_editor_plan_record`,
  `sequence_editor_operation_record`,
  `patch_stack_audit_event_record`, and
  `sequence_editor_audit_event_record` shapes, plus the closed
  patch-stack-member landing-state, validation-staleness,
  sequence-editor operation-class, and sequence-editor lifecycle
  vocabularies.

Worked cases (a local solo change with a pre-mutation recovery object;
a multi-member patch stack with the base identity pinned and validation
staleness tracked per member; an AI branch-agent change attributable to
a run record; an imported patch-bundle stack pinned to its bundle
envelope; a sequence-editor reword / squash / drop plan that paused
mid-rebase on a conflict and rolled back to the pre-mutation recovery
object; a side-branch export for collaborator review; an orphan worktree
queued for cleanup; a denial when a topology-changing flow tried to
mutate without first naming a recovery object; and a denial when a
downstream surface tried to silently reorder a patch-stack member) live
under
[`/fixtures/vcs/change_stack_cases/`](../../fixtures/vcs/change_stack_cases/).

The eventual change-stack crate's Rust types are the schema of record.
This document and the JSON Schema exports are the cross-tool boundary
every non-owning surface reads. The merge / conflict-class contract and
the dedicated history-edit recovery contract are forward dependencies:
when they land, this contract MUST be the upstream they cite for
change-object, patch-stack, and sequence-editor identity, and
`merge_conflict_class_record_id_ref` and
`history_edit_recovery_record_id_ref` slots on every record become
required non-null on rows that admitted a conflict or history-edit
recovery escalation. Until those contracts land, the slots are reserved
and nullable. If this document and a later merge / conflict-class or
history-edit recovery contract disagree, those contracts win for
conflict and history-edit recovery semantics and this document MUST be
updated in the same change.

Companion artifacts:

- [`/schemas/vcs/review_workspace.schema.json`](../../schemas/vcs/review_workspace.schema.json)
  and
  [`/docs/vcs/review_workspace_contract.md`](review_workspace_contract.md)
  — the review-workspace, comment-anchor, and merge-queue contract a
  change object's downstream review surface reads. A
  `change_object_record` whose landing state is
  `submitted_review_pending` or `submitted_review_in_progress` resolves
  through one `review_workspace_record`; this contract never mints a
  parallel review-state vocabulary.
- [`/schemas/recovery/local_history_entry.schema.json`](../../schemas/recovery/local_history_entry.schema.json)
  and
  [`/docs/reliability/local_history_contract.md`](../reliability/local_history_contract.md)
  — the local-history recovery timeline a `recovery_object_class` of
  `pre_mutation_local_history_checkpoint_minted` resolves into. A
  rollback to a recovery object reads one local-history entry; the
  change-object contract never duplicates the local-history payload.
- [`/schemas/workspace/source_locator.schema.json`](../../schemas/workspace/source_locator.schema.json)
  and
  [`/schemas/workspace/checkout_plan.schema.json`](../../schemas/workspace/checkout_plan.schema.json)
  — the opaque locators a `local_locator` block on a change object
  cites. Raw absolute paths and raw branch / commit URLs never appear
  on a change-object record.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  — the approval-ticket model every mutation-class change-object
  rollback / export action cites. Side-branch export, AI branch-agent
  output export, and patch-stack import each resolve to an approval
  ticket plus a command id.
- [`/schemas/automation/run_record.schema.json`](../../schemas/automation/run_record.schema.json)
  and
  [`/docs/automation/recipe_and_macro_contract.md`](../automation/recipe_and_macro_contract.md)
  — the run-record contract every AI branch-agent or recipe-driven
  change resolves through. A change object whose authorship class is
  `ai_branch_agent_authored` cites a run-record ref; this contract
  never duplicates the run lineage.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — re-exported `freshness_class`, `client_scope`, and
  `redaction_class` vocabularies (ADR-0011). The change-stack
  contract never redefines them.
- [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json)
  — re-exported `workspace_trust_state_class` and policy / trust
  envelope (ADR-0001 / ADR-0018). A topology-changing flow never
  appears available under an unset trust decision; the schema enforces
  this through the `workspace_trust_unset_or_restricted` denial.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — change-object,
  worktree-lifecycle, patch-stack, and sequence-editor architecture.
- `.t2/docs/Aureline_PRD.md` — change-object, patch-stack, and
  history-edit MUST/SHOULD language for explicit recovery objects,
  no-hidden-cross-worktree-writes, attributable AI branch-agent
  outputs, and deterministic rollback / export.

If this contract disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same change.

## Why freeze this now

Until this contract lands, every surface that touches a change object
or a patch stack would be free to invent its own change-state
vocabulary:

- A side-branch agent could write into another worktree to "stage" a
  change, leaving the user with no record of which worktree was the
  authoritative source of the change before the agent ran. The
  reviewer would discover the cross-worktree write only through a
  later merge conflict.
- A patch-stack panel could silently reorder members on a refactor or
  retarget a stack onto a new base without recording the prior order
  or the prior base. A subsequent rollback would leave behind a stack
  whose validation history could not be reconciled with any reachable
  base.
- A sequence-editor plan could begin a rewrite with no pre-mutation
  recovery object minted, leaving the user no path back to the
  pre-rewrite state if the plan paused on a conflict. The reviewer
  would learn the recovery option does not exist only after the
  conflict.
- An AI branch-agent run could mutate a worktree with no run-record
  attribution, leaving support and audit unable to answer "who
  authored this change" beyond a generic "automation" badge.
- An imported patch bundle could land with no source class or bundle
  envelope, so a downstream re-export could not distinguish bundle
  provenance from local truth.
- An orphan worktree could be reaped silently on cleanup, deleting
  in-progress work without an export-before-delete option or a
  tombstone the user could review.

Freezing one record family
(`change_object_record`, `patch_stack_record`,
`patch_stack_member_record`, `sequence_editor_plan_record`,
`sequence_editor_operation_record`) and the closed worktree-lifecycle,
change-class, authorship, landing-state, recovery-object, sequence-op,
validation-staleness, rollback/export, orphan-cleanup, and
conflict-handoff vocabularies they read solves all six problems in one
shape.

## Scope

Frozen at this revision:

1. The `change_object_record` shape every surface reads to identify one
   change, including:
   - one `change_object_class` from the closed five-value vocabulary
     (`commit_authored_locally`, `commit_imported_from_remote`,
     `change_authored_by_ai_branch_agent`,
     `change_imported_from_patch_bundle`,
     `change_synthesized_during_history_edit`);
   - one `change_object_authorship_class` from the closed five-value
     vocabulary (`human_user_authored`, `pair_session_co_authored`,
     `ai_branch_agent_authored`, `imported_from_external_patch_bundle`,
     `synthesized_by_history_edit_operation`);
   - one `change_object_landing_state` from the closed eight-value
     vocabulary (`unsubmitted_local_only`,
     `submitted_review_pending`, `submitted_review_in_progress`,
     `accepted_landed_canonical`,
     `accepted_landed_via_squash_or_fixup`,
     `dropped_explicit_user_action`, `superseded_by_rewrite`,
     `abandoned_orphan_pending_cleanup`);
   - the local locator (`workspace_id_ref` plus
     `branch_or_worktree_ref` plus the bound `worktree_lifecycle_state`
     from the closed seven-value vocabulary
     `worktree_pristine`, `worktree_active_editing`,
     `worktree_in_sequence_edit`, `worktree_paused_pending_conflict`,
     `worktree_archived_orphan_pending_cleanup`,
     `worktree_exported_for_handoff`, `worktree_torn_down`);
   - the `base_identity_ref` (the opaque base / parent identity the
     change is rooted on; immutable for the change's life) and the
     `prior_change_object_id_refs` chain (predecessors when the change
     was rewritten or rebased);
   - the `ai_branch_agent_run_ref` (required when authorship is
     `ai_branch_agent_authored`; null otherwise) and the
     `imported_patch_bundle_ref` (required when the change-object
     class is `change_imported_from_patch_bundle`; null otherwise);
   - the `recovery_object_ref` and its
     `recovery_object_class` from the closed five-value vocabulary
     (`pre_mutation_reflog_anchor_minted`,
     `pre_mutation_local_history_checkpoint_minted`,
     `pre_mutation_patch_stack_snapshot_minted`,
     `pre_mutation_branch_label_anchor_minted`,
     `pre_mutation_combined_recovery_packet_minted`);
   - the `conflict_handoff_class` from the closed five-value vocabulary
     (`no_conflict_path_required`,
     `conflict_resolution_packet_required_pending_decision`,
     `conflict_resolution_packet_admitted_user_resolved`,
     `history_edit_recovery_packet_required_pending_decision`,
     `history_edit_recovery_packet_admitted_user_resolved`);
   - the optional `worktree_orphan_cleanup_class` from the closed
     four-value vocabulary
     (`cleanup_archive_with_tombstone_only_no_reuse`,
     `cleanup_export_then_archive`,
     `cleanup_explicit_user_discard_acknowledged`,
     `cleanup_blocked_pending_export_for_handoff`);
   - the `policy_context` (epoch + workspace trust state) and the
     `redaction_class` the change publishes under;
   - the `client_scopes` the change is admitted on;
   - and the audit-event row shape on the `change_object` audit stream.

2. The `patch_stack_record` shape every ordered stack of changes reads,
   including:
   - the bound `workspace_id_ref` and `base_branch_or_worktree_ref`;
   - the `base_identity_ref` (the immutable base identity for the
     stack's life; a stack that retargets onto a new base resolves to
     a fresh `patch_stack_record` with a recovery object pinning the
     prior stack);
   - the `patch_stack_lifecycle_state` from the closed seven-value
     vocabulary (`patch_stack_drafting`,
     `patch_stack_active_review_open`,
     `patch_stack_paused_pending_conflict`,
     `patch_stack_paused_pending_history_edit_recovery`,
     `patch_stack_landed_completed`,
     `patch_stack_abandoned_rolled_back`,
     `patch_stack_archived_tombstone`);
   - the `ordered_member_id_refs` array (opaque refs to
     `patch_stack_member_record` rows; ordering is the stack's
     landing order; reordering is a fresh stack version with a
     recovery object pinning the prior order);
   - the optional `exported_patch_stack_snapshot_ref` (required when
     the stack was exported for handoff) and
     `imported_from_patch_stack_snapshot_ref` (required when the stack
     was imported from a snapshot);
   - the `recovery_object_ref` pinned at stack mint and bumped on
     every topology-changing operation (reorder, retarget, drop,
     squash, history-edit replay);
   - and the audit-event row shape on the `patch_stack` audit stream.

3. The `patch_stack_member_record` shape every stack member reads,
   including:
   - the `patch_stack_id_ref`;
   - the `ordinal` (zero-based; preserves landing order; mutating the
     ordinal is a fresh member row whose `superseded_by_member_id_ref`
     cites the prior row);
   - the `change_object_id_ref`;
   - the `patch_stack_member_landing_state` from the closed eight-value
     vocabulary (`member_unsubmitted_local_only`,
     `member_submitted_review_pending`,
     `member_submitted_review_in_progress`,
     `member_accepted_landed_canonical`,
     `member_dropped_explicit_user_action`,
     `member_superseded_by_member_rewrite`,
     `member_blocked_pending_conflict_resolution`,
     `member_blocked_pending_history_edit_recovery`);
   - the `validation_staleness_class` from the closed five-value
     vocabulary (`validation_fresh_against_current_base`,
     `validation_stale_base_advanced`,
     `validation_stale_member_rewritten`,
     `validation_unverified_no_run`,
     `validation_blocked_provider_unreachable`);
   - the `recovery_object_ref` (required for any member row that will
     be mutated through the sequence-editor plan: reorder, drop,
     squash, fixup, retarget) and the `conflict_handoff_class`;
   - and the `superseded_by_member_id_ref` slot for rewrite chains.

4. The `sequence_editor_plan_record` shape every history-edit plan
   reads, including:
   - the `target_workspace_id_ref`, `target_branch_or_worktree_ref`,
     and `target_base_identity_ref`;
   - the optional `target_patch_stack_id_ref` (required when the
     sequence edit operates on a known patch stack; null when the
     edit operates on a single branch with no stack overlay);
   - the `recovery_object_ref` (REQUIRED before the plan moves out of
     `sequence_plan_drafted`; a plan that admits an operation without
     citing a recovery object denies with
     `sequence_editor_recovery_object_required_before_admission`);
   - the `sequence_editor_lifecycle_state` from the closed eight-value
     vocabulary (`sequence_plan_drafted`,
     `sequence_plan_admitted`,
     `sequence_plan_in_progress`,
     `sequence_plan_paused_pending_conflict`,
     `sequence_plan_paused_pending_user_resolution`,
     `sequence_plan_completed_recovery_object_minted`,
     `sequence_plan_aborted_rolled_back_to_recovery_object`,
     `sequence_plan_archived_tombstone`);
   - the `ordered_operation_id_refs` array (opaque refs to
     `sequence_editor_operation_record` rows);
   - the `actor_ref`, `command_id_ref`, and `approval_ticket_ref`
     attribution for every plan that admits a mutation-class
     operation; and
   - the audit-event row shape on the `sequence_editor` audit stream.

5. The `sequence_editor_operation_record` shape every history-edit
   operation reads, including:
   - the `sequence_editor_plan_id_ref`;
   - the `ordinal` (zero-based; preserves the plan's operation order);
   - the `sequence_editor_operation_class` from the closed
     eleven-value vocabulary (`sequence_op_pick`, `sequence_op_reword`,
     `sequence_op_edit`, `sequence_op_squash`, `sequence_op_fixup`,
     `sequence_op_drop`, `sequence_op_exec`, `sequence_op_break`,
     `sequence_op_label`, `sequence_op_reset`,
     `sequence_op_merge_replay`);
   - the `target_change_object_id_ref` (required for `pick`, `reword`,
     `edit`, `squash`, `fixup`, `drop`; null for `exec`, `break`,
     `label`, `reset`, `merge_replay` where no single change object is
     pinned);
   - the optional `output_change_object_id_ref` (required when the
     operation has been admitted and produced a synthesized change,
     e.g. a `squash` whose output is a new change with class
     `change_synthesized_during_history_edit`);
   - the `sequence_op_status` from the closed six-value vocabulary
     (`op_admitted`, `op_in_progress`, `op_blocked_pending_conflict`,
     `op_completed`, `op_aborted_rolled_back`, `op_superseded`);
   - and the `conflict_handoff_class` so a paused operation that
     escalates to a conflict-resolution packet or a history-edit
     recovery packet can resume cleanly.

6. The acceptance invariants this contract enforces:

   - Every topology-changing flow names a recovery object before
     mutation. A `sequence_editor_plan_record` that transitions out of
     `sequence_plan_drafted` MUST cite a non-null `recovery_object_ref`;
     a missing ref denies with
     `sequence_editor_recovery_object_required_before_admission`. A
     `change_object_record` whose `change_object_class` is
     `change_synthesized_during_history_edit` MUST cite a
     `recovery_object_ref` whose class is
     `pre_mutation_reflog_anchor_minted`,
     `pre_mutation_local_history_checkpoint_minted`,
     `pre_mutation_patch_stack_snapshot_minted`, or
     `pre_mutation_combined_recovery_packet_minted`. A
     `patch_stack_record` whose lifecycle is
     `patch_stack_active_review_open`, `patch_stack_paused_*`, or
     `patch_stack_landed_completed` MUST cite a non-null
     `recovery_object_ref`.
   - Patch-stack identity preserves base identity, landing order, and
     validation staleness. A surface that retargets a stack onto a new
     base resolves to a fresh `patch_stack_record` with a recovery
     object pinning the prior stack; in-place mutation of
     `base_identity_ref` denies with
     `patch_stack_base_identity_must_not_mutate_in_place`. A surface
     that reorders members denies with
     `patch_stack_member_silent_reorder_forbidden` unless the
     reordered ordinals are paired with `superseded_by_member_id_ref`
     citations recording the prior ordering.
   - No contract path admits hidden cross-worktree writes. A change
     object whose worktree_lifecycle_state is `worktree_active_editing`
     or `worktree_in_sequence_edit` MUST cite a single
     `branch_or_worktree_ref` and any mutation that touches a
     different worktree resolves to a fresh change-object row pinning
     the new worktree (the schema's allOf gate forbids a row that
     names two locators); a violation denies with
     `cross_worktree_write_forbidden`.
   - AI branch-agent outputs are attributable. A change object whose
     authorship class is `ai_branch_agent_authored` MUST cite a
     non-null `ai_branch_agent_run_ref`; a missing ref denies with
     `ai_branch_agent_run_ref_required_for_ai_authored_change`.
     Imported patch bundles MUST cite a non-null
     `imported_patch_bundle_ref`; a missing ref denies with
     `imported_patch_bundle_ref_required_for_imported_change`.
   - Rollback / export is deterministic. A
     `rollback_to_pre_mutation_recovery_object` action MUST cite the
     `recovery_object_ref` it is rolling back to; the export classes
     each pin the snapshot they emit. A surface that emits an export
     without resolving a recovery object or a snapshot denies with
     `rollback_or_export_recovery_object_required`.
   - Orphan worktree cleanup is reviewable. A change object whose
     landing state is `abandoned_orphan_pending_cleanup` MUST cite a
     non-null `worktree_orphan_cleanup_class`; the cleanup class
     resolves to one of the four named postures and the
     `cleanup_export_then_archive` posture MUST cite a non-null
     `exported_patch_stack_snapshot_ref` (or an equivalent export
     packet) before the orphan is reaped. Silent reaping denies with
     `worktree_orphan_silent_reap_forbidden`.
   - Sequence-editor plans escalate cleanly into the conflict and
     history-edit recovery contracts. A plan paused on a conflict
     resolves `conflict_handoff_class` to
     `conflict_resolution_packet_required_pending_decision`; once the
     user resolves the conflict, the plan flips to
     `conflict_resolution_packet_admitted_user_resolved`. A plan
     paused on a history-edit recovery escalates through
     `history_edit_recovery_packet_*` analogues. Until the merge /
     conflict-class contract and the dedicated history-edit recovery
     contract land, the `merge_conflict_class_record_id_ref` and
     `history_edit_recovery_record_id_ref` slots on every record are
     reserved and nullable.

Out of scope until a superseding decision row opens:

- Implementing Git plumbing (real refs, real pack files, real merge
  algorithms, real rebase plumbing). The contract reserves the row
  shape; the engine is a later lane.
- Implementing a real sequence editor or a real history-rewrite tool.
  The contract reserves the plan / operation rows the eventual editor
  binds to.
- Implementing AI branch-agent workflows. The contract reserves the
  `ai_branch_agent_run_ref` slot; the run shape lives in
  `schemas/automation/run_record.schema.json`.
- Building the merge / conflict-class contract or the dedicated
  history-edit recovery contract. They are forward dependencies
  (slots reserved).
- Cross-repo, multi-base, or distributed patch-stack federation. Out
  of scope at this revision.

## 1. The change-object record

Every surface that names a change in Aureline (the diff explorer,
the AI branch-agent panel, the patch-stack panel, the
sequence-editor plan view, the import / export wizard, the support
bundle, the audit lane) MUST resolve the change it is operating on
to exactly one `change_object_record`. The record is the answer to
seven questions a reviewer must be able to answer without opening
any other object:

1. *What kind of change is this?* — `change_object_class`.
2. *Who authored it?* — `change_object_authorship_class` plus the
   `ai_branch_agent_run_ref` or `imported_patch_bundle_ref` envelope.
3. *Where does it live, and what worktree is it bound to right now?* —
   the local locator plus `worktree_lifecycle_state`.
4. *What base is it rooted on, and what predecessors did it
   supersede?* — `base_identity_ref` plus
   `prior_change_object_id_refs`.
5. *What is its landing state?* — `change_object_landing_state`.
6. *What recovery object did the row mint before the most recent
   topology-changing mutation?* — `recovery_object_ref` plus
   `recovery_object_class`.
7. *Is the change clean of conflict or escalating into the conflict /
   history-edit recovery contracts?* — `conflict_handoff_class`.

### 1.1 Change-object class

`change_object_class` is closed and exhaustive:

- `commit_authored_locally` — a commit / change authored on the user's
  own clone, not synthesized through a history-rewrite, not authored
  by an AI agent, not imported from a bundle. Authorship resolves to
  `human_user_authored` or `pair_session_co_authored`.
- `commit_imported_from_remote` — a commit fetched from a remote
  authority. Authorship is whichever class the import chain reports.
- `change_authored_by_ai_branch_agent` — a change authored by an AI
  branch-agent run. Authorship resolves to `ai_branch_agent_authored`
  and `ai_branch_agent_run_ref` MUST be non-null.
- `change_imported_from_patch_bundle` — a change hydrated from an
  imported patch bundle. Authorship resolves to
  `imported_from_external_patch_bundle` and
  `imported_patch_bundle_ref` MUST be non-null.
- `change_synthesized_during_history_edit` — a change synthesized by a
  sequence-editor operation (the output of a `squash`, the rewritten
  output of a `reword` or `edit`, etc.). Authorship resolves to
  `synthesized_by_history_edit_operation` and the row MUST cite a
  `recovery_object_ref` so a rollback to the pre-rewrite state is
  reachable.

### 1.2 Worktree lifecycle states

`worktree_lifecycle_state` is closed and the same vocabulary every
worktree-aware surface reads:

- `worktree_pristine` — clean working tree; no in-flight edits.
- `worktree_active_editing` — the user is editing inside this
  worktree; mutations resolve through this row.
- `worktree_in_sequence_edit` — the worktree is held by a
  sequence-editor plan mid-rebase. Cross-worktree writes are
  forbidden during this state; the schema's allOf gate denies any
  change-object mutation that names a different worktree with
  `cross_worktree_write_forbidden`.
- `worktree_paused_pending_conflict` — the worktree paused on a
  conflict; `conflict_handoff_class` MUST be one of the
  `conflict_resolution_packet_*` values.
- `worktree_archived_orphan_pending_cleanup` — the worktree is no
  longer reachable from the user's main flow but in-progress work has
  not been reviewed. `worktree_orphan_cleanup_class` MUST be non-null.
- `worktree_exported_for_handoff` — the worktree was exported as a
  patch-stack snapshot or a side-branch export packet. The export
  ref is the only authority the row carries until import.
- `worktree_torn_down` — the worktree is gone. The change-object
  record remains as a tombstone for audit / restore.

### 1.3 Authorship

`change_object_authorship_class` is closed:

- `human_user_authored` — the named user authored it. `actor_ref`
  resolves to a single human actor.
- `pair_session_co_authored` — authored inside a collaboration session
  with co-authors recorded by reference. `actor_ref` resolves to the
  primary author; co-authors travel through the collaboration session
  authority record.
- `ai_branch_agent_authored` — authored by an AI branch-agent run.
  `ai_branch_agent_run_ref` MUST be non-null.
- `imported_from_external_patch_bundle` — hydrated from an external
  patch bundle. `imported_patch_bundle_ref` MUST be non-null.
- `synthesized_by_history_edit_operation` — synthesized by a
  sequence-editor operation. The synthesizing operation lives in the
  patch-stack schema's `sequence_editor_operation_record`.

### 1.4 Landing state

`change_object_landing_state` is closed and the row a downstream
review surface reads to know whether the change is open, in review,
landed, dropped, superseded, or abandoned:

- `unsubmitted_local_only` — local-only; no review surface is open.
- `submitted_review_pending` — a review workspace was opened against
  this change but the provider overlay has not been fetched.
- `submitted_review_in_progress` — review is active; cite the
  `review_workspace_record`.
- `accepted_landed_canonical` — landed on the canonical base.
- `accepted_landed_via_squash_or_fixup` — landed as part of a squash
  or fixup; the synthesized output's predecessor chain cites this
  row through `prior_change_object_id_refs`.
- `dropped_explicit_user_action` — the user explicitly dropped the
  change; the rollback / export path cites the recovery object.
- `superseded_by_rewrite` — a sequence-editor operation rewrote this
  change; cite the successor through the patch-stack member's
  `superseded_by_member_id_ref`.
- `abandoned_orphan_pending_cleanup` — the change was left behind
  when a worktree was reaped; `worktree_orphan_cleanup_class` MUST
  be non-null.

### 1.5 Recovery objects

`recovery_object_class` is closed. Every topology-changing flow
mints exactly one recovery object and pins its ref before the
mutation begins:

- `pre_mutation_reflog_anchor_minted` — a reflog-style branch anchor
  pinning the pre-mutation HEAD / branch tip.
- `pre_mutation_local_history_checkpoint_minted` — a local-history
  checkpoint (see
  `schemas/recovery/local_history_entry.schema.json`).
- `pre_mutation_patch_stack_snapshot_minted` — a patch-stack snapshot
  pinning the pre-mutation ordered members.
- `pre_mutation_branch_label_anchor_minted` — a branch label anchor
  pinning a named recovery point under the workspace's local
  retention.
- `pre_mutation_combined_recovery_packet_minted` — a combined packet
  citing two or more of the above; used when a topology change spans
  multiple recovery surfaces (e.g. a sequence-editor plan that
  rewrites a patch stack).

The acceptance invariant: a row that claims a topology-changing
mutation without citing a recovery object denies with
`change_object_recovery_object_required_before_topology_change` (on
change objects) or
`sequence_editor_recovery_object_required_before_admission` (on
sequence-editor plans) or
`patch_stack_recovery_object_required_before_topology_change` (on
patch stacks).

### 1.6 Conflict handoff

`conflict_handoff_class` is closed. It is the cross-link from a
change object, a patch-stack member, or a sequence-editor operation
into the merge / conflict-class contract and the dedicated
history-edit recovery contract:

- `no_conflict_path_required` — the row has not escalated.
- `conflict_resolution_packet_required_pending_decision` — the row
  paused on a conflict; the user has not resolved yet. The merge /
  conflict-class contract's row will be cited through
  `merge_conflict_class_record_id_ref` once that contract lands.
- `conflict_resolution_packet_admitted_user_resolved` — the user
  resolved; the row resumes.
- `history_edit_recovery_packet_required_pending_decision` — a
  history-edit operation paused on a recovery escalation; the
  dedicated history-edit recovery contract's row will be cited
  through `history_edit_recovery_record_id_ref` once that contract
  lands.
- `history_edit_recovery_packet_admitted_user_resolved` — the user
  resolved the recovery escalation; the row resumes.

Until the merge / conflict-class contract and the dedicated
history-edit recovery contract land, `merge_conflict_class_record_id_ref`
and `history_edit_recovery_record_id_ref` are reserved and nullable.
The reserved slots survive the next contract's landing without a
breaking change: today they are nullable, later they become required
non-null on rows whose `conflict_handoff_class` is one of the
`*_required_pending_decision` or `*_admitted_user_resolved` values.

### 1.7 Orphan cleanup

`worktree_orphan_cleanup_class` is closed and is required (non-null)
exactly when a change object's landing state is
`abandoned_orphan_pending_cleanup` or its worktree lifecycle state is
`worktree_archived_orphan_pending_cleanup`:

- `cleanup_archive_with_tombstone_only_no_reuse` — the worktree is
  archived as a tombstone; no further reuse is admissible.
- `cleanup_export_then_archive` — the worktree is exported (via a
  patch-stack snapshot or a side-branch export packet) and then
  archived. The export ref MUST be non-null before the archive flips.
- `cleanup_explicit_user_discard_acknowledged` — the user explicitly
  discarded the in-progress work; the discard event lives on the
  audit stream.
- `cleanup_blocked_pending_export_for_handoff` — the cleanup is
  blocked pending a handoff export; the row remains in the
  `worktree_archived_orphan_pending_cleanup` state until the export
  completes.

Silent reaping denies with `worktree_orphan_silent_reap_forbidden`.

## 2. The patch-stack record and members

A patch stack is an ordered, base-pinned, validation-aware sequence of
change objects that the user is shaping for landing. Every surface
that names a stack (the patch-stack panel, the AI branch-agent
review surface, the sequence-editor plan view, support / export)
MUST resolve the stack to exactly one `patch_stack_record` and resolve
each member to exactly one `patch_stack_member_record`.

### 2.1 Stack identity and base

The stack's `base_identity_ref` is immutable for the stack's life. A
surface that retargets the stack onto a new base does not mutate the
existing record; it mints a fresh `patch_stack_record` whose
`recovery_object_ref` cites the prior stack's snapshot. A row that
mutates `base_identity_ref` in place denies with
`patch_stack_base_identity_must_not_mutate_in_place`.

### 2.2 Member ordering and rewrite chains

`ordered_member_id_refs` is the stack's landing order. Mutating this
order is a fresh stack version with a recovery object pinning the
prior order; the member rows are not destructively re-ordinaled in
place. The schema's allOf gate denies a duplicate ordinal across two
non-superseded member rows in the same stack with
`patch_stack_member_ordinal_uniqueness_required`. A surface that
silently reorders without minting a successor row denies with
`patch_stack_member_silent_reorder_forbidden`.

### 2.3 Validation staleness

`validation_staleness_class` is closed:

- `validation_fresh_against_current_base` — the member's validation
  run was executed against the stack's current `base_identity_ref`
  and matches.
- `validation_stale_base_advanced` — the base advanced since the run.
- `validation_stale_member_rewritten` — the member was rewritten by a
  sequence-editor operation since the run; the run does not cover the
  current member.
- `validation_unverified_no_run` — no validation run has been
  executed; the row is honest about the absence.
- `validation_blocked_provider_unreachable` — the validation provider
  was unreachable on the last attempt.

### 2.4 Member landing states

`patch_stack_member_landing_state` is closed and pairs through allOf
gates with the member's `change_object_landing_state`:

- `member_unsubmitted_local_only`,
  `member_submitted_review_pending`,
  `member_submitted_review_in_progress`,
  `member_accepted_landed_canonical`,
  `member_dropped_explicit_user_action`,
  `member_superseded_by_member_rewrite`,
  `member_blocked_pending_conflict_resolution`,
  `member_blocked_pending_history_edit_recovery`.

A member whose landing state is `member_blocked_pending_conflict_resolution`
MUST cite `conflict_handoff_class = conflict_resolution_packet_required_pending_decision`;
a member whose landing state is `member_blocked_pending_history_edit_recovery`
MUST cite `conflict_handoff_class = history_edit_recovery_packet_required_pending_decision`.

## 3. The sequence-editor plan and operations

A sequence-editor plan is the explicit row that names a history-edit
intent (rebase, reword, squash, fixup, drop, exec, break, label,
reset, merge replay) before any mutation begins. Every history-edit
surface MUST resolve the plan it is operating on to exactly one
`sequence_editor_plan_record` and resolve each operation to exactly
one `sequence_editor_operation_record`.

### 3.1 Plan admission requires a recovery object

A plan whose lifecycle state moves out of `sequence_plan_drafted`
MUST cite a non-null `recovery_object_ref`. A plan that admits an
operation without citing a recovery object denies with
`sequence_editor_recovery_object_required_before_admission`. The
plan's `actor_ref`, `command_id_ref`, and `approval_ticket_ref` MUST
be non-null on every admitted plan; a missing ref denies with
`sequence_editor_attribution_missing`.

### 3.2 Operation classes

`sequence_editor_operation_class` is closed and exhaustive (eleven
values). Per-operation invariants:

| operation              | `target_change_object_id_ref` | `output_change_object_id_ref` |
|------------------------|-------------------------------|-------------------------------|
| `sequence_op_pick`     | required                      | optional (matches target)     |
| `sequence_op_reword`   | required                      | required when admitted        |
| `sequence_op_edit`     | required                      | required when admitted        |
| `sequence_op_squash`   | required                      | required when admitted        |
| `sequence_op_fixup`    | required                      | required when admitted        |
| `sequence_op_drop`     | required                      | null                          |
| `sequence_op_exec`     | null                          | null                          |
| `sequence_op_break`    | null                          | null                          |
| `sequence_op_label`    | null                          | null                          |
| `sequence_op_reset`    | null                          | null                          |
| `sequence_op_merge_replay` | required                  | required when admitted        |

A row that mismatches the table denies with
`sequence_editor_operation_target_or_output_mismatch`.

### 3.3 Pause / resume escalation

A plan whose lifecycle is `sequence_plan_paused_pending_conflict`
MUST cite at least one operation whose `sequence_op_status` is
`op_blocked_pending_conflict` and whose `conflict_handoff_class` is
`conflict_resolution_packet_required_pending_decision`. A plan whose
lifecycle is `sequence_plan_paused_pending_user_resolution` MUST
cite at least one operation with the matching pause-class
escalation. Resuming the plan flips the operation row's status and
the plan's lifecycle in lockstep; silent resumption (a plan that
flips out of pause without resolving the escalation) denies with
`sequence_editor_pause_resolution_required_before_resume`.

### 3.4 Abort path

A plan that aborts MUST resolve its lifecycle to
`sequence_plan_aborted_rolled_back_to_recovery_object` and cite the
recovery object the abort rolled back to. A row that claims an
abort without resolving to the recovery object denies with
`sequence_editor_abort_recovery_object_required`.

## 4. Rollback / export rules

`rollback_or_export_class` is closed and the row a downstream
surface reads to know whether a rollback or an export is admissible:

- `rollback_to_pre_mutation_recovery_object` — admissible only when
  `recovery_object_ref` is non-null. A rollback that names no
  recovery object denies with
  `rollback_or_export_recovery_object_required`.
- `export_patch_stack_snapshot_for_handoff` — admissible only when
  `exported_patch_stack_snapshot_ref` (on the stack record) or an
  equivalent snapshot envelope is non-null.
- `export_ai_branch_agent_run_for_review` — admissible only when
  `ai_branch_agent_run_ref` is non-null. The export does not embed
  raw run logs; it cites the run record.
- `export_side_branch_for_collaborator_review` — admissible only when
  the row is bound to a single worktree and the export packet was
  minted under an approval ticket.
- `export_for_support_bundle` — admissible only under
  `internal_support_restricted` redaction class.
- `import_patch_stack_snapshot_from_handoff` — admissible only when
  `imported_from_patch_stack_snapshot_ref` is non-null and the
  import was minted under an approval ticket.

The rollback / export class is recorded on the audit stream; the
contract never embeds raw bundle bytes, raw run logs, or raw
provider URLs.

## 5. Audit streams

Three audit streams are reserved by this contract:

- `change_object_audit_event` — closed event-id vocabulary including
  `change_object_minted`, `change_object_authorship_resolved`,
  `change_object_landing_state_changed`,
  `change_object_recovery_object_pinned`,
  `change_object_rolled_back_to_recovery_object`,
  `change_object_exported_for_handoff`,
  `change_object_imported_from_bundle`,
  `change_object_archived`,
  `change_object_audit_denial_emitted`. Denial events MUST cite one
  denial reason from the `change_object_denial_reason` vocabulary.
- `patch_stack_audit_event` — closed event-id vocabulary including
  `patch_stack_drafted`, `patch_stack_member_appended`,
  `patch_stack_member_superseded`,
  `patch_stack_validation_run_recorded`,
  `patch_stack_paused_pending_conflict`,
  `patch_stack_paused_pending_history_edit_recovery`,
  `patch_stack_landed_completed`,
  `patch_stack_abandoned_rolled_back`,
  `patch_stack_exported_for_handoff`,
  `patch_stack_imported_from_snapshot`,
  `patch_stack_archived`,
  `patch_stack_audit_denial_emitted`. Denial events MUST cite one
  denial reason from the `patch_stack_denial_reason` vocabulary.
- `sequence_editor_audit_event` — closed event-id vocabulary
  including `sequence_editor_plan_drafted`,
  `sequence_editor_plan_admitted`,
  `sequence_editor_operation_admitted`,
  `sequence_editor_operation_in_progress`,
  `sequence_editor_operation_blocked_pending_conflict`,
  `sequence_editor_operation_completed`,
  `sequence_editor_plan_completed`,
  `sequence_editor_plan_paused`,
  `sequence_editor_plan_resumed`,
  `sequence_editor_plan_aborted_rolled_back`,
  `sequence_editor_audit_denial_emitted`. Denial events MUST cite one
  denial reason from the `sequence_editor_denial_reason` vocabulary.

Adding a new denial reason or a new audit-event id is additive-minor
and bumps the per-record schema-version const; repurposing an
existing value is breaking and requires a new decision row.

## 6. Redaction posture

Every record published against this contract carries one
`redaction_class` from the re-exported capability-lifecycle
vocabulary. Defaults:

- Local-only change objects, patch-stack rows, and sequence-editor
  plans default to `metadata_safe_default`.
- Change objects whose authorship class is `ai_branch_agent_authored`
  MAY raise to `internal_support_restricted` when the run touched
  organisationally restricted state.
- Patch-stack export packets and side-branch export packets MAY
  raise to `internal_support_restricted` for support exports and
  `operator_only_restricted` when the stack touched a credentialed
  flow.
- Sequence-editor operations whose target change touched a
  credentialed flow MUST raise to `operator_only_restricted`.

Raw absolute paths, raw branch / commit URLs, raw author identity
strings, raw commit message bodies, raw patch bodies, raw run-log
bodies, raw approval-ticket bodies, raw notebook cell text, raw
terminal bytes, and raw URLs never appear on any record published
against this contract. Every payload travels by opaque ref or
through the redaction-aware label registry.

## 7. Acceptance cross-walk

| Acceptance bullet from the plan | Where it lands |
|---|---|
| Every topology-changing flow names a recovery object before mutation. | §1.5 recovery-object class + §3.1 plan admission gate + the `change_object_recovery_object_required_before_topology_change`, `sequence_editor_recovery_object_required_before_admission`, and `patch_stack_recovery_object_required_before_topology_change` denials. Fixtures `change_object_local_solo_with_recovery.yaml`, `sequence_editor_plan_admitted_with_recovery.yaml`, and `sequence_editor_plan_admission_without_recovery_denied.yaml`. |
| Patch-stack fixtures preserve base identity, landing order, validation staleness, and rollback / export determinism. | §2.1 base-identity allOf gate + §2.2 ordering gate + §2.3 validation-staleness vocabulary + §4 rollback / export class + the `patch_stack_base_identity_must_not_mutate_in_place`, `patch_stack_member_silent_reorder_forbidden`, and `rollback_or_export_recovery_object_required` denials. Fixtures `patch_stack_multi_member_with_validation_staleness.yaml` and `patch_stack_member_silent_reorder_denied.yaml`. |
| No contract path allows hidden cross-worktree writes or silent reorder / retarget of stack members. | §1.2 worktree-lifecycle gate + §2.2 ordering gate + the `cross_worktree_write_forbidden`, `patch_stack_base_identity_must_not_mutate_in_place`, and `patch_stack_member_silent_reorder_forbidden` denials. Fixtures `patch_stack_member_silent_reorder_denied.yaml` and `change_object_orphan_worktree_pending_cleanup.yaml`. |
| Reviewer can follow a side-branch or history-edit scenario from change object → sequence state → conflict / recovery packet without hidden state. | §1.6 conflict-handoff class + §3.3 pause / resume gate + the `sequence_editor_pause_resolution_required_before_resume` denial + the reserved `merge_conflict_class_record_id_ref` and `history_edit_recovery_record_id_ref` slots. Fixtures `sequence_editor_plan_paused_pending_conflict.yaml`, `change_object_ai_branch_agent_authored.yaml`, and `patch_stack_imported_from_bundle.yaml`. |

## 8. Versioning

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
