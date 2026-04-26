# Hosted-review inbox, provider-authoritative state, and merge-policy contract

This document freezes how Aureline keeps local draft review state, local
parity results, provider-authoritative checks, merge-queue posture, and
offline review continuity mechanically distinguishable before any
hosted-review provider integration lands. Every reviewer-facing surface
(the pending-review inbox, the review-workspace header, the
checks / evaluation strip, the merge-queue entry row, the stack /
dependency strip, the approval / publish sheet) resolves the row it is
rendering to one of three record families:

- `review_inbox_row_record` — one pending-review-inbox row pointing at
  one `review_workspace_record` plus the inbox-class, inbox-state, and
  offline-continuation posture the inbox surface reads.
- `review_workspace_state_record` — the review-workspace header, the
  checks / evaluation strip, the stack / dependency strip, and the
  approval / publish sheet for one workspace, with every strip cell
  honest about whether it is provider-authoritative, a local parity
  estimate, stale relative to base / head, not evaluated on this
  surface, or a draft-only local row.
- `merge_queue_entry_record` — the queue-eligible / queue-blocked /
  queue-landed entry row for one workspace, naming the mergeability
  class, the queue-eligibility class, the closed block-reason
  vocabulary (checks stale, policy blocked, approval invalidated,
  stale base, ...), the local-vs-provider rerun-authority class, and
  the stack / dependency strip the entry inherits.

No surface mints a parallel "ready to merge" badge, a parallel
"reviewing" inbox vocabulary, or a parallel "blocked" cue. The closed
vocabularies in this contract are the only legal answers.

The machine-readable boundaries are:

- [`/schemas/vcs/review_inbox_row.schema.json`](../../schemas/vcs/review_inbox_row.schema.json)
  — the `review_inbox_row_record` and
  `review_inbox_row_audit_event_record` shapes plus the closed
  inbox-class, inbox-state, offline-continuation, freshness-sync, and
  denial-reason vocabularies.
- [`/schemas/vcs/review_workspace_state.schema.json`](../../schemas/vcs/review_workspace_state.schema.json)
  — the `review_workspace_state_record` and
  `review_workspace_state_audit_event_record` shapes plus the closed
  header / checks / approval / stack / publish-sheet strip vocabularies
  including `strip_cell_authority_class`,
  `approval_strip_state_class`, and
  `publish_sheet_publication_state_class`.
- [`/schemas/vcs/merge_queue_entry.schema.json`](../../schemas/vcs/merge_queue_entry.schema.json)
  — the `merge_queue_entry_record` and
  `merge_queue_entry_audit_event_record` shapes plus the closed
  mergeability / queue-eligibility / queue-block-reason /
  rerun-authority vocabularies.

Worked cases (a local-parity-only workspace whose checks were
evaluated on the user's clone with no provider overlay; a provider-
authoritative checks workspace where the provider owns
required-check / approval / mergeability cues; a merge-queue entry
blocked because the workspace's base drifted and the captured
provider rule snapshot went stale; a stacked-review entry that
inherits an upstream workspace's block-reason through the stack /
dependency strip; a provider-outage workspace that continues to admit
local review, draft approvals, and a publish-later / export-only
escape hatch without relabelling cached overlay cues as authoritative)
live under
[`/fixtures/vcs/hosted_review_cases/`](../../fixtures/vcs/hosted_review_cases/).

The eventual hosted-review and merge-policy crates' Rust types are the
schema of record. This document and the JSON Schema exports are the
cross-tool boundary every non-owning surface reads. This contract is
the upstream the
[`docs/vcs/review_workspace_contract.md`](review_workspace_contract.md)
forward-dependency slot
(`hosted_review_inbox_record_id_ref`) and the
`merge_policy_record_id_ref` slot will resolve through. If this
document and a later provider-API or merge-engine contract disagree,
those later contracts win for wire / runtime semantics and this
document MUST be updated in the same change.

Companion artifacts:

- [`/schemas/vcs/review_workspace.schema.json`](../../schemas/vcs/review_workspace.schema.json)
  and
  [`/docs/vcs/review_workspace_contract.md`](review_workspace_contract.md)
  — the upstream `review_workspace_record`,
  `merge_queue_action_record`, and `review_anchor_record` family.
  Every record in this contract cites the workspace and never re-mints
  a `review_workspace_source_class` /
  `provider_overlay_freshness_class` /
  `provider_authority_class` value. The merge-queue entry composes
  with the merge-queue action record (the entry is the row; the action
  is the per-mutation event).
- [`/schemas/vcs/review_pack.schema.json`](../../schemas/vcs/review_pack.schema.json)
  and
  [`/schemas/vcs/review_evaluation_result.schema.json`](../../schemas/vcs/review_evaluation_result.schema.json)
  and
  [`/docs/vcs/review_pack_contract.md`](review_pack_contract.md)
  — the review-pack manifest the checks / evaluation strip evaluates
  against, and the normalized `review_evaluation_result_record` every
  strip cell cites. The strip never re-mints a check outcome, a
  parity-state class, or a divergence label; it cites the result.
- [`/schemas/vcs/patch_stack.schema.json`](../../schemas/vcs/patch_stack.schema.json)
  and
  [`/docs/vcs/change_stack_contract.md`](change_stack_contract.md)
  — the patch-stack and member rows the stack / dependency strip
  cites. Stack ordering, base-pin identity, and the
  `patch_stack_member_landing_state` vocabulary are owned by that
  contract; this row carries the strip view, not a duplicate ordering.
- [`/schemas/vcs/branch_row.schema.json`](../../schemas/vcs/branch_row.schema.json)
  and
  [`/schemas/vcs/worktree_row.schema.json`](../../schemas/vcs/worktree_row.schema.json)
  and
  [`/docs/vcs/git_state_and_worktree_contract.md`](git_state_and_worktree_contract.md)
  — the branch / worktree rows the workspace header and the
  base-drift / head-drift cells resolve through. Raw branch names and
  raw absolute paths never cross this boundary.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  — the approval-ticket model every publish-sheet / approval-strip
  row cites when the action is a mutation-class publish. A draft
  approval that has not been published carries no approval ticket and
  resolves through `approval_strip_state_draft_only_local`.
- [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  — the browser-handoff packet model used when an inbox row resolves
  through a browser-handoff token source. Raw URLs never appear on a
  record published against this contract.
- [`/schemas/work_items/work_item_detail.schema.json`](../../schemas/work_items/work_item_detail.schema.json)
  — the work-item model the inbox row's `linked_work_item_ref` and
  the publish-sheet `linked_work_item_ref` cite when a hosted-review
  row threads back into a work item.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — re-exported `freshness_class`, `client_scope`, and
  `redaction_class` vocabularies (ADR-0011).
- [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json)
  — re-exported `workspace_trust_state_class` and policy / trust
  envelope (ADR-0001 / ADR-0018). A queue entry never appears
  available under an unset trust decision; the
  `queue_entry_blocked_reason` vocabulary names
  `policy_epoch_expired_re_evaluation_required` and
  `workspace_trust_unset_or_restricted` for that path.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — the
  hosted-review inbox, the merge-queue entry, and the
  approval / publish-sheet matrices.
- `.t2/docs/Aureline_PRD.md` — hosted-review MUST/SHOULD language for
  honest provider-authoritative cues, durable offline review
  continuity, and attributable merge-queue entries.

If this contract disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same change.

## Why freeze this now

Until this contract lands, every surface that touches a hosted-review
inbox or a merge-queue entry would be free to invent its own row
vocabulary:

- A pending-review inbox could collapse "I am the author", "review was
  requested from me", "I am following this", "I was @-mentioned", and
  "I have a draft local row that has not been published" into one
  ambiguous "active" state. A reviewer working through an inbox
  triage would have to read tool-specific copy to know which rows are
  on them right now.
- An inbox row whose provider overlay is stale-beyond-grace could
  silently relabel cached cues as authoritative, so the chip the
  reviewer reads would not change when the provider went unreachable.
- A review-workspace header could publish one composite "ready" badge
  conflating the local checks pass, the provider checks pass, the
  approval state, and the merge-queue admission. A reviewer would not
  be able to tell whether the badge meant "local parity passed but
  provider is still pending" or "provider passed but base drifted".
- The checks / evaluation strip could render a missing evaluator
  as green by omission, repeating the green-by-omission failure mode
  the review-pack contract already forbids on the per-result row but
  has no view-level guard against on the strip.
- The approval / publish sheet could render a draft local approval
  identically to a published provider approval, so the reviewer would
  not be able to tell whether their approval had crossed the wire.
  Offline-captured approvals would land at the provider on next sync
  with no visible distinction from a real-time approval.
- A merge-queue entry could appear "queue-eligible" while the
  workspace's base has drifted, the required checks are stale, the
  approval has been invalidated by a force-push, or the policy epoch
  has expired. The reviewer would press "land" and discover the
  block only on the provider's failure path.
- Stacked review dependencies would be expressed in tool-specific copy
  ("this PR depends on PR #123"), with no machine-readable way to
  inherit an upstream block-reason through the stack so the downstream
  entry's blocked reason resolves to "upstream blocked" with a
  citation, rather than re-deriving an independent block-reason.
- A provider outage would be expressed as a single error toast, with
  no honest local-continuation path. The reviewer could not tell
  whether they could still post local comments, draft local
  approvals, queue a publish-later, or export an evidence packet.

Freezing one record family
(`review_inbox_row_record`, `review_workspace_state_record`,
`merge_queue_entry_record`) and the closed inbox-state, strip-cell
authority, approval-strip state, mergeability, queue-eligibility,
queue-block-reason, rerun-authority, and offline-continuation
vocabularies they read solves all eight problems in one shape.

## Scope

Frozen at this revision:

1. The `review_inbox_row_record` shape every pending-review inbox
   surface reads, including:
   - one `review_inbox_row_class` from the closed seven-value
     vocabulary
     (`authored_by_current_actor`,
     `review_requested_from_current_actor`,
     `following_for_visibility_only`,
     `mentioned_in_thread`,
     `assigned_for_landing`,
     `draft_only_local_not_yet_published`,
     `imported_bundle_inbox_row`);
   - one `review_inbox_state_class` from the closed five-value
     vocabulary
     (`provider_authoritative`,
     `local_parity_estimate`,
     `stale_relative_to_base_or_head`,
     `not_evaluated_on_this_surface`,
     `draft_only_no_provider_state`)
     so the reviewer can read the cue mechanically;
   - one `offline_continuation_class` from the closed five-value
     vocabulary
     (`online_synced_no_offline_state`,
     `offline_capture_pending_publish_later`,
     `offline_export_only_no_publish_later`,
     `provider_outage_local_continues_publish_blocked`,
     `provider_outage_local_continues_publish_queued`)
     so a provider outage path is mechanically distinguishable from a
     deliberate offline draft;
   - one `inbox_freshness_sync_class` from the closed four-value
     vocabulary
     (`inbox_row_fresh`,
     `inbox_row_stale_within_grace`,
     `inbox_row_stale_beyond_grace_local_continues`,
     `inbox_row_unverifiable_provider_unreachable`)
     so the inbox chip is mechanical;
   - the closed `review_inbox_lifecycle_state` vocabulary
     (`open_pending_action`, `acknowledged_no_action_required`,
     `dismissed_by_actor`, `archived_tombstone`);
   - reserved refs for `linked_work_item_ref`,
     `linked_review_pack_id_ref`, `linked_review_evaluation_result_id_ref`,
     `merge_queue_entry_id_ref`, and `merge_policy_record_id_ref`; and
   - the audit-event row shape on the `review_inbox_row` audit stream.

2. The `review_workspace_state_record` shape every review-workspace
   surface reads, composed of four named strips and one publish sheet:
   - the **header strip** carrying the bound
     `review_workspace_id_ref`, the head / base ref pair (each one a
     `branch_or_worktree_ref` into the git-state contract), the
     workspace's `head_drift_class` and `base_drift_class` from the
     closed four-value vocabulary
     (`fresh_no_drift`,
     `drifted_within_grace`,
     `drifted_beyond_grace_re_evaluate`,
     `drift_unverifiable_provider_unreachable`),
     and the workspace's authoring `policy_context`;
   - the **checks / evaluation strip** as a list of
     `checks_strip_cell` rows, each one citing one
     `review_evaluation_result_id_ref` plus one
     `strip_cell_authority_class` from the closed five-value
     vocabulary
     (`provider_authoritative`,
     `local_parity_estimate`,
     `stale_relative_to_base_or_head`,
     `not_evaluated_on_this_surface`,
     `draft_only_no_provider_state`)
     so a missing evaluator is forced to publish a typed
     `not_evaluated_on_this_surface` cell rather than disappear into
     a green-by-omission view;
   - the **approval / publish sheet** as a list of
     `approval_strip_cell` rows, each one citing one
     `approval_strip_state_class` from the closed six-value vocabulary
     (`approval_strip_state_provider_authoritative_approved`,
     `approval_strip_state_provider_authoritative_changes_requested`,
     `approval_strip_state_provider_authoritative_dismissed`,
     `approval_strip_state_draft_only_local_pending_publish`,
     `approval_strip_state_publish_later_queued`,
     `approval_strip_state_provider_unreachable_local_continues`)
     plus an `approval_ticket_ref` that is required (non-null) for the
     three `provider_authoritative_*` states and the
     `publish_later_queued` state and forbidden (null) for the
     `draft_only_local_pending_publish` and
     `provider_unreachable_local_continues` states; the publish sheet
     itself names the row's
     `publish_sheet_publication_state_class` from the closed
     four-value vocabulary
     (`publish_sheet_synced_with_provider`,
     `publish_sheet_draft_only_local`,
     `publish_sheet_publish_later_queued`,
     `publish_sheet_export_only_no_provider_publish`);
   - the **stack / dependency strip** as a list of
     `stack_strip_cell` rows, each one citing one upstream
     `patch_stack_member_id_ref` plus one
     `stack_strip_dependency_state_class` from the closed five-value
     vocabulary
     (`stack_dependency_admissible_member_landed`,
     `stack_dependency_pending_upstream_member_not_yet_landed`,
     `stack_dependency_blocked_upstream_member_blocked`,
     `stack_dependency_drifted_upstream_member_re_evaluation_required`,
     `stack_dependency_superseded_upstream_member_superseded`),
     so a downstream entry inherits the upstream block reason rather
     than re-deriving one;
   - reserved refs for `linked_work_item_refs`,
     `linked_review_pack_id_ref`, `merge_queue_entry_id_ref`, and
     `merge_policy_record_id_ref`; and
   - the audit-event row shape on the `review_workspace_state` audit
     stream.

3. The `merge_queue_entry_record` shape every merge-queue panel reads,
   including:
   - the bound `target_review_workspace_id_ref` plus the bound
     `provider_rule_snapshot_id_ref` (an opaque ref into the
     review-workspace contract's `provider_rule_snapshot` model;
     this row never duplicates the snapshot body);
   - one `merge_queue_entry_class` from the closed five-value
     vocabulary
     (`queue_eligible_admissible`,
     `queue_blocked_pending_resolution`,
     `queue_landed`,
     `queue_dequeued_by_actor`,
     `queue_failed_to_land_pending_review`);
   - one `mergeability_class` from the closed seven-value vocabulary
     (`mergeable_clean`,
     `mergeable_with_conflicts_pending_resolution`,
     `not_mergeable_required_check_failing`,
     `not_mergeable_required_review_missing_or_dismissed`,
     `not_mergeable_base_drifted_re_evaluation_required`,
     `not_mergeable_approval_invalidated_by_force_push`,
     `not_mergeable_policy_or_trust_blocked`);
   - one `queue_entry_blocked_reason` from the closed nine-value
     vocabulary
     (`not_blocked_entry_admissible`,
     `provider_rule_snapshot_stale`,
     `required_check_unverified_or_failing`,
     `required_review_missing_or_dismissed`,
     `base_drifted_relative_to_workspace`,
     `approval_invalidated_by_force_push`,
     `upstream_stack_member_blocked`,
     `policy_epoch_expired_re_evaluation_required`,
     `workspace_trust_unset_or_restricted`)
     paired through allOf with `merge_queue_entry_class` so a
     `queue_eligible_admissible` row that names any other reason
     denies with
     `merge_queue_entry_must_not_appear_eligible_under_blocked_reason`;
   - one `local_vs_provider_rerun_authority_class` from the closed
     four-value vocabulary
     (`local_rerun_admissible_provider_rerun_unavailable`,
     `provider_rerun_authoritative_local_rerun_advisory`,
     `both_rerun_authorities_admissible_user_picks`,
     `no_rerun_admissible_re_evaluation_required_against_fresh_base`);
   - the `merge_queue_entry_lifecycle_state` from the closed
     six-value vocabulary
     (`proposed`, `enqueued`, `landed`, `dequeued`, `failed`,
     `superseded`);
   - the `stack_dependency_strip` (re-cited from the workspace state
     row so the entry inherits the stack view rather than re-mints
     it); and
   - the audit-event row shape on the `merge_queue_entry` audit
     stream.

4. The acceptance invariants this contract enforces:

   - **Reviewer can compare local vs provider evidence.** Every
     checks-strip cell carries a non-null
     `strip_cell_authority_class`. A surface that renders a missing
     evaluator without publishing a typed
     `not_evaluated_on_this_surface` cell denies with
     `checks_strip_green_by_omission_forbidden`. Provider-authoritative
     and local-parity-estimate cells MUST cite different
     `review_evaluation_result_id_ref` values (the result rows already
     pin their `evaluation_source_class`); a strip that pins the same
     result id under both authority classes denies with
     `checks_strip_authority_must_match_evaluation_source`.
   - **Offline state remains visibly distinct.** Every approval-strip
     cell carries a non-null `approval_strip_state_class`. A
     `draft_only_local_pending_publish` or
     `publish_later_queued` cell that also carries a non-null
     `provider_published_at` denies with
     `approval_strip_publish_state_must_match_authority_class`. A
     `provider_authoritative_*` cell that omits `approval_ticket_ref`
     denies with
     `approval_strip_publish_must_cite_approval_ticket`. The
     publish-sheet's `publish_sheet_publication_state_class` MUST
     match the strip cells per the allOf gate.
   - **Queue entries cannot appear eligible under a blocked reason.**
     A `queue_eligible_admissible` row that names any
     non-`not_blocked_entry_admissible` reason denies with
     `merge_queue_entry_must_not_appear_eligible_under_blocked_reason`.
     A `queue_blocked_pending_resolution` row that names
     `not_blocked_entry_admissible` denies with the same reason. A
     queue entry whose mergeability is
     `not_mergeable_*` MUST NOT carry
     `queue_entry_class = queue_eligible_admissible`.
   - **Stacked dependencies inherit upstream blocks.** A queue entry
     whose stack-strip carries a
     `stack_dependency_blocked_upstream_member_blocked` cell MUST
     carry `merge_queue_entry_class = queue_blocked_pending_resolution`
     and `queue_entry_blocked_reason = upstream_stack_member_blocked`;
     a row that asserts the cell but resolves to
     `queue_eligible_admissible` denies with
     `merge_queue_entry_must_not_appear_eligible_under_blocked_reason`.
   - **Provider outage continues local review.** An inbox row whose
     `inbox_freshness_sync_class` is
     `inbox_row_unverifiable_provider_unreachable` MUST carry
     `review_inbox_state_class` of
     `local_parity_estimate`,
     `not_evaluated_on_this_surface`, or
     `draft_only_no_provider_state`; a row that retains
     `provider_authoritative` under an unverifiable freshness denies
     with
     `inbox_state_authority_must_not_outlive_freshness`. The
     `offline_continuation_class` MUST be one of
     `provider_outage_local_continues_publish_blocked`,
     `provider_outage_local_continues_publish_queued`, or
     `offline_export_only_no_publish_later` for the outage path.
   - **Forward links resolve.** Every record carries a
     `merge_policy_record_id_ref` slot (currently always `null` until
     a merge-policy resolver lands) and the inbox row's reserved refs
     resolve through the existing review-workspace, review-pack,
     review-evaluation-result, work-item, and merge-queue-entry
     record families. Repurposing a reserved slot is a breaking
     change that requires a new decision row.

Out of scope until a superseding decision row opens:

- Implementing any provider HTTP / OAuth / GitHub / GitLab /
  Bitbucket / Azure DevOps adapter. The contract reserves the
  inbox-row, workspace-state, and merge-queue-entry shape; the wire
  integration is a later lane.
- Implementing a real merge queue, batch policy, kernel, or worker.
  The contract reserves the entry shape; the engine is a later lane.
- Building the full hosted-review reader UI (split-diff renderer,
  comment threads, hover cards, suggestion editor). The contract
  reserves the data model the UI binds to.
- Cross-repo review, multi-host review aggregation, and queue
  federation. Out of scope at this revision.

## 1. The review-inbox row

Every pending-review-inbox surface MUST resolve the row it is
rendering to exactly one `review_inbox_row_record`. The record is the
answer to four questions a reviewer must be able to answer without
opening any other object:

1. *Why is this row in my inbox?* — `review_inbox_row_class`.
2. *Is this row provider-authoritative right now?* —
   `review_inbox_state_class` plus `inbox_freshness_sync_class`.
3. *If the provider is unreachable, can I still review / draft /
   publish-later?* — `offline_continuation_class`.
4. *What is the row's lifecycle posture?* —
   `review_inbox_lifecycle_state`.

### 1.1 Inbox-class vocabulary

`review_inbox_row_class` is closed and exhaustive. The same review
workspace MAY appear in more than one row class for the same actor
(e.g. an actor authored a workspace AND was @-mentioned in a thread on
it); each class is its own row with its own state.

### 1.2 Inbox-state vocabulary

`review_inbox_state_class` is the inbox-level authority chip:

- `provider_authoritative` — the row's headline cues are owned by the
  provider; the inbox MUST cite a fresh `inbox_freshness_sync_class`
  to keep the chip honest.
- `local_parity_estimate` — the row's headline cues are computed
  locally from a fresh provider snapshot; the inbox MUST surface a
  typed "estimated locally" disclosure.
- `stale_relative_to_base_or_head` — the row's headline cues are
  pinned to a base or head identity that has drifted; the row remains
  visible but the inbox MUST NOT relabel the chip as authoritative.
- `not_evaluated_on_this_surface` — the row exists for triage / link
  purposes only; this surface did not evaluate the workspace.
- `draft_only_no_provider_state` — the row is a local draft (e.g.
  the actor opened a workspace they have not yet published to a
  provider). The provider has no overlay for this workspace.

### 1.3 Offline-continuation vocabulary

`offline_continuation_class` is closed:

- `online_synced_no_offline_state` — the actor is online and the row
  has no offline draft / queued action.
- `offline_capture_pending_publish_later` — the actor authored a
  comment / approval / dismissal locally that has not been published
  yet; the inbox MUST surface a "publish-later queued" cue.
- `offline_export_only_no_publish_later` — the row supports export
  only; no publish-later action is admissible (e.g. an
  imported-bundle row whose provider authority is forbidden until the
  user opts the workspace back to a composite posture).
- `provider_outage_local_continues_publish_blocked` — the provider
  is unreachable and publish-later actions are forbidden under this
  row's policy / trust posture. Local review continues.
- `provider_outage_local_continues_publish_queued` — the provider
  is unreachable but publish-later actions are admissible and have
  been queued for the next sync.

The acceptance invariant: an inbox row whose
`inbox_freshness_sync_class` is
`inbox_row_unverifiable_provider_unreachable` MUST resolve
`review_inbox_state_class` to one of `local_parity_estimate`,
`not_evaluated_on_this_surface`, or
`draft_only_no_provider_state`, and `offline_continuation_class` to
one of `provider_outage_local_continues_publish_blocked`,
`provider_outage_local_continues_publish_queued`, or
`offline_export_only_no_publish_later`. A row that names
`provider_authoritative` under an unverifiable freshness denies with
`inbox_state_authority_must_not_outlive_freshness`.

### 1.4 Lifecycle states

`review_inbox_lifecycle_state` is closed:

- `open_pending_action` — the row is pending action by the actor.
- `acknowledged_no_action_required` — the row was acknowledged but no
  further action is required (e.g. a follow-only row).
- `dismissed_by_actor` — the actor dismissed the row.
- `archived_tombstone` — the row was archived for retention or
  cleanup; the row remains as a tombstone for audit / restore.

## 2. The review-workspace state record

Every review-workspace surface MUST resolve to exactly one
`review_workspace_state_record` per `review_workspace_id_ref`. The
record carries four strips:

1. *header strip* — bound workspace, head / base refs, drift classes.
2. *checks / evaluation strip* — list of cells, each one a
   review-evaluation-result citation plus an authority class.
3. *approval / publish sheet* — list of approval cells plus the
   publish-sheet publication state.
4. *stack / dependency strip* — list of upstream patch-stack member
   citations plus a dependency-state class.

### 2.1 Strip-cell authority

`strip_cell_authority_class` is closed and is the only legal answer
to the question "is this cell provider-authoritative or a local
parity estimate?". The five values match the inbox-state vocabulary
so a downstream surface that lifts a cell into the inbox row does not
need to translate.

The acceptance invariant: a checks-strip cell that omits a
`review_evaluation_result_id_ref` AND omits a typed
`not_evaluated_on_this_surface` authority class denies with
`checks_strip_green_by_omission_forbidden`. A cell whose authority
class is `provider_authoritative` MUST cite a result whose
`evaluation_source_class` is `provider_overlay_evaluation`; a cell
whose authority class is `local_parity_estimate` MUST cite a result
whose `evaluation_source_class` is one of
`local_workstation_evaluation`, `ci_pipeline_evaluation`,
`ai_review_overlay_evaluation`, or `browser_companion_evaluation`. A
mismatch is a documented invariant the strip surface MUST refuse to
publish; the matched audit denial is
`checks_strip_authority_must_match_evaluation_source`.

### 2.2 Approval-strip and publish-sheet states

`approval_strip_state_class` is closed and is the only legal answer
to the question "has this approval crossed the wire?". The
`approval_ticket_ref` is required (non-null) on the three
`provider_authoritative_*` states and on the
`publish_later_queued` state; it is forbidden (null) on the
`draft_only_local_pending_publish` and
`provider_unreachable_local_continues` states. A row that asserts a
`provider_authoritative_*` state without an `approval_ticket_ref`
denies with `approval_strip_publish_must_cite_approval_ticket`. A row
that asserts a `draft_only_local_pending_publish` state with a
non-null `approval_ticket_ref` denies with
`approval_strip_publish_state_must_match_authority_class`.

`publish_sheet_publication_state_class` is closed and reads the
publish-sheet level posture (the per-cell state above is per-row;
this class is per-sheet). The allOf gate forbids a sheet whose state
is `publish_sheet_synced_with_provider` from carrying any
`draft_only_local_pending_publish` or `publish_later_queued` cell;
the failure denies with
`publish_sheet_publication_state_must_match_strip_cells`.

### 2.3 Stack / dependency strip

`stack_strip_dependency_state_class` is closed. The acceptance
invariant: a stack-strip cell whose state is
`stack_dependency_blocked_upstream_member_blocked` MUST appear on a
workspace whose merge-queue entry resolves to
`queue_blocked_pending_resolution` /
`upstream_stack_member_blocked`; a workspace asserting that the cell
is admissible while its queue entry resolves to
`queue_eligible_admissible` denies through the merge-queue-entry
contract's
`merge_queue_entry_must_not_appear_eligible_under_blocked_reason`.

## 3. The merge-queue entry

Every merge-queue panel MUST resolve every queue row to exactly one
`merge_queue_entry_record`. The record is the answer to five questions
a reviewer must be able to answer without opening the provider:

1. *Which workspace is this entry for?* —
   `target_review_workspace_id_ref`.
2. *Is this entry queue-eligible right now, and if not, why not?* —
   `merge_queue_entry_class` plus `queue_entry_blocked_reason` plus
   `mergeability_class`.
3. *Which provider rule snapshot is the entry pinned to?* —
   `provider_rule_snapshot_id_ref` (cited from the review-workspace
   contract; never duplicated).
4. *If a rerun is needed, who has authority to run it?* —
   `local_vs_provider_rerun_authority_class`.
5. *Does the entry inherit a block from an upstream stack member?* —
   the stack-dependency strip.

### 3.1 Mergeability vocabulary

`mergeability_class` is the user-readable mergeability cue:

- `mergeable_clean` — no conflicts, no blocking checks, fresh base.
- `mergeable_with_conflicts_pending_resolution` — the workspace has
  conflicts the user MUST resolve before landing; the queue entry
  MUST be `queue_blocked_pending_resolution`.
- `not_mergeable_required_check_failing` — a required check failed.
- `not_mergeable_required_review_missing_or_dismissed` — a required
  review is missing or has been dismissed.
- `not_mergeable_base_drifted_re_evaluation_required` — the
  workspace's base drifted relative to the captured provider rule
  snapshot.
- `not_mergeable_approval_invalidated_by_force_push` — a force-push
  invalidated the prior approval.
- `not_mergeable_policy_or_trust_blocked` — the policy epoch expired
  or workspace trust is unset / restricted.

### 3.2 Queue-entry-class and blocked-reason gating

`merge_queue_entry_class` is closed. The allOf gate:

- `queue_eligible_admissible` MUST cite
  `queue_entry_blocked_reason = not_blocked_entry_admissible`. A row
  that names any other reason denies with
  `merge_queue_entry_must_not_appear_eligible_under_blocked_reason`.
- `queue_blocked_pending_resolution` MUST cite a non-`not_blocked_*`
  reason; a row naming `not_blocked_entry_admissible` denies with
  the same reason.
- `queue_eligible_admissible` MUST NOT carry a `mergeability_class`
  beginning with `not_mergeable_`; a mismatch denies with
  `merge_queue_entry_mergeability_must_match_class`.

### 3.3 Rerun authority

`local_vs_provider_rerun_authority_class` is closed. A workspace
whose provider overlay is unreachable resolves to
`local_rerun_admissible_provider_rerun_unavailable`; a workspace
where both authorities are admissible (the actor has scope to push a
rerun on the provider AND the local clone can replay) resolves to
`both_rerun_authorities_admissible_user_picks`. A row that asserts
`local_rerun_admissible_provider_rerun_unavailable` while the
workspace's overlay freshness is `provider_overlay_fresh` denies with
`merge_queue_entry_rerun_authority_must_match_overlay_freshness`.

## 4. Forward dependencies

Two refs are reserved on every record:

- `merge_policy_record_id_ref` — the merge-policy resolver is a
  forward dependency. Once it lands, every queue entry MUST cite the
  merge-policy row that resolved the entry's admission. Until then,
  the slot is reserved and always `null`.
- `linked_work_item_ref` (on inbox rows and on the workspace-state
  publish sheet) — the work-item contract is upstream; this ref is
  optional today and remains optional. It is never required by this
  contract.

The reserved slots survive the next contract's landing without a
breaking change: today they are nullable; later they become required
non-null when the upstream contract lands and bumps the
`review_inbox_row_schema_version` /
`review_workspace_state_schema_version` /
`merge_queue_entry_schema_version` constants.

## 5. Audit streams

Three audit streams are reserved by this contract:

- `review_inbox_row_audit_event` — closed event-id vocabulary
  including `review_inbox_row_opened`,
  `review_inbox_row_state_changed`,
  `review_inbox_row_offline_capture_added`,
  `review_inbox_row_publish_later_queued`,
  `review_inbox_row_acknowledged`,
  `review_inbox_row_dismissed`,
  `review_inbox_row_archived`,
  `review_inbox_row_audit_denial_emitted`. Denial events MUST cite
  one denial reason from the `review_inbox_row_denial_reason`
  vocabulary.
- `review_workspace_state_audit_event` — closed event-id vocabulary
  including `review_workspace_state_opened`,
  `review_workspace_state_strip_cell_added`,
  `review_workspace_state_strip_cell_authority_changed`,
  `review_workspace_state_approval_strip_cell_added`,
  `review_workspace_state_publish_sheet_state_changed`,
  `review_workspace_state_stack_strip_cell_added`,
  `review_workspace_state_archived`,
  `review_workspace_state_audit_denial_emitted`.
- `merge_queue_entry_audit_event` — closed event-id vocabulary
  including `merge_queue_entry_proposed`,
  `merge_queue_entry_enqueued`,
  `merge_queue_entry_blocked`,
  `merge_queue_entry_landed`,
  `merge_queue_entry_failed_to_land`,
  `merge_queue_entry_dequeued`,
  `merge_queue_entry_audit_denial_emitted`.

The denial-reason vocabularies are listed in the schemas. Adding a
new denial reason or audit-event id is additive-minor and bumps the
per-record schema-version const; repurposing an existing value is
breaking and requires a new decision row.

## 6. Redaction posture

Every record published against this contract carries one
`redaction_class` from the re-exported capability-lifecycle
vocabulary. Defaults:

- Local-only inbox rows / workspace-state rows / queue entries
  default to `metadata_safe_default`.
- Inbox rows whose underlying workspace is
  `provider_overlay_fetched`, `composite_local_with_provider_overlay`,
  or `browser_handoff_token_source` MAY raise to
  `internal_support_restricted` when the row touches organisationally
  restricted review state.
- Queue entries whose actor or approval ticket touches a credentialed
  flow MUST raise to `operator_only_restricted`.

Raw absolute paths, raw branch / commit URLs, raw author identity
strings, raw comment bodies, raw provider rule bodies, raw
approval-ticket bodies, raw notebook cell text, raw terminal bytes,
and raw URLs never appear on any record published against this
contract. Every payload travels by opaque ref or through the
redaction-aware reviewable-label registry.

## 7. Acceptance cross-walk

| Acceptance bullet from the plan | Where it lands |
|---|---|
| Reviewers can compare local evidence with provider evidence without collapsing them into one ambiguous "ready to merge" badge. | §2.1 strip-cell authority class plus the `checks_strip_green_by_omission_forbidden` and `checks_strip_authority_must_match_evaluation_source` denials. Fixtures `local_parity_only.yaml` and `provider_authoritative_checks.yaml`. |
| Offline-captured comments, draft approvals, and publish-later actions remain visibly distinct from provider-synced state. | §1.3 offline-continuation vocabulary + §2.2 approval-strip vocabulary plus the `approval_strip_publish_must_cite_approval_ticket`, `approval_strip_publish_state_must_match_authority_class`, and `publish_sheet_publication_state_must_match_strip_cells` denials. Fixture `provider_outage_local_continuation.yaml`. |
| Fixtures cover at least: local parity only, provider-authoritative checks, merge-queue blocked on stale base, stacked review dependency, and provider outage with continued local review/export path. | Fixtures `local_parity_only.yaml`, `provider_authoritative_checks.yaml`, `merge_queue_blocked_stale_base.yaml`, `stacked_review_dependency.yaml`, and `provider_outage_local_continuation.yaml`. |

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
