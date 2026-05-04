# Offline-draft packet and provider-reconciliation contract

This document freezes the cross-tool **offline-draft** and
**provider-reconciliation** object model that every Aureline surface
reads when a user (or AI assistant pending user confirmation) authors
a local change set against a provider-owned (or local-only) work item
under conditions where the apply path cannot run synchronously: a
provider outage, an intentional offline-only session, a managed-
workstation no-system-browser environment, a restricted-trust
workspace, an unsatisfied freshness floor, a pending policy-epoch
re-evaluation, a pending account-mapping resolution, a pending step-up
authenticator, or a user-explicit deferral into a draft.

The goal is to preserve work-item continuity under provider outage
or intentional offline work so Aureline can queue, reconcile, and
explain deferred changes safely. Specifically, on every surface that
touches the deferred change, Aureline can disclose:

- *what* the user authored (typed local change set rows pinning per
  provider field overwrite / append / unset / unchanged-local-only,
  per linked review / change object / patch stack / branch / worktree
  update, per validation-evidence attach / detach, per blocking
  relationship change);
- *which target* the change would publish to once reconciled (the
  intended target account / install grant / delegated token /
  browser-handoff session / managed-admin lane / pending-mapping /
  local-only target);
- *which expiry window* the deferred change is admissible under (the
  authored-at / expires-at / freshness-floor / extension-admissibility
  truth so reviewers see "safe to publish later" vs "must re-review"
  mechanically);
- *which reconciliation strategy* the apply path will resolve the
  draft under (try-publish-now-when-online, queue-for-publish-later,
  route-through-browser-handoff, capture-offline-pending-drain,
  inspect-only-what-if, re-review-required-after-expiry,
  archive-local-only);
- *which conflict* the reconciliation observed (the typed conflict-
  origin axis plus per-field conflict rows pinning local-draft vs
  provider-remote labels) when the apply path drains and finds the
  provider state has drifted, the remote object has been superseded,
  the connected account has been revoked, or the bound account
  mapping has remapped.

The machine-readable boundaries are:

- [`/schemas/work_items/offline_draft_packet.schema.json`](../../schemas/work_items/offline_draft_packet.schema.json)
  — `offline_draft_packet_record` and
  `offline_draft_packet_audit_event_record`.
- [`/schemas/work_items/provider_reconciliation.schema.json`](../../schemas/work_items/provider_reconciliation.schema.json)
  — `provider_reconciliation_record` and
  `provider_reconciliation_audit_event_record`.

Worked fixtures (no-network draft, queued publish within expiry,
expired queued action requiring re-review, and conflict between local
draft and provider-updated state) live under
[`/fixtures/work_items/offline_reconciliation_cases/`](../../fixtures/work_items/offline_reconciliation_cases/).

This contract **composes with and does not replace** the upstream
contracts it cites:

- [`/docs/work_items/work_item_contract.md`](work_item_contract.md)
  and the schemas
  [`/schemas/work_items/work_item_detail.schema.json`](../../schemas/work_items/work_item_detail.schema.json),
  [`/schemas/work_items/status_transition_packet.schema.json`](../../schemas/work_items/status_transition_packet.schema.json),
  and
  [`/schemas/work_items/offline_handoff_packet.schema.json`](../../schemas/work_items/offline_handoff_packet.schema.json)
  — `work_item_detail_record` is the per-row header the offline-draft
  packet binds back to; `status_transition_packet_record` is the
  previewed mutation the draft was authored against;
  `offline_handoff_packet_record` is the captured-under-unavailability
  packet the offline-draft attaches to (one packet pair per offline
  capture).
- [`/docs/work_items/change_intent_and_publish_preview_contract.md`](change_intent_and_publish_preview_contract.md)
  and the schemas
  [`/schemas/work_items/change_intent.schema.json`](../../schemas/work_items/change_intent.schema.json)
  and
  [`/schemas/work_items/external_publish_preview.schema.json`](../../schemas/work_items/external_publish_preview.schema.json)
  — `change_intent_record` is the rationale / target scope / linked
  review / approvals / validation evidence record the offline-draft
  forwards through `linked_change_intent_record_id_ref`;
  `external_publish_preview_record` is the projected before / after
  labels record the offline-draft forwards through
  `linked_external_publish_preview_record_id_ref`.
- [`/docs/work_items/status_transition_review_contract.md`](status_transition_review_contract.md)
  and the schema
  [`/schemas/work_items/transition_review.schema.json`](../../schemas/work_items/transition_review.schema.json)
  — the offline-draft packet is the *authored* draft; the transition-
  review packet is the typed user-facing review of the previewed
  mutation. Both compose: a deferred transition under a transition-
  review packet's `admissible_via_queue_for_publish_later` /
  `admissible_via_browser_handoff_only` /
  `deferred_publish_captured_offline_pending_drain` lane materialises
  one offline-draft packet that the apply path drains through one
  provider-reconciliation record.
- [`/docs/work_items/provider_object_and_traceability_contract.md`](provider_object_and_traceability_contract.md)
  and the schemas
  [`/schemas/work_items/provider_object.schema.json`](../../schemas/work_items/provider_object.schema.json)
  and
  [`/schemas/work_items/traceability_link.schema.json`](../../schemas/work_items/traceability_link.schema.json)
  — `provider_field_class` is re-exported verbatim onto local-change-
  set rows (per typed provider field overwrite / append / unset /
  unchanged-local-only / pending-resolution) and onto the
  reconciliation packet's per-field conflict rows;
  `traceability_link_record` is cited by reference from local-change-
  set rows that cross the work-item boundary.
- [`/docs/providers/provider_mode_contract.md`](../providers/provider_mode_contract.md)
  and the schemas
  [`/schemas/providers/publish_later_record.schema.json`](../../schemas/providers/publish_later_record.schema.json),
  [`/schemas/providers/provider_callback_envelope.schema.json`](../../schemas/providers/provider_callback_envelope.schema.json)
  — `publish_later_queue_item_record`,
  `provider_consequence_preview_record`,
  `account_mapping_binding_record`, and
  `provider_callback_envelope_record` are cited by reference from the
  offline-draft and reconciliation packets.
- [`/docs/integration/browser_handoff_contract.md`](../integration/browser_handoff_contract.md)
  / ADR-0010 — `browser_handoff_packet_record` and the typed
  approval-ticket lifecycle. Browser-handoff dispositions cite the
  packet by reference.
- ADR-0001 / ADR-0007 / ADR-0010 / ADR-0011 / ADR-0018 — workspace
  trust, secret-broker handle / raw-secret-forbidden boundary,
  browser-handoff and approval-ticket envelope, capability lifecycle
  / freshness / client-scope / redaction, and workspace-trust state.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — work-item,
  provider-mode, offline-draft, publish-later, and reconciliation
  passages.
- `.t2/docs/Aureline_Technical_Design_Document.md` — offline-draft
  packet, provider-reconciliation outcome, and stale-intent / expiry
  passages.
- `.t2/docs/Aureline_PRD.md` — work-item MUST / SHOULD language for
  truthful preservation of deferred changes under provider outage
  or offline work, typed expiry / re-review thresholds, and forbidden
  silent fan-out of expired or conflicting drafts.

If this contract disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same change.

## Why freeze this now

Without one frozen offline-draft and reconciliation contract: the
issue / planning surface drops the user's offline edit on the floor
when the network returns and the form re-renders against fresh
remote state; the publish-later queue silently advances a queued
provider mutation past its safe window because no expiry truth is
recorded; a queued action whose authored target account has since
been revoked or remapped silently mutates against the *new* account;
a queued action whose target object has been superseded remotely
(closed, merged into another issue, deleted) silently re-applies the
local draft and clobbers remote work; and the support-handoff
packet exports the deferred change as free text that doesn't
reconcile on re-import.

Worse, when the user authors an offline draft against a provider
issue and then walks away for a day, the apply path has no typed way
to distinguish "this is safe to publish later — same target,
no drift, no policy roll" from "this must be re-reviewed — the
remote has drifted, the policy epoch has rolled, the account has
remapped, the workspace trust has been downgraded" without
inspecting provider logs.

This contract closes that gap with **two records**:

1. one `offline_draft_packet_record` that pins the typed local change
   set, the typed intended target, the typed expiry window with
   extension-admissibility, the typed reconciliation strategy, the
   typed conflict-disclosure block (initially empty; populated when
   reconciliation observes drift), and the typed draft-lifecycle
   class;
2. one `provider_reconciliation_record` that pins the typed
   reconciliation outcome (clean publish, conflict, superseded
   remote state, missing authority, account mismatch, archived
   local-only, or re-review required for one of five typed reasons)
   plus the typed missing-authority class, the per-field conflict
   rows, and the typed next-safe-action so a downstream surface
   knows mechanically whether to offer try-again / re-review /
   archive / open-account-mapping.

## Scope

Frozen at this revision:

- one `offline_draft_packet_record` per authored draft over a work-
  item detail row — with the typed `draft_origin_class`,
  `intended_target_block`, `local_change_set` (ordered, minItems-1,
  one row per typed change kind), `expiry_window_block` (authored_at
  / expires_at / freshness_floor_ref / extension_admissibility_class),
  `reconciliation_strategy_class`, `conflict_disclosure_block`
  (typed conflict-origin axis plus ordered conflict-field rows),
  `draft_lifecycle_class`, lineage refs to the bound change-intent /
  status-transition packet / external-publish preview / offline-
  handoff packet / publish-later queue item / browser-handoff packet
  / account-mapping binding / provider-reconciliation record, and the
  shared `origin_disclosure` / `policy_context` / `redaction_class`
  blocks;
- one `provider_reconciliation_record` per drain attempt over an
  offline-draft packet — with the typed `reconciliation_outcome_class`
  (eleven values), `missing_authority_class` (ten values, including
  no_missing_authority), `conflict_origin_class` (re-exported
  verbatim from the offline-draft packet), `conflict_field_rows`,
  `next_safe_action_class` (nineteen values), and lineage refs to
  the bound publish-later queue item / browser-handoff packet /
  offline-handoff packet / account-mapping binding / provider-callback
  envelopes / provider-consequence preview / superseded predecessor;
- one closed audit-event vocabulary per record family;
- one closed denial-reason vocabulary per record family.

## Out of scope

- Implementing offline sync engines, provider merge logic, or
  conflict-resolution UI. This contract freezes the typed
  vocabularies those engines and surfaces read and write.
- Provider HTTP / OAuth / webhook / signature-verification protocols.
- Final user-facing copy. The schemas freeze the typed vocabulary;
  copy lives in the design system.

## 1. Offline-draft packet

Every authored deferred change over a work item resolves through
one `offline_draft_packet_record`. The record carries:

- a `draft_origin_class` from the sixteen-value frozen vocabulary
  (nine `human_authored_*` lanes, `ai_proposed_pending_user_confirmation`,
  four `imported_from_*` lanes, and two admin-minted lanes);
- an `intended_target_block` pinning the typed `target_account_class`
  (re-exported from `external_publish_preview.schema.json`) plus the
  bound `target_account_subject_ref`,
  `connected_provider_record_id_ref`, `target_object_identity_ref`,
  and reviewable `target_object_label`;
- a `local_change_set` of one or more ordered rows over the closed
  fifteen-value `local_change_kind_class` set
  (`provider_field_overwrite`, `provider_field_append`,
  `provider_field_unset`, `provider_field_unchanged_local_only`,
  `provider_field_pending_resolution`, `comment_append_local_draft`,
  `linked_review_update_local_draft`,
  `linked_change_or_patch_stack_update_local_draft`,
  `linked_branch_or_worktree_attach_or_detach_local_draft`,
  `validation_evidence_attach_or_detach_local_draft`,
  `blocking_relationship_change_local_draft`,
  `subscriber_or_watcher_change_local_draft`,
  `rename_or_metadata_change_local_draft`,
  `release_artifact_publish_local_draft`, `other_local_change`);
- an `expiry_window_block` pinning the typed `authored_at` /
  `expires_at` / optional `freshness_floor_ref` /
  `extension_admissibility_class` over the five-value vocabulary
  (`extension_not_admissible_re_review_after_expiry_required`,
  `extension_admissible_within_freshness_floor`,
  `extension_admissible_via_explicit_user_re_confirmation`,
  `extension_admissible_via_managed_admin_only`,
  `extension_blocked_by_policy_admin`);
- a `reconciliation_strategy_class` from the seven-value frozen
  vocabulary (`try_publish_now_when_online_within_expiry`,
  `queue_for_publish_later_within_expiry`,
  `route_through_browser_handoff_when_browser_available`,
  `capture_offline_pending_drain`, `inspect_only_what_if_no_apply`,
  `re_review_required_after_expiry`,
  `archive_local_only_no_provider_path`);
- a `conflict_disclosure_block` pinning the typed
  `conflict_origin_class` plus ordered `conflict_field_rows`
  (initially empty with `no_conflict_observed`; populated when
  reconciliation observes drift);
- a `draft_lifecycle_class` from the fifteen-value frozen vocabulary
  (`drafted_pending_reconciliation`,
  `queued_for_publish_within_expiry`,
  `routed_through_browser_handoff_pending_browser`,
  `captured_offline_pending_drain`,
  `published_clean_via_provider_reconciliation`,
  `blocked_by_conflict_pending_re_review`,
  `blocked_by_superseded_remote_state_pending_re_review`,
  `blocked_by_missing_authority_pending_user_action`,
  `blocked_by_account_mismatch_pending_user_action`,
  `blocked_by_workspace_trust_unset_or_restricted`,
  `blocked_by_policy_admin_suppressed`,
  `expired_re_review_required`,
  `archived_local_only_no_provider_path`,
  `withdrawn_before_apply`,
  `superseded_by_offline_draft_packet_record`);
- lineage refs to the bound change-intent / status-transition packet
  / external-publish preview / offline-handoff packet / publish-
  later queue item / browser-handoff packet / account-mapping
  binding / provider-reconciliation record / superseded predecessor;
- the shared `origin_disclosure`, `policy_context`, and
  `redaction_class` blocks.

The record never carries raw provider URLs, raw provider issue
bodies, raw comment bodies, raw delegated tokens, raw branch / commit
URLs, raw author identity strings, raw absolute paths, raw
notification payloads, or raw automation payloads. All identity
crosses the boundary as opaque refs and reviewable labels
(<= 1024 graphemes).

### 1.1 Local-change-set rows

Every row pins the typed truth for one observable change in the
draft. Per-row gates:

- `provider_field_overwrite` and `provider_field_append` rows MUST
  cite a typed `provider_field_class` (outside `other`) plus
  non-empty `before_state_value_label` and
  `after_state_value_label`.
- `provider_field_unset` rows MUST cite a typed `provider_field_class`
  plus a non-empty `before_state_value_label` (the value being
  unset).
- `provider_field_unchanged_local_only` rows MUST cite a typed
  `provider_field_class` plus a non-empty `local_overlay_value_label`
  (the locally-only overlay value that does not publish).
- `linked_review_update_local_draft`,
  `linked_change_or_patch_stack_update_local_draft`,
  `linked_branch_or_worktree_attach_or_detach_local_draft`,
  `validation_evidence_attach_or_detach_local_draft`,
  `blocking_relationship_change_local_draft`, and
  `release_artifact_publish_local_draft` rows MUST cite a non-empty
  `linked_artifact_record_id_ref`.

### 1.2 Expiry-window and stale-intent rules

The schema's `allOf` gates pin the following invariants:

- `queue_for_publish_later_within_expiry` strategy MUST cite a
  non-empty `linked_publish_later_queue_item_record_id_ref`.
- `route_through_browser_handoff_when_browser_available` strategy
  MUST cite a non-empty `linked_browser_handoff_packet_ref`.
- `capture_offline_pending_drain` strategy MUST cite a non-empty
  `linked_offline_handoff_packet_record_id_ref`.
- `expired_re_review_required` lifecycle MUST resolve
  `reconciliation_strategy_class` to `re_review_required_after_expiry`.
- `published_clean_via_provider_reconciliation` lifecycle MUST cite
  a non-empty `linked_provider_reconciliation_record_id_ref` AND a
  non-empty `linked_provider_callback_envelope_record_id_refs` so a
  draft cannot flip to "published" without a fresh provider-callback
  observation. The denial reason
  `clean_publish_must_cite_provider_callback_envelope_record_id_ref`
  is the gate on the reconciliation packet.
- `blocked_by_conflict_pending_re_review` lifecycle MUST cite a
  non-empty `linked_provider_reconciliation_record_id_ref` AND
  resolve `conflict_disclosure_block.conflict_origin_class` outside
  `no_conflict_observed`.
- `blocked_by_superseded_remote_state_pending_re_review` /
  `blocked_by_missing_authority_pending_user_action` /
  `blocked_by_account_mismatch_pending_user_action` /
  `archived_local_only_no_provider_path` lifecycles MUST cite a
  non-empty `linked_provider_reconciliation_record_id_ref`.
- `blocked_by_account_mismatch_pending_user_action` lifecycle MUST
  cite a non-empty `linked_account_mapping_binding_record_id_ref`.
- Every `blocked_*` and `expired_re_review_required` lifecycle MUST
  cite a non-empty `block_reason_summary`.
- `withdrawn_before_apply` lifecycle MUST cite a non-empty
  `withdrawn_at`.
- `superseded_by_offline_draft_packet_record` lifecycle MUST cite a
  non-empty `superseded_by_offline_draft_packet_record_id_ref`.
- `ai_proposed_pending_user_confirmation` and `imported_from_*`
  origins MUST resolve `reconciliation_strategy_class` to
  `inspect_only_what_if_no_apply` or
  `archive_local_only_no_provider_path` only.
- `local_only_no_provider_account` target MUST resolve
  `reconciliation_strategy_class` to
  `archive_local_only_no_provider_path` or
  `inspect_only_what_if_no_apply`.
- `account_mapping_binding_pending_user_resolution` target MUST
  cite a non-empty `linked_account_mapping_binding_record_id_ref`.
- Captured-under-unavailability `human_authored_*` origins
  (`human_authored_provider_unreachable`,
  `human_authored_browser_handoff_blocked`) and the
  `imported_from_offline_handoff_packet_no_live_provider_path`
  origin MUST cite a non-empty
  `linked_offline_handoff_packet_record_id_ref`. Restricted-trust,
  freshness-unsatisfied, policy-pending, account-pending,
  step-up-pending, and user-deferred origins MAY (but need not) cite
  a bound offline-handoff packet.

### 1.3 Conflict disclosure

The `conflict_disclosure_block` is non-null on every offline-draft
packet. Its safe default is `conflict_origin_class = no_conflict_observed`
plus an empty `conflict_field_rows` list. When reconciliation
observes drift, the block flips to one of twelve typed conflict
origins (`provider_field_value_drifted_during_offline_window`,
`provider_object_lifecycle_state_drifted`,
`provider_object_split_or_merge_observed`,
`provider_object_deleted_or_archived_remotely`,
`provider_assignee_or_owner_changed_remotely`,
`provider_label_or_milestone_changed_remotely`,
`provider_blocking_relationship_changed_remotely`,
`linked_review_or_change_object_drifted_remotely`,
`freshness_floor_unsatisfied_remote_object_drifted`,
`account_remap_observed_during_offline_window`,
`policy_epoch_rolled_during_offline_window`,
`imported_handoff_replay_no_live_provider_path`) plus at least one
`conflict_field_row` carrying the local-draft and provider-remote
labels per affected provider field. The block also cites a non-empty
`linked_provider_reconciliation_record_id_ref` when the conflict was
observed (so the reconciliation packet is the audit trail).

## 2. Provider-reconciliation outcomes

Every drain attempt against an offline-draft packet resolves through
one `provider_reconciliation_record`. The record carries:

- a `reconciliation_outcome_class` from the eleven-value frozen
  vocabulary:
  - `clean_publish_admissible` — the apply path observed a fresh
    provider-callback envelope, the bound expiry has not elapsed,
    no conflict was observed, and no missing authority was found.
    `next_safe_action_class = no_action_already_published_clean`.
  - `conflict_observed_local_vs_provider` — at least one provider
    field has drifted between the local draft and the live remote
    state; per-field conflict rows pin the typed origin and labels.
    The bound offline-draft packet MUST flip to
    `blocked_by_conflict_pending_re_review`.
  - `superseded_by_remote_state` — the provider object has been
    superseded (closed, merged, deleted, split). The packet MUST
    cite a non-empty `remote_revision_label_after`.
  - `missing_authority_no_apply` — the apply path lacks authority
    (provider unreachable / health degraded / connected account
    revoked / token scope narrowed / managed-admin blocked /
    policy-admin suppressed / workspace-trust unset / step-up
    pending / browser-handoff blocked). The packet MUST cite a
    typed `missing_authority_class`.
  - `account_mismatch_no_apply` — the bound connected account no
    longer maps to the target object (account remap, account
    unbinding, project-scoped grant rotated). The packet MUST cite
    a non-empty `linked_account_mapping_binding_record_id_ref`.
  - `archived_local_only_no_provider_path` — the user (or imported-
    evidence-only origin) chose to archive the draft locally with
    no provider apply.
  - `re_review_required_expired_intent` — the bound offline-draft
    packet's `expires_at` has elapsed.
  - `re_review_required_freshness_drift` — the freshness floor was
    raised and the draft no longer satisfies it.
  - `re_review_required_account_remap` — the account remapped but
    a fresh re-review is admissible against the new mapping.
  - `re_review_required_policy_epoch_rolled` — the policy epoch
    rolled between authoring and reconciliation.
  - `re_review_required_workspace_trust_downgrade` — workspace
    trust was downgraded from trusted to restricted.
- a `missing_authority_class` (required, defaults to
  `no_missing_authority` for every outcome outside
  `missing_authority_no_apply`);
- the typed `conflict_origin_class` plus ordered
  `conflict_field_rows` (re-exported from the offline-draft packet's
  conflict-disclosure block);
- a `next_safe_action_class` from the nineteen-value frozen
  vocabulary so a downstream surface knows whether to offer
  try-again / re-review / archive / reselect-account / wait-for-
  recovery / open-in-browser-handoff;
- lineage refs to the bound publish-later queue item / browser-
  handoff packet / offline-handoff packet / account-mapping
  binding / provider-callback envelopes / provider-consequence
  preview / superseded predecessor;
- the shared `origin_disclosure`, `policy_context`, and
  `redaction_class` blocks.

### 2.1 Reconciliation gates

The schema's `allOf` gates pin the following invariants:

- `clean_publish_admissible` MUST cite a non-empty
  `linked_provider_callback_envelope_record_id_refs`, MUST resolve
  `missing_authority_class` to `no_missing_authority`, MUST resolve
  `conflict_origin_class` to `no_conflict_observed`, MUST resolve
  `next_safe_action_class` to `no_action_already_published_clean`,
  and MUST forbid any conflict_field_row.
- `conflict_observed_local_vs_provider` MUST cite a non-empty
  `linked_provider_callback_envelope_record_id_refs`, MUST resolve
  `conflict_origin_class` outside `no_conflict_observed`, MUST carry
  at least one conflict_field_row, MUST cite a non-empty
  `block_reason_summary`, AND MUST resolve `next_safe_action_class`
  to one of `re_review_required_then_retry`,
  `withdraw_offline_draft_packet`, or
  `archive_local_only_no_provider_path`.
- `superseded_by_remote_state` MUST cite a non-empty
  `linked_provider_callback_envelope_record_id_refs`, a non-empty
  `remote_revision_label_after`, AND a non-empty
  `block_reason_summary`.
- `missing_authority_no_apply` MUST resolve `missing_authority_class`
  outside `no_missing_authority`, MUST cite a non-empty
  `block_reason_summary`, AND MUST resolve `next_safe_action_class`
  to one of `wait_for_provider_health_recovery`,
  `wait_for_step_up_clearance`, `request_managed_admin_review`,
  `request_policy_admin_review`,
  `open_in_provider_via_browser_handoff`,
  `re_review_required_on_workspace_trust_downgrade`, or
  `withdraw_offline_draft_packet`.
- `account_mismatch_no_apply` MUST cite a non-empty
  `linked_account_mapping_binding_record_id_ref` AND a non-empty
  `block_reason_summary` AND resolve `next_safe_action_class` to
  `reselect_connected_account`,
  `resolve_account_mapping_binding`, or
  `withdraw_offline_draft_packet`.
- `archived_local_only_no_provider_path` MUST resolve
  `missing_authority_class` to `no_missing_authority`,
  `conflict_origin_class` to `no_conflict_observed`,
  `next_safe_action_class` to `archive_local_only_no_provider_path`,
  AND MUST forbid `linked_publish_later_queue_item_record_id_ref` /
  `linked_browser_handoff_packet_ref` /
  `linked_provider_callback_envelope_record_id_refs`.
- Each `re_review_required_*` outcome MUST resolve
  `next_safe_action_class` to its matching
  `re_review_required_*` action AND cite a non-empty
  `block_reason_summary`.
- `re_review_required_account_remap` MUST cite a non-empty
  `linked_account_mapping_binding_record_id_ref`.

## 3. Relation rules

The offline-draft and provider-reconciliation packets compose with
the upstream relation contracts:

- **Offline draft ↔ work-item detail header.** Every offline-draft
  packet MUST cite a non-empty `work_item_detail_record_id_ref`.
- **Offline draft ↔ change intent.** The
  `linked_change_intent_record_id_ref` is the path from the draft to
  the rationale / target scope / linked review / approvals /
  validation evidence record. Empty only when the draft was authored
  without a structured change intent.
- **Offline draft ↔ status-transition packet.** The
  `linked_status_transition_packet_record_id_ref` is the path to the
  previewed mutation. Empty admissible only when the draft is a
  comment-only or rename-only draft with no structured packet.
- **Offline draft ↔ external-publish preview.** Empty for
  `archive_local_only_no_provider_path` and
  `inspect_only_what_if_no_apply` strategies that have no provider
  preview.
- **Offline draft ↔ offline-handoff packet.** Required when the
  draft origin is one of the captured-under-unavailability lanes
  (provider unreachable, browser-blocked, restricted-trust,
  freshness-unsatisfied, policy-pending, account-pending,
  step-up-pending, user-deferred) or
  `imported_from_offline_handoff_packet_no_live_provider_path`.
- **Offline draft ↔ publish-later queue item / browser-handoff
  packet / account-mapping binding.** The matching
  `linked_*_record_id_ref` is required when the strategy or the
  lifecycle pins the matching deferred lane.
- **Reconciliation ↔ offline draft.** Every
  `provider_reconciliation_record` MUST cite a non-empty
  `offline_draft_packet_record_id_ref`. The reconciliation never
  floats free of a draft.
- **Reconciliation ↔ provider-callback envelopes.** Required
  non-empty for `clean_publish_admissible`,
  `conflict_observed_local_vs_provider`, and
  `superseded_by_remote_state` outcomes (the conflict / clean
  publish / supersession was observed via a provider read).

## 4. Truthful escape hatches

Every degraded reconciliation outcome exposes at least one truthful
escape hatch named through the `next_safe_action_class` vocabulary:

1. **Retry within expiry** —
   `retry_publish_now_within_expiry` /
   `retry_publish_via_publish_later_queue` /
   `retry_publish_via_browser_handoff` admit a typed retry path when
   the cause is transient (provider health flap, browser-handoff
   becomes available).
2. **Re-review** — `re_review_required_then_retry` /
   `re_review_required_after_expiry` /
   `re_review_required_on_freshness_drift` /
   `re_review_required_on_account_remap` /
   `re_review_required_on_policy_epoch_roll` /
   `re_review_required_on_workspace_trust_downgrade` pin a typed
   re-review path that mints a fresh transition-review packet
   against the current provider state.
3. **Account resolution** — `reselect_connected_account` /
   `resolve_account_mapping_binding` route through the account-
   mapping binding so the user can resolve the account mismatch
   without losing the local change set.
4. **Wait** — `wait_for_provider_health_recovery` /
   `wait_for_step_up_clearance` pin a typed wait state.
5. **Admin escalation** — `request_managed_admin_review` /
   `request_policy_admin_review` route the deferred change to the
   admin lane.
6. **Browser handoff** — `open_in_provider_via_browser_handoff`
   routes through a typed browser-handoff packet so the user can
   complete the action in the system browser.
7. **Local archival** — `archive_local_only_no_provider_path` pins
   a typed local-only archival with no provider apply.
8. **Withdraw** — `withdraw_offline_draft_packet` pins a typed
   withdrawal lifecycle (`withdrawn_before_apply` on the draft).

## 5. Cross-cutting record relations

A deferred change's life across all work-items records is:

- the `work_item_detail_record` is the **per-row header**;
- the `change_intent_record` is the **rationale + scope + linked
  review + side effects + approvals + evidence** record;
- the `external_publish_preview_record` is the **previewed publish
  with target account / context, before / after labels, deferred
  consequences**;
- the `status_transition_packet_record` is the **previewed mutation
  the apply path commits**;
- the `transition_review_record` is the **typed user-facing review
  packet** the user confirms against;
- the `offline_handoff_packet_record` is the **captured-under-
  unavailability snapshot** the draft attaches to;
- the `offline_draft_packet_record` (this contract) is the **typed
  authored deferred change** with local change set, intended target,
  expiry window, reconciliation strategy, and conflict disclosure;
- the `provider_reconciliation_record` (this contract) is the
  **typed apply-time outcome** the apply path mints, gating clean
  publish behind a fresh provider-callback observation and an
  unelapsed expiry, and pinning every blocked outcome to a typed
  next-safe-action.

## 6. Redaction posture (frozen)

Every record declares a `redaction_class` from the ADR-0010 /
ADR-0007 set (`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`). Raw
provider URLs, raw provider issue bodies, raw comment bodies, raw
label values that disclose customer / tenant identity, raw
delegated tokens, raw branch / commit URLs, raw author identity
strings, raw absolute paths, raw notification payloads, and raw
automation payloads MUST NOT cross this boundary on any surface
regardless of class. Exports, support bundles, mutation-journal
entries, evidence packets, replay captures, and AI context
captures carry opaque refs and structured fields only.

Narrowing is permitted: admin policy MAY remove
`try_publish_now_when_online_within_expiry` from a workspace,
forbid `route_through_browser_handoff_when_browser_available` on a
restricted-trust workspace, raise a freshness floor, pin the row to
`inspect_only_what_if_no_apply`, or suppress the AI-proposed draft
path entirely. Widening beyond the frozen rules is forbidden.

## 7. Audit-event reuse

Local-only offline-draft and provider-reconciliation lifecycle
events fire on the `work_items` audit stream using the closed event
ids:

### Offline-draft packet

- `offline_draft_packet_authored`
- `offline_draft_packet_local_change_set_amended`
- `offline_draft_packet_pinned_to_change_intent`
- `offline_draft_packet_pinned_to_status_transition_packet`
- `offline_draft_packet_admitted_to_publish_later_queue`
- `offline_draft_packet_routed_to_browser_handoff`
- `offline_draft_packet_captured_offline_pending_drain`
- `offline_draft_packet_reconciliation_attempted`
- `offline_draft_packet_published_clean`
- `offline_draft_packet_blocked_by_conflict`
- `offline_draft_packet_blocked_by_superseded_remote_state`
- `offline_draft_packet_blocked_by_missing_authority`
- `offline_draft_packet_blocked_by_account_mismatch`
- `offline_draft_packet_expired_re_review_required`
- `offline_draft_packet_archived_local_only`
- `offline_draft_packet_withdrawn_before_apply`
- `offline_draft_packet_superseded`
- `offline_draft_packet_audit_denial_emitted`

### Provider-reconciliation

- `provider_reconciliation_attempted`
- `provider_reconciliation_clean_publish_minted`
- `provider_reconciliation_conflict_observed`
- `provider_reconciliation_superseded_by_remote_state_observed`
- `provider_reconciliation_missing_authority_observed`
- `provider_reconciliation_account_mismatch_observed`
- `provider_reconciliation_archived_local_only`
- `provider_reconciliation_re_review_required_expired`
- `provider_reconciliation_re_review_required_freshness_drift`
- `provider_reconciliation_re_review_required_account_remap`
- `provider_reconciliation_re_review_required_policy_epoch_rolled`
- `provider_reconciliation_re_review_required_workspace_trust_downgrade`
- `provider_reconciliation_superseded`
- `provider_reconciliation_audit_denial_emitted`

Provider-side events (callback validation, queue drain, queue
rejection, handoff revocation) stay on the ADR-0010
`provider_handoff` audit stream. This contract introduces no new
ids on that stream; the offline-draft and reconciliation packets
are the *payloads* those frozen events reference.

## 8. Acceptance criteria cross-walk

| Acceptance criterion | Where enforced |
| --- | --- |
| Provider outages or offline work still produce typed draft packets with clear expiry and reconciliation posture instead of hidden unsent state. | Section 1 (offline-draft packet); section 1.2 (expiry-window block, stale-intent rules); the `expiry_window_block.expires_at` and `reconciliation_strategy_class` are required fields. |
| Reconciliation preserves origin, intended target, and authority context even when account or provider state has changed. | Section 1 (`origin_disclosure` / `intended_target_block`); section 2 (provider-reconciliation outcomes preserve the bound offline-draft packet's origin and target across `account_mismatch_no_apply`, `re_review_required_account_remap`, `superseded_by_remote_state`, and `missing_authority_no_apply` outcomes); section 2.1 (gates pinning the bound `linked_account_mapping_binding_record_id_ref` and the typed `missing_authority_class`). |
| Users can distinguish "safe to publish later" from "must re-review" without inspecting provider logs. | Section 1 (`reconciliation_strategy_class` 7-value vocabulary); section 1.2 (lifecycle gates pin typed `expired_re_review_required` / `blocked_by_conflict_pending_re_review` / `blocked_by_superseded_remote_state_pending_re_review` / `blocked_by_missing_authority_pending_user_action` / `blocked_by_account_mismatch_pending_user_action` lanes); section 2 (`reconciliation_outcome_class` 11-value vocabulary plus `next_safe_action_class` 19-value vocabulary). |
| Worked fixtures cover no-network draft, queued publish within expiry, expired queued action, and conflict between local draft and provider-updated state. | `/fixtures/work_items/offline_reconciliation_cases/` carries one fixture per case. |

## 9. Schema-of-record posture (frozen)

Rust types in the eventual work-items crate are the source of
truth. The JSON Schema exports at
`schemas/work_items/offline_draft_packet.schema.json` and
`schemas/work_items/provider_reconciliation.schema.json` are the
cross-tool boundaries every non-owning surface reads.

Adding a new record kind, `draft_origin_class`,
`local_change_kind_class`, `reconciliation_strategy_class`,
`draft_lifecycle_class`, `conflict_origin_class`,
`extension_admissibility_class`, `reconciliation_outcome_class`,
`missing_authority_class`, `next_safe_action_class`, denial reason,
or audit-event id is additive-minor and bumps the per-record
schema version. Repurposing an existing value is breaking and
requires a new decision row.

There is no external IDL or code-generator toolchain at this
revision; this mirrors the posture of the upstream contracts the
offline-draft and reconciliation packets cite by reference.
