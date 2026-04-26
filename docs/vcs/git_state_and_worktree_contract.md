# Git local-vs-provider truth, branch / worktree / history / stash contract

This document freezes the cross-tool record family every Aureline
surface reads when it presents local Git state — branches and their
remote-tracking refs, worktrees (the main repository checkout plus any
linked / sparse worktrees), in-progress history operations (merge,
rebase, cherry-pick, revert, mailbox-apply, bisect), and stashes —
before SCM UX is free to blur local truth, provider review state,
local-history / editor recovery, and worktree ownership into a single
misleading timeline. Every branch surface, worktree surface, and
in-progress-operation banner reads exactly one record from this family
and never invents its own "git state" vocabulary. Provider review
overlays (the row class shipped in
[`/docs/vcs/review_workspace_contract.md`](review_workspace_contract.md))
appear here only as a visually secondary chip; this contract pins
local Git as the authoritative truth and the provider PR / MR overlay
as the secondary cue.

The machine-readable boundaries are:

- [`/schemas/vcs/branch_row.schema.json`](../../schemas/vcs/branch_row.schema.json)
  — the `branch_row_record`, `stash_row_record`, and
  `branch_row_audit_event_record` shapes plus the closed
  branch-class, unpublished-commit-risk, remote-tracking-freshness,
  workset-cleanliness, recovery-action, open-action, provider-overlay
  visibility, and stash-class vocabularies.
- [`/schemas/vcs/worktree_row.schema.json`](../../schemas/vcs/worktree_row.schema.json)
  — the `worktree_row_record` and `worktree_row_audit_event_record`
  shapes plus the closed worktree-kind, worktree-registry-lifecycle,
  worktree-removal, and worktree-removal-banner-class vocabularies.
- [`/schemas/vcs/history_operation_state.schema.json`](../../schemas/vcs/history_operation_state.schema.json)
  — the `history_operation_state_record` and
  `history_operation_state_audit_event_record` shapes plus the closed
  history-operation-kind, history-operation-state, in-progress-banner,
  and review-sheet vocabularies for the branch-switch / worktree-removal
  / merge / rebase / cherry-pick / revert / mailbox-apply / bisect paths.

Worked cases (a local-only branch with unpublished commits and a
recovery prompt before switch; a local branch with a fresh remote
overlay and a provider PR chip layered visually secondary; a
detached-head state with no branch label; a linked worktree pinned to
a feature branch; a sparse worktree with workset-limited cleanliness
explicitly labeled; a worktree pending removal with the metadata-only
versus filesystem-deletion review sheet shown; an in-progress rebase
paused on a conflict with the resume / abort review sheet; an
in-progress bisect with the next-step banner; a stash row pinning a
base ref so apply is reachable; and a denial when a downstream surface
tried to remove a worktree under a single generic "remove" action) live
under
[`/fixtures/vcs/git_state_cases/`](../../fixtures/vcs/git_state_cases/).

The eventual git-service crate's Rust types are the schema of record.
This document and the JSON Schema exports are the cross-tool boundary
every non-owning surface reads. The merge / conflict-class contract
and the dedicated history-edit recovery contract are forward
dependencies: when they land, this contract MUST be the upstream they
cite for branch / worktree / in-progress-operation identity, and
`merge_conflict_class_record_id_ref` and
`history_edit_recovery_record_id_ref` slots on every record become
required non-null on rows whose conflict-handoff class is one of the
escalation values. Until those contracts land, the slots are reserved
and nullable.

Companion artifacts:

- [`/schemas/vcs/change_object.schema.json`](../../schemas/vcs/change_object.schema.json)
  and
  [`/docs/vcs/change_stack_contract.md`](change_stack_contract.md)
  — the change-object, worktree-lifecycle, patch-stack, and
  sequence-editor truth a branch row's bound change cites. A
  `branch_row_record` whose head pins one or more local-only changes
  resolves through `change_object_record` ids; this contract never
  duplicates the change-stack vocabulary. The change-stack contract's
  `worktree_lifecycle_state` is the *per-change* posture; the worktree
  registry row defined here is the *worktree* identity.
- [`/schemas/vcs/review_workspace.schema.json`](../../schemas/vcs/review_workspace.schema.json)
  and
  [`/docs/vcs/review_workspace_contract.md`](review_workspace_contract.md)
  — the provider-review overlay a branch row may layer on visually
  as a secondary chip. The review-workspace contract is authoritative
  for review-state vocabulary; this contract carries only an opaque
  ref to a `review_workspace_record` and an explicit
  `provider_overlay_visual_secondary_class` so the chip never claims
  to be the local Git truth.
- [`/schemas/recovery/local_history_entry.schema.json`](../../schemas/recovery/local_history_entry.schema.json)
  — the local-history checkpoint a `branch_recovery_action_class` of
  `recovery_local_history_checkpoint_minted_before_switch` resolves
  into. This contract never duplicates the local-history payload.
- [`/schemas/workspace/source_locator.schema.json`](../../schemas/workspace/source_locator.schema.json)
  and
  [`/schemas/workspace/checkout_plan.schema.json`](../../schemas/workspace/checkout_plan.schema.json)
  — the opaque locators a `branch_label_ref`,
  `linked_worktree_id_ref`, and `worktree_path_ref` cite. Raw absolute
  paths and raw branch / commit / remote URLs never appear on a
  branch / worktree / in-progress-operation record.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  — the approval-ticket model every mutation-class action
  (worktree removal, branch retarget, in-progress-operation abort)
  cites. A mutation never appears available without resolving to
  an approval ticket plus a command id.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — re-exported `freshness_class`, `client_scope`, and
  `redaction_class` vocabularies (ADR-0011). This contract never
  redefines them.
- [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json)
  — re-exported `workspace_trust_state_class` and policy / trust
  envelope (ADR-0001 / ADR-0018). A topology-changing flow (branch
  switch with unpublished commits, worktree removal, in-progress
  operation abort) never appears available under an unset trust
  decision.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — git service
  architecture, worktree-aware status, sparse / workset composition
  (Appendix BJ), and the git / code-host / review / CI provider matrix
  (Appendix BK).
- `.t2/docs/Aureline_PRD.md` — "the worktree is sacred" rule, side-
  branch / worktree separation, rebase / cherry-pick workflow MUST /
  SHOULD language, and worktree-and-branch-awareness rules for
  partial-truth surfaces.

If this contract disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same change.

## Why freeze this now

Until this contract lands, every surface that touches a branch, a
worktree, an in-progress operation, or a stash would be free to
invent its own state vocabulary:

- A "branch picker" could blur the local branch label, the
  remote-tracking ref, and the provider PR overlay into a single
  string. The user would press "switch" and discover unpublished
  commits only after the switch had already moved HEAD.
- A "worktree remove" button could pretend that metadata-only removal
  (prune the admin entry under `.git/worktrees/`), filesystem
  deletion (also `rm -rf` the working directory), and path-preserve
  removal (keep the working directory intact, prune the admin entry
  only) are one action. The user would lose work to a single
  destructive default with no review sheet.
- An "in-progress merge / rebase / cherry-pick" status banner could
  silently disappear when the surface refreshed, leaving the
  reviewer no path to "resume", "abort to recovery", or "view review
  sheet".
- A "sparse worktree" surface could render "clean" when the working
  directory matches its sparse cone but unstaged changes outside the
  cone exist. The reviewer would see a green chip with no honest
  workset-cleanliness label.
- A "stash" surface could lose the base ref the stash was minted
  against. Apply would later succeed silently against a drifted base
  with no warning.
- A "provider PR" chip could occupy the same visual weight as the
  local branch label. A reviewer working through a provider outage
  would no longer be able to tell whether the row in front of them
  was authoritative local Git truth or a stale overlay.

Freezing one record family
(`branch_row_record`, `stash_row_record`, `worktree_row_record`,
`history_operation_state_record`) and the closed branch-class,
unpublished-commit-risk, worktree-kind, worktree-removal-class,
history-operation-kind / state, recovery-action, open-action, and
provider-overlay visibility vocabularies they read solves all six
problems in one shape.

## Scope

Frozen at this revision:

1. The `branch_row_record` shape every branch-aware surface reads,
   including:
   - one `branch_class` from the closed eight-value vocabulary
     (`local_branch_no_remote_tracking`,
     `local_branch_tracking_remote_in_sync`,
     `local_branch_tracking_remote_ahead`,
     `local_branch_tracking_remote_behind`,
     `local_branch_tracking_remote_diverged`,
     `local_branch_tracking_remote_gone_upstream_deleted`,
     `detached_head_state_no_branch_label`,
     `imported_or_synthetic_branch_label`);
   - one `unpublished_commit_risk_class` from the closed five-value
     vocabulary (`no_unpublished_commits_clean`,
     `unpublished_commits_present_local_only`,
     `unpublished_commits_present_diverged_with_remote`,
     `unpublished_commits_unknown_remote_unreachable`,
     `unpublished_commits_not_applicable_no_tracking`);
   - the optional `remote_tracking_envelope` (the tracking ref id,
     last-known remote revision id, ahead / behind counts, the
     `remote_tracking_freshness_class` from the closed four-value
     vocabulary `remote_tracking_fresh`,
     `remote_tracking_stale_within_grace`,
     `remote_tracking_stale_beyond_grace_local_continues`,
     `remote_tracking_unavailable_local_continues`, and the
     last-fetched timestamp). Required (non-null) on every
     `branch_class` other than `local_branch_no_remote_tracking`,
     `detached_head_state_no_branch_label`, and
     `imported_or_synthetic_branch_label`;
   - the `linked_worktree_id_ref` (the opaque ref to the
     `worktree_row_record` the branch is currently checked out in;
     null when no worktree owns the branch);
   - the `head_revision_id_ref` (opaque) and the
     `change_object_id_refs` array (opaque change-object ids on
     `local_branch_tracking_remote_ahead` /
     `local_branch_tracking_remote_diverged` rows so the unpublished
     commits are reviewable through the change-stack contract);
   - the `workset_cleanliness_class` from the closed five-value
     vocabulary
     (`full_worktree_cleanliness`,
     `sparse_worktree_workset_limited_cleanliness`,
     `cone_mode_workset_limited_cleanliness`,
     `partial_clone_promisor_limited_cleanliness`,
     `unknown_workset_cleanliness_unverified`) so a "clean" chip on a
     sparse / workset-limited surface is honest about scope;
   - the optional `provider_overlay_visual_secondary` block
     (`provider_review_workspace_id_ref` plus
     `provider_overlay_visual_secondary_class` from the closed
     three-value vocabulary
     `overlay_visual_secondary_local_truth_authoritative`,
     `overlay_visual_secondary_provider_authoritative_for_provider_owned_rows`,
     `overlay_unavailable_local_truth_only_no_chip`). The chip never
     replaces the local branch label; the chip is rendered as a
     secondary cue only;
   - the `branch_recovery_action_class` from the closed seven-value
     vocabulary
     (`recovery_no_action_required_no_unpublished_commits`,
     `recovery_local_history_checkpoint_minted_before_switch`,
     `recovery_branch_label_anchor_minted_before_switch`,
     `recovery_export_unpublished_to_patch_stack_snapshot_before_switch`,
     `recovery_export_to_side_branch_for_collaborator_review_before_switch`,
     `recovery_combined_packet_minted_before_switch`,
     `recovery_blocked_pending_user_review`);
   - the `branch_open_action_class` from the closed six-value
     vocabulary
     (`open_action_check_out_branch_in_main_worktree`,
     `open_action_check_out_branch_in_linked_worktree`,
     `open_action_attach_provider_review_overlay_view_only`,
     `open_action_view_only_no_mutation_admissible`,
     `open_action_blocked_pending_in_progress_operation_resolution`,
     `open_action_blocked_pending_workspace_trust_decision`);
   - the `policy_context` (epoch + workspace trust state) and the
     `redaction_class` the row publishes under;
   - the `client_scopes` the row is admitted on; and
   - the audit-event row shape on the `branch_row` audit stream.

2. The `stash_row_record` shape every stash-aware surface reads,
   including:
   - the `stash_class` from the closed four-value vocabulary
     (`stash_authored_locally`,
     `stash_imported_from_handoff`,
     `stash_synthesized_during_branch_switch_recovery`,
     `stash_archived_tombstone`);
   - the `base_branch_or_worktree_ref` and `base_revision_id_ref`
     (both required and non-null; the schema's allOf gate denies a
     stash row that omits its base ref so apply against a drifted
     base is mechanically forbidden);
   - the `linked_worktree_id_ref` (the worktree the stash was minted
     in);
   - the `stash_lifecycle_state` from the closed four-value
     vocabulary
     (`stash_active_apply_admissible`,
     `stash_applied_consumed`,
     `stash_dropped_explicit_user_action`,
     `stash_archived_tombstone`); and
   - the audit-event row shape on the `branch_row` audit stream.

3. The `worktree_row_record` shape every worktree-aware surface reads,
   including:
   - one `worktree_kind_class` from the closed four-value vocabulary
     (`worktree_main_repository_checkout`,
     `worktree_linked_secondary_checkout`,
     `worktree_imported_handoff_snapshot_view_only`,
     `worktree_synthetic_recovery_view_only`);
   - the `worktree_path_ref` (an opaque locator into the workspace
     family; raw absolute paths never appear);
   - the `bound_branch_id_ref` (the branch the worktree currently
     checks out; null when the worktree's branch class is
     `detached_head_state_no_branch_label`);
   - the `head_revision_id_ref` (opaque);
   - the `worktree_workset_cleanliness_class` (the same closed
     five-value vocabulary as on the branch row);
   - the `worktree_registry_lifecycle_state` from the closed
     seven-value vocabulary
     (`worktree_registered_active_clean`,
     `worktree_registered_active_with_in_progress_operation`,
     `worktree_registered_paused_pending_conflict`,
     `worktree_pending_removal_user_review_required`,
     `worktree_removed_metadata_only_admin_directory_pruned`,
     `worktree_removed_filesystem_deleted_admin_directory_pruned`,
     `worktree_archived_tombstone`);
   - the optional `worktree_removal_class` from the closed five-value
     vocabulary
     (`worktree_removal_metadata_only_admin_directory_pruned`,
     `worktree_removal_filesystem_deletion_admin_directory_pruned`,
     `worktree_removal_path_preserve_admin_directory_pruned_only`,
     `worktree_removal_blocked_pending_export_for_handoff`,
     `worktree_removal_blocked_pending_in_progress_operation_resolution`)
     paired through allOf gates with the lifecycle row so a single
     generic "remove" action cannot hide the difference;
   - the `worktree_removal_review_sheet_class` from the closed
     four-value vocabulary
     (`review_sheet_not_applicable_no_removal_pending`,
     `review_sheet_metadata_only_vs_filesystem_deletion_distinction_required`,
     `review_sheet_path_preserve_explicit_user_acknowledgement_required`,
     `review_sheet_blocked_pending_in_progress_operation_resolution`);
   - the optional `in_progress_history_operation_state_id_ref`
     (opaque ref to a `history_operation_state_record`; required
     non-null when the lifecycle is
     `worktree_registered_active_with_in_progress_operation` or
     `worktree_registered_paused_pending_conflict`);
   - the `policy_context`, `redaction_class`, `client_scopes`,
     `actor_ref`, timestamps, and the audit-event row shape on the
     `worktree_row` audit stream.

4. The `history_operation_state_record` shape every in-progress-
   operation banner reads, including:
   - one `history_operation_kind_class` from the closed nine-value
     vocabulary
     (`no_operation_clean`,
     `merge_in_progress`,
     `rebase_interactive_in_progress`,
     `rebase_noninteractive_in_progress`,
     `cherry_pick_in_progress`,
     `revert_in_progress`,
     `apply_mailbox_in_progress`,
     `bisect_in_progress`,
     `apply_or_am_imported_patch_in_progress`);
   - one `history_operation_state_class` from the closed six-value
     vocabulary
     (`not_applicable_clean`,
     `paused_pending_conflict_resolution`,
     `paused_pending_user_continuation`,
     `running_no_user_action_required`,
     `completed_recovery_object_minted`,
     `aborted_rolled_back_to_recovery_object`);
   - the `target_worktree_id_ref` (the worktree the operation owns;
     required non-null when the kind is not `no_operation_clean`);
   - the optional `target_branch_id_ref` (the branch the operation
     runs on; null when bisect runs on a detached head);
   - the `recovery_object_ref` (REQUIRED when the kind is anything
     other than `no_operation_clean`; a row whose state is
     `paused_*`, `completed_*`, or `aborted_*` MUST cite a non-null
     ref so the row is rollback-reachable);
   - the `conflict_handoff_class` re-exported from the change-stack
     contract (used to escalate into the merge / conflict-class
     contract once that lands);
   - the `in_progress_banner_class` from the closed six-value
     vocabulary
     (`banner_no_banner_required_clean`,
     `banner_in_progress_operation_resume_or_abort`,
     `banner_branch_switch_blocked_pending_in_progress_operation`,
     `banner_worktree_removal_blocked_pending_in_progress_operation`,
     `banner_bisect_next_step_required`,
     `banner_apply_imported_patch_continue_or_skip`);
   - the `review_sheet_class` from the closed five-value vocabulary
     (`review_sheet_no_sheet_required_clean`,
     `review_sheet_in_progress_operation_resume_or_abort`,
     `review_sheet_in_progress_operation_abort_to_recovery_object_only`,
     `review_sheet_branch_switch_with_unpublished_commits_recovery_required`,
     `review_sheet_worktree_removal_metadata_vs_filesystem_distinction_required`);
   - the `actor_ref`, `command_id_ref`, and `approval_ticket_ref`
     attribution for every state past `not_applicable_clean`; and
   - the audit-event row shape on the `history_operation_state`
     audit stream.

5. The acceptance invariants this contract enforces:

   - Local Git truth is authoritative; provider review overlays are
     visually secondary. A `branch_row_record` whose
     `provider_overlay_visual_secondary_class` is
     `overlay_visual_secondary_local_truth_authoritative` MUST resolve
     the local branch label as the primary cue; a surface that renders
     a provider chip as the primary label denies with
     `provider_overlay_must_not_replace_local_branch_label`. A
     `branch_row_record` whose overlay block is omitted MUST NOT
     advertise a provider chip; a surface that renders one anyway
     denies with `provider_overlay_chip_required_overlay_record`.
   - Reviewers can answer where a change lives, who owns the working
     directory, whether there are unpublished commits, and how to
     recover before switching / removing / rebasing. A row whose
     `unpublished_commit_risk_class` is
     `unpublished_commits_present_local_only` or
     `unpublished_commits_present_diverged_with_remote` MUST cite a
     `branch_recovery_action_class` that is not
     `recovery_no_action_required_no_unpublished_commits`; a violation
     denies with `branch_recovery_action_required_for_unpublished_commits`.
   - Worktree removal cannot hide metadata-only removal, filesystem
     deletion, and path-preserve behaviour behind one generic action.
     A `worktree_row_record` whose lifecycle moves into
     `worktree_pending_removal_user_review_required` MUST cite a
     `worktree_removal_class` and a non-`review_sheet_not_applicable_no_removal_pending`
     `worktree_removal_review_sheet_class`. A row whose lifecycle is
     `worktree_removed_metadata_only_admin_directory_pruned` MUST cite
     `worktree_removal_class = worktree_removal_metadata_only_admin_directory_pruned`;
     `worktree_removed_filesystem_deleted_admin_directory_pruned` MUST cite
     `worktree_removal_class = worktree_removal_filesystem_deletion_admin_directory_pruned`.
     A surface that lands a removed lifecycle without naming the
     matching removal class denies with
     `worktree_removal_class_required_for_removed_lifecycle`. A
     surface that emits a single generic "remove" action without
     resolving to one of the three explicit removal classes denies
     with `worktree_removal_generic_action_forbidden`.
   - Sparse / workset cleanliness is explicitly labelled. A
     `branch_row_record` or `worktree_row_record` whose
     `workset_cleanliness_class` is anything other than
     `full_worktree_cleanliness` MUST surface the workset-limited cue
     on every freshness chip; a surface that renders "clean" without
     the matching workset-limited disclosure denies with
     `workset_cleanliness_must_match_chip_disclosure`.
   - In-progress operations escalate cleanly into the conflict and
     history-edit recovery contracts. A
     `history_operation_state_record` whose state is
     `paused_pending_conflict_resolution` MUST cite
     `conflict_handoff_class = conflict_resolution_packet_required_pending_decision`;
     a state of `paused_pending_user_continuation` MUST pair with
     `conflict_handoff_class = conflict_resolution_packet_admitted_user_resolved`
     (when the user has resolved the conflict and the operation
     awaits an explicit "continue") or
     `history_edit_recovery_packet_required_pending_decision` (when
     the operation paused on a recovery escalation). The reserved
     `merge_conflict_class_record_id_ref` and
     `history_edit_recovery_record_id_ref` slots are nullable today
     and become required non-null when the merge / conflict-class
     contract and the dedicated history-edit recovery contract land.
   - Branch switch with unpublished commits is reviewable. A
     `branch_open_action_class` of
     `open_action_check_out_branch_in_main_worktree` paired with a
     source row whose `unpublished_commit_risk_class` is
     `unpublished_commits_present_local_only` or
     `unpublished_commits_present_diverged_with_remote` MUST resolve
     a `branch_recovery_action_class` other than
     `recovery_no_action_required_no_unpublished_commits` *and* a
     `review_sheet_class` of
     `review_sheet_branch_switch_with_unpublished_commits_recovery_required`.
     A surface that admits the switch silently denies with
     `branch_switch_recovery_review_sheet_required_before_switch`.
   - In-progress operations block branch switch and worktree
     removal. A `branch_row_record` whose
     `branch_open_action_class` is
     `open_action_blocked_pending_in_progress_operation_resolution`
     MUST cite the in-progress operation row through the bound
     worktree's `in_progress_history_operation_state_id_ref`; a
     missing ref denies with
     `in_progress_operation_block_must_cite_state_record`.
   - Stash apply against a drifted base is forbidden silently. A
     `stash_row_record` MUST cite a non-null
     `base_branch_or_worktree_ref` *and* `base_revision_id_ref`; a
     row that omits either denies with
     `stash_base_ref_required_for_apply_admission`.

Out of scope until a superseding decision row opens:

- Implementing Git plumbing (real refs, real pack files, real merge
  algorithms, real rebase / cherry-pick plumbing, real bisect engine,
  real stash apply). The contract reserves the row shape; the engine
  is a later lane.
- Building the merge / conflict-class contract or the dedicated
  history-edit recovery contract. They are forward dependencies
  (slots reserved).
- Implementing Git history visualisation (commit graph, branch
  topology rendering). The contract reserves the data the
  visualisation binds to.
- Cross-repo branch / worktree federation, multi-host remote
  aggregation, or queue federation. Out of scope at this revision.

## 1. The branch row record

Every branch-aware surface in Aureline (the branch picker, the
change-stack strip, the source-control panel, the AI branch-agent
panel, the support / export bundle's git section, the audit lane)
MUST resolve the branch it is operating on to exactly one
`branch_row_record`. The record is the answer to seven questions a
reviewer must be able to answer without opening any other object:

1. *What kind of branch is this?* — `branch_class`.
2. *Are there unpublished commits, and what is the risk class?* —
   `unpublished_commit_risk_class` plus the optional
   `change_object_id_refs` chain into the change-stack contract.
3. *Where is its remote-tracking ref, and how fresh?* —
   `remote_tracking_envelope` plus
   `remote_tracking_freshness_class`.
4. *Which worktree owns the branch right now?* —
   `linked_worktree_id_ref`.
5. *Is the cleanliness chip honest about workset / sparse scope?* —
   `workset_cleanliness_class`.
6. *Is the provider PR / MR overlay layered on, and what authority
   does it claim?* — `provider_overlay_visual_secondary` block plus
   `provider_overlay_visual_secondary_class`.
7. *What recovery and open actions are admissible right now?* —
   `branch_recovery_action_class` plus `branch_open_action_class`.

### 1.1 Branch class

`branch_class` is closed and exhaustive:

- `local_branch_no_remote_tracking` — purely local; no tracking ref
  exists. `remote_tracking_envelope` MUST be null.
- `local_branch_tracking_remote_in_sync` — local matches the
  tracking ref's last-known revision. `remote_tracking_envelope`
  MUST be non-null.
- `local_branch_tracking_remote_ahead` — local has commits the
  tracking ref does not. `unpublished_commit_risk_class` MUST be
  `unpublished_commits_present_local_only`.
- `local_branch_tracking_remote_behind` — tracking ref has commits
  local does not.
- `local_branch_tracking_remote_diverged` — both have commits the
  other lacks. `unpublished_commit_risk_class` MUST be
  `unpublished_commits_present_diverged_with_remote`.
- `local_branch_tracking_remote_gone_upstream_deleted` — tracking
  ref existed historically but is now gone upstream.
- `detached_head_state_no_branch_label` — the worktree points at a
  commit with no branch label. `linked_worktree_id_ref` MUST be
  non-null when the detached head lives in a worktree.
- `imported_or_synthetic_branch_label` — branch label minted by an
  import / handoff / recovery flow, not authored locally.

### 1.2 Unpublished commit risk class

`unpublished_commit_risk_class` is closed:

- `no_unpublished_commits_clean` — nothing local that is not on the
  tracking ref.
- `unpublished_commits_present_local_only` — local has commits the
  tracking ref does not; the user MUST resolve the
  `branch_recovery_action_class` before any topology-changing flow
  (branch switch, worktree remove, rebase) admits.
- `unpublished_commits_present_diverged_with_remote` — both sides
  have unpublished work; `branch_recovery_action_class` MUST mint a
  recovery object before any topology-changing flow.
- `unpublished_commits_unknown_remote_unreachable` — the remote is
  unreachable and the count cannot be determined; honestly admit
  the unknown rather than render `clean`.
- `unpublished_commits_not_applicable_no_tracking` — the branch has
  no tracking ref so the question does not apply.

### 1.3 Remote-tracking freshness

`remote_tracking_freshness_class` is closed and named so the chip
the reviewer reads is mechanical:

- `remote_tracking_fresh` — last fetched within the workspace's
  grace window.
- `remote_tracking_stale_within_grace` — past ideal freshness but
  inside the grace window; chip surfaces "stale" without blocking.
- `remote_tracking_stale_beyond_grace_local_continues` — past the
  grace window; remote-tracking cues fall back to "last known".
- `remote_tracking_unavailable_local_continues` — remote was
  unreachable on the last attempt; local Git continues to be the
  authoritative truth.

The freshness on the *branch's tracking ref* and the freshness on
the *provider review overlay* are independent. A row may have a
fresh tracking ref but a stale or unavailable provider overlay (or
vice versa). The `provider_overlay_visual_secondary_class` covers
the overlay; this freshness class covers the tracking ref only.

### 1.4 Workset cleanliness

`workset_cleanliness_class` is closed:

- `full_worktree_cleanliness` — the working directory reflects the
  whole repository; "clean" / "dirty" chips are global.
- `sparse_worktree_workset_limited_cleanliness` — the worktree is
  under a sparse-checkout workset; chip MUST advertise that
  cleanliness is *workset-limited*.
- `cone_mode_workset_limited_cleanliness` — sparse-checkout cone
  mode limits visibility; chip MUST advertise *cone-limited*.
- `partial_clone_promisor_limited_cleanliness` — partial clone
  through a promisor remote; chip MUST advertise *partial-clone-limited*.
- `unknown_workset_cleanliness_unverified` — workset boundary
  cannot be verified on this row; chip MUST advertise *unverified*.

A row whose workset class is anything other than
`full_worktree_cleanliness` and that surfaces "clean" without the
matching disclosure denies with
`workset_cleanliness_must_match_chip_disclosure`.

### 1.5 Provider overlay visibility

`provider_overlay_visual_secondary_class` is closed:

- `overlay_visual_secondary_local_truth_authoritative` — the chip
  is shown as a secondary cue; local branch is authoritative for
  the row.
- `overlay_visual_secondary_provider_authoritative_for_provider_owned_rows`
  — the overlay is fresh and the provider is authoritative for
  *provider-owned* rows (approval, mergeability, queue) but the
  branch-row primary label remains the local branch.
- `overlay_unavailable_local_truth_only_no_chip` — the overlay
  block is omitted entirely and no chip is rendered.

The chip never replaces the local branch label. A surface that
renders a provider PR / MR title or status as the primary cue
denies with `provider_overlay_must_not_replace_local_branch_label`.

### 1.6 Recovery and open actions

`branch_recovery_action_class` and `branch_open_action_class` are
both closed and read mechanically by the action surface (the
"switch" / "checkout" / "open in worktree" / "open review overlay"
buttons). A row whose
`unpublished_commit_risk_class` is one of the
`unpublished_commits_present_*` values MUST resolve the recovery
class to one of the non-`recovery_no_action_required_no_unpublished_commits`
values; a switch that does not pair with a recovery object denies
with `branch_recovery_action_required_for_unpublished_commits`.

A row whose bound worktree carries an in-progress operation MUST
resolve the open class to
`open_action_blocked_pending_in_progress_operation_resolution`.

## 2. The stash row record

Every stash-aware surface MUST resolve the stash to exactly one
`stash_row_record`. The record is the answer to four questions:

1. *What class of stash is this?* — `stash_class`.
2. *Which worktree minted it?* — `linked_worktree_id_ref`.
3. *What base is it pinned against?* — `base_branch_or_worktree_ref`
   plus `base_revision_id_ref` (both required and non-null).
4. *Is apply still admissible?* — `stash_lifecycle_state`.

A stash row that omits its base ref denies with
`stash_base_ref_required_for_apply_admission`. The row never
embeds the stashed patch body; the body is a forward reference
into the patch-stack snapshot family or the local-history entry
the stash was minted into.

## 3. The worktree row record

Every worktree-aware surface MUST resolve to exactly one
`worktree_row_record`. The record is the answer to six questions:

1. *What kind of worktree is this?* — `worktree_kind_class`.
2. *Where does it live?* — `worktree_path_ref`.
3. *Which branch is it bound to?* — `bound_branch_id_ref` plus
   `head_revision_id_ref`.
4. *Is its workset cleanliness chip honest?* —
   `worktree_workset_cleanliness_class`.
5. *Is there an in-progress operation owned by this worktree?* —
   `in_progress_history_operation_state_id_ref` plus the
   `worktree_registry_lifecycle_state`.
6. *Is removal pending, and which removal class applies?* —
   `worktree_removal_class` plus
   `worktree_removal_review_sheet_class`.

### 3.1 Worktree kind

`worktree_kind_class` is closed:

- `worktree_main_repository_checkout` — the primary `.git` directory
  worktree.
- `worktree_linked_secondary_checkout` — `git worktree add` style
  secondary checkout.
- `worktree_imported_handoff_snapshot_view_only` — minted from an
  import / handoff packet; mutations are forbidden through this
  row.
- `worktree_synthetic_recovery_view_only` — minted by a recovery
  flow (e.g. an orphan-cleanup export-then-archive surface);
  mutations are forbidden through this row.

### 3.2 Worktree registry lifecycle

`worktree_registry_lifecycle_state` is closed and ordered:

- `worktree_registered_active_clean` — registered and the worktree
  has no in-progress operation.
- `worktree_registered_active_with_in_progress_operation` — an
  in-progress operation is running; the row MUST cite
  `in_progress_history_operation_state_id_ref`.
- `worktree_registered_paused_pending_conflict` — the worktree's
  in-progress operation paused on a conflict.
- `worktree_pending_removal_user_review_required` — removal is
  proposed but the review sheet has not yet been acknowledged.
- `worktree_removed_metadata_only_admin_directory_pruned` — the
  admin directory was pruned but the working directory was kept on
  disk.
- `worktree_removed_filesystem_deleted_admin_directory_pruned` —
  the admin directory was pruned and the working directory was
  removed from disk.
- `worktree_archived_tombstone` — the worktree row is kept as a
  tombstone for audit / restore.

### 3.3 Worktree removal class and review sheet

`worktree_removal_class` is closed:

- `worktree_removal_metadata_only_admin_directory_pruned` — only
  the admin entry under `.git/worktrees/` is pruned; the working
  directory on disk is preserved (rename, archive, or hand off).
- `worktree_removal_filesystem_deletion_admin_directory_pruned` —
  the admin entry is pruned *and* the working directory is removed
  from disk.
- `worktree_removal_path_preserve_admin_directory_pruned_only` —
  the admin entry is pruned; the working directory is explicitly
  preserved by the user with an acknowledgement event on the
  audit stream.
- `worktree_removal_blocked_pending_export_for_handoff` — removal
  blocked because a handoff export is required first.
- `worktree_removal_blocked_pending_in_progress_operation_resolution`
  — removal blocked because an in-progress operation is running.

`worktree_removal_review_sheet_class` is closed and pairs through
allOf gates with the lifecycle / removal class so a single generic
"remove" action is mechanically forbidden:

- `review_sheet_not_applicable_no_removal_pending`
- `review_sheet_metadata_only_vs_filesystem_deletion_distinction_required`
- `review_sheet_path_preserve_explicit_user_acknowledgement_required`
- `review_sheet_blocked_pending_in_progress_operation_resolution`

A surface that lands a `worktree_pending_removal_user_review_required`
lifecycle without naming the removal class and the matching review
sheet class denies with `worktree_removal_generic_action_forbidden`.
A surface that lands a removed lifecycle whose
`worktree_removal_class` does not match denies with
`worktree_removal_class_required_for_removed_lifecycle`.

## 4. The history-operation-state record

Every in-progress operation banner / review sheet MUST resolve to
exactly one `history_operation_state_record`. The record is the
answer to seven questions:

1. *What kind of operation is in progress?* —
   `history_operation_kind_class`.
2. *What state is it in?* — `history_operation_state_class`.
3. *Which worktree owns the operation?* — `target_worktree_id_ref`.
4. *Which branch is it running on?* — `target_branch_id_ref` (null
   for bisect on detached head).
5. *What recovery object did the operation pin?* —
   `recovery_object_ref`.
6. *How does it escalate into the conflict / history-edit recovery
   contracts?* — `conflict_handoff_class`.
7. *Which banner / review sheet does the surface render right now?*
   — `in_progress_banner_class` plus `review_sheet_class`.

### 4.1 Operation kind

`history_operation_kind_class` is closed and exhaustive:

- `no_operation_clean` — the worktree has no in-progress operation.
  Every other field MUST resolve to `not_applicable` /
  `banner_no_banner_required_clean` /
  `review_sheet_no_sheet_required_clean`.
- `merge_in_progress` — `git merge` paused or running.
- `rebase_interactive_in_progress` — `git rebase -i` (the
  sequence-editor plan path lives in the change-stack contract;
  this row is the *worktree-side* record of the in-progress op).
- `rebase_noninteractive_in_progress` — `git rebase` (no plan).
- `cherry_pick_in_progress` — `git cherry-pick`.
- `revert_in_progress` — `git revert`.
- `apply_mailbox_in_progress` — `git am`.
- `bisect_in_progress` — `git bisect`.
- `apply_or_am_imported_patch_in_progress` — applying an imported
  patch bundle through a non-`am` path.

### 4.2 Operation state

`history_operation_state_class` is closed:

- `not_applicable_clean` — operation is `no_operation_clean`.
- `paused_pending_conflict_resolution` — paused on a conflict; MUST
  pair with
  `conflict_handoff_class = conflict_resolution_packet_required_pending_decision`.
- `paused_pending_user_continuation` — conflict resolved or the
  step finished and the operation awaits an explicit "continue"
  (e.g. `git rebase --continue`); MUST pair with
  `conflict_handoff_class = conflict_resolution_packet_admitted_user_resolved`
  *or* `history_edit_recovery_packet_*` analogues when the
  continuation depends on a history-edit recovery escalation.
- `running_no_user_action_required` — the operation is making
  progress and no user action is required right now (e.g. a
  long-running rebase between conflicts).
- `completed_recovery_object_minted` — the operation completed
  cleanly; the recovery object the operation pinned remains
  reachable for rollback.
- `aborted_rolled_back_to_recovery_object` — the operation was
  aborted and the worktree was rolled back to the recovery object
  the operation pinned at admission.

### 4.3 Banner and review sheet

`in_progress_banner_class` is the banner the in-progress surface
renders; `review_sheet_class` is the review sheet a topology-
changing flow (branch switch, worktree removal) shows when blocked
by the in-progress state. Both are closed and pair through allOf
gates with the kind / state.

The acceptance invariants:

- A row whose state is `paused_pending_conflict_resolution` MUST
  cite `banner_in_progress_operation_resume_or_abort` and
  `review_sheet_in_progress_operation_resume_or_abort`.
- A `bisect_in_progress` row MUST cite
  `banner_bisect_next_step_required` and (if a topology change is
  attempted) `review_sheet_in_progress_operation_resume_or_abort`.
- A row whose state is `aborted_rolled_back_to_recovery_object`
  MUST cite the `recovery_object_ref` it rolled back to; a missing
  ref denies with
  `history_operation_abort_recovery_object_required`.

## 5. Rollback / open / removal admission rules

The schemas enforce, through allOf gates and closed denial-reason
vocabularies, the following rules:

| Rule | Mechanism |
|---|---|
| Branch switch with unpublished commits is blocked until a recovery object is minted. | `branch_recovery_action_required_for_unpublished_commits` denial on `branch_row_record`. |
| Provider PR / MR overlay never replaces the local branch label. | `provider_overlay_must_not_replace_local_branch_label` denial. |
| Provider chip rendered without an overlay record denies. | `provider_overlay_chip_required_overlay_record` denial. |
| Worktree removal cannot use one generic action. | `worktree_removal_generic_action_forbidden` and `worktree_removal_class_required_for_removed_lifecycle` denials. |
| Path-preserve removal requires explicit user acknowledgement. | `worktree_removal_review_sheet_class = review_sheet_path_preserve_explicit_user_acknowledgement_required` paired through allOf with `worktree_removal_path_preserve_admin_directory_pruned_only`. |
| In-progress operation blocks branch switch and worktree removal. | `in_progress_operation_block_must_cite_state_record` denial on `branch_row_record` / `worktree_row_record`. |
| Stash apply against a drifted base is forbidden silently. | `stash_base_ref_required_for_apply_admission` denial. |
| Sparse / workset-limited surfaces label cleanliness honestly. | `workset_cleanliness_must_match_chip_disclosure` denial. |
| In-progress operation abort cites the recovery object. | `history_operation_abort_recovery_object_required` denial. |

## 6. Audit streams

Three audit streams are reserved by this contract:

- `branch_row_audit_event` — closed event-id vocabulary including
  `branch_row_minted`, `branch_row_remote_tracking_fetched`,
  `branch_row_unpublished_risk_recomputed`,
  `branch_row_provider_overlay_attached`,
  `branch_row_recovery_object_pinned_before_switch`,
  `branch_row_switched`, `branch_row_archived`,
  `stash_row_minted`, `stash_row_applied`, `stash_row_dropped`,
  `branch_row_audit_denial_emitted`. Denial events MUST cite one
  reason from the `branch_row_denial_reason` vocabulary.
- `worktree_row_audit_event` — closed event-id vocabulary including
  `worktree_row_registered`,
  `worktree_row_in_progress_operation_started`,
  `worktree_row_paused_pending_conflict`,
  `worktree_row_pending_removal_review_required`,
  `worktree_row_removed_metadata_only`,
  `worktree_row_removed_filesystem_deleted`,
  `worktree_row_path_preserved_acknowledged`,
  `worktree_row_archived`,
  `worktree_row_audit_denial_emitted`. Denial events MUST cite one
  reason from the `worktree_row_denial_reason` vocabulary.
- `history_operation_state_audit_event` — closed event-id
  vocabulary including
  `history_operation_state_admitted`,
  `history_operation_state_paused_pending_conflict`,
  `history_operation_state_paused_pending_user_continuation`,
  `history_operation_state_continued`,
  `history_operation_state_completed`,
  `history_operation_state_aborted_rolled_back`,
  `history_operation_state_audit_denial_emitted`. Denial events MUST
  cite one reason from the
  `history_operation_state_denial_reason` vocabulary.

Adding a new denial reason or a new audit-event id is additive-minor
and bumps the per-record schema-version const; repurposing an
existing value is breaking and requires a new decision row.

## 7. Redaction posture

Every record published against this contract carries one
`redaction_class` from the re-exported capability-lifecycle
vocabulary. Defaults:

- Local branch / worktree / stash / in-progress-operation rows
  default to `metadata_safe_default`.
- Rows whose actor or approval ticket touches a credentialed flow
  (e.g. a worktree removal admitted under a managed-admin surface)
  MUST raise to `operator_only_restricted`.
- Support exports of any row MUST raise to
  `internal_support_restricted`.
- Provider-overlay refs travel through opaque ids only; raw URLs,
  raw branch names, raw remote names, raw author identity strings,
  raw absolute paths, raw commit message bodies, raw patch bodies,
  and raw stash payload bytes never appear on any record published
  against this contract.

## 8. Acceptance cross-walk

| Acceptance bullet from the plan | Where it lands |
|---|---|
| Reviewers can answer where a change lives, which working directory owns it, whether there are unpublished commits, and how to recover before switching / removing / rebasing. | §1.1–§1.6 branch-row record + §3 worktree-row record + §4 history-operation-state record + the `branch_recovery_action_required_for_unpublished_commits` and `branch_switch_recovery_review_sheet_required_before_switch` denials. Fixtures `branch_row_local_only_with_unpublished_commits.yaml`, `branch_row_tracking_remote_with_provider_overlay_layered.yaml`, and `history_operation_state_rebase_paused_pending_conflict.yaml`. |
| Worktree removal cannot hide metadata-only removal, filesystem deletion, and path-preserve behaviour behind one generic action. | §3.3 worktree-removal class + review-sheet class allOf gate + the `worktree_removal_generic_action_forbidden`, `worktree_removal_class_required_for_removed_lifecycle`, and `review_sheet_path_preserve_explicit_user_acknowledgement_required` denials. Fixtures `worktree_row_pending_removal_metadata_vs_filesystem_review_sheet.yaml` and `worktree_row_generic_removal_denied.yaml`. |
| Fixtures cover at least: detached head, linked worktrees, sparse worktree with limited cleanliness, stash with base ref, and provider review chip layered over local branch truth. | Fixtures `branch_row_detached_head_state.yaml`, `worktree_row_linked_secondary_checkout.yaml`, `worktree_row_sparse_workset_limited_cleanliness.yaml`, `stash_row_pinned_to_base_ref.yaml`, and `branch_row_tracking_remote_with_provider_overlay_layered.yaml`. |

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
