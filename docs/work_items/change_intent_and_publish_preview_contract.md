# Change-intent metadata and external-publish preview contract

This document freezes the cross-tool change-intent and external-publish
preview object model that every Aureline surface reads when a user (or
AI assistant pending user confirmation) proposes a change against a
provider-owned (or local-only) work item, and *before* any provider
mutation has actually fired.

The goal is to make provider-linked change workflows reviewable before
they mutate external state, so Aureline can explain, on every surface
that touches the change:

- *what* the user intends to change (rationale, target scope, expected
  side effects, required approvals, validation evidence references);
- *which review or change object* the change is linked to (as a typed
  relation, not a free-text note);
- *what would happen if the change were published* (per-provider-field
  before / after labels, per-provider-action notification fan-out,
  what stays local, what account / install grant / delegated token /
  browser-handoff session / managed-admin lane will receive the
  mutation);
- *what happens if the publish is deferred* (queued, captured offline,
  routed through a browser handoff, blocked pending prerequisites);
- *which queue item, browser-handoff packet, offline-handoff packet,
  account-mapping binding, status-transition packet, or callback
  envelope* the apply path will eventually walk.

The machine-readable boundaries are:

- [`/schemas/work_items/change_intent.schema.json`](../../schemas/work_items/change_intent.schema.json)
  — `change_intent_record` and `change_intent_audit_event_record`.
- [`/schemas/work_items/external_publish_preview.schema.json`](../../schemas/work_items/external_publish_preview.schema.json)
  — `external_publish_preview_record` and
  `external_publish_preview_audit_event_record`.

Worked fixtures (local-only draft change intent; publish-now external
publish preview; review-linked status-change change intent;
deferred-publish change intent routed through a browser handoff;
deferred-publish external publish preview pending account-mapping
resolution; release-class change intent pinned to a publish-later
external publish preview) live under
[`/fixtures/work_items/change_intent_cases/`](../../fixtures/work_items/change_intent_cases/).

This contract **composes with and does not replace** the upstream
contracts it cites:

- [`/docs/work_items/work_item_contract.md`](work_item_contract.md)
  and the schemas
  [`/schemas/work_items/work_item_detail.schema.json`](../../schemas/work_items/work_item_detail.schema.json),
  [`/schemas/work_items/status_transition_packet.schema.json`](../../schemas/work_items/status_transition_packet.schema.json),
  and
  [`/schemas/work_items/offline_handoff_packet.schema.json`](../../schemas/work_items/offline_handoff_packet.schema.json)
  — `work_item_detail_record` is the per-row header the change intent
  binds back to; `status_transition_packet_record` is the previewed
  mutation that commits the intent on apply;
  `offline_handoff_packet_record` is the captured-under-unavailability
  packet the intent's apply path attaches to when the provider was
  unreachable, the system browser was blocked, the workspace was
  restricted, the freshness floor was unsatisfied, the policy bundle
  had rolled, the account mapping was pending, the step-up
  authenticator was pending, or the user explicitly deferred the
  action.
- [`/docs/work_items/provider_object_and_traceability_contract.md`](provider_object_and_traceability_contract.md)
  and the schemas
  [`/schemas/work_items/provider_object.schema.json`](../../schemas/work_items/provider_object.schema.json)
  and
  [`/schemas/work_items/traceability_link.schema.json`](../../schemas/work_items/traceability_link.schema.json)
  — `provider_field_class` is re-exported verbatim onto the
  `affected_provider_field_classes` row on the change intent's
  expected-side-effect entries and onto the per-provider-field
  projected-change row on the external publish preview;
  `traceability_link_record` is the stable per-link record the
  intent's linked review / branch / change-object / validation-evidence
  references read against.
- [`/docs/providers/provider_mode_contract.md`](../providers/provider_mode_contract.md)
  and the schemas
  [`/schemas/providers/publish_later_record.schema.json`](../../schemas/providers/publish_later_record.schema.json),
  [`/schemas/providers/provider_callback_envelope.schema.json`](../../schemas/providers/provider_callback_envelope.schema.json)
  — `predicted_side_effect_class`, `irreversibility_class`,
  `mutation_mode`, `provider_actor_class`, and
  `provider_consequence_preview_record` are re-exported from there;
  the change intent's apply path mints `publish_later_queue_item_record`
  / `provider_consequence_preview_record` /
  `account_mapping_binding_record` /
  `provider_callback_envelope_record` through the publish-later
  contract.
- [`/docs/vcs/review_workspace_contract.md`](../vcs/review_workspace_contract.md),
  [`/docs/vcs/review_pack_contract.md`](../vcs/review_pack_contract.md),
  [`/docs/vcs/change_stack_contract.md`](../vcs/change_stack_contract.md)
  and the schemas under `/schemas/vcs/` — `review_workspace_record`,
  `review_pack_record`, `review_evaluation_result_record`,
  `review_anchor_record`, `change_object_record`, `patch_stack_record`,
  `branch_row`, `worktree_row`. The change intent cites these by
  reference for its linked-review and change-object relations.
- [`/docs/integration/browser_handoff_contract.md`](../integration/browser_handoff_contract.md)
  / ADR-0010 — `browser_handoff_packet_record` and the typed approval
  ticket lifecycle. The change intent and the external publish
  preview cite the browser-handoff packet by reference on the
  `routed_browser_handoff` lifecycle and the
  `publish_routed_through_browser_handoff_open_in_provider` publish
  timing.
- ADR-0001 / ADR-0007 / ADR-0010 / ADR-0011 / ADR-0018 — workspace
  trust, secret-broker handle / raw-secret-forbidden boundary,
  browser-handoff and approval-ticket envelope, capability lifecycle
  / freshness / client-scope / redaction, and workspace-trust state.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — work-item,
  provider-mode, change-intent, and publish-preview passages.
- `.t2/docs/Aureline_Technical_Design_Document.md` — change-intent
  metadata, publish-preview record, and deferred-publish lifecycle
  passages.
- `.t2/docs/Aureline_PRD.md` — work-item MUST/SHOULD language for
  truthful disclosure of provider mutation scope, target account /
  context, deferred-publish window, and durable change-intent
  visibility across local draft, review, and publish.

If this contract disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same change.

## Why freeze this now

Without one frozen contract: the issue / planning surface invents a
local-draft "edit" affordance whose label says "Save" but whose apply
path silently calls the provider; the review surface invents a "Comment"
button that looks identical whether it stays local or fans out to the
provider's notification list; the change-stack panel invents a
"Publish" button whose preview is a free-text rendering of the
intended diff with no typed before / after labels; the
admin-reconciliation console reconstructs the change history from the
publish-later queue alone, missing the *why* the user authored the
change in the first place; and the support-handoff packet exports the
review-link as a free-text note that doesn't survive a re-import.

Worse, when the change is deferred (queued, captured offline, routed
through a browser handoff), the user has no single place to see what
was committed to. The status-transition packet captures the previewed
mutation, the publish-later queue item captures the deferred apply,
the browser-handoff packet captures the launch, the offline-handoff
packet captures the capture; but no single record carries the
*rationale*, the *expected side effects*, the *required approvals*,
the *target account / context*, the *before / after labels*, and the
*deferred-publish consequences* together so the user, queue-review,
and support-export consumers can see one truthful preview.

This contract closes that gap with **one change-intent record** that
pins rationale, target scope, linked review, expected side effects,
required approvals, and validation-evidence references; and **one
external-publish preview record** that pins target account / context,
projected provider-field changes (with before / after labels),
projected provider-action changes (with notification fan-out), what
stays local, and what happens if publish is deferred.

## Scope

Frozen at this revision:

- one `change_intent_record` per authored change against a
  work-item detail row — with rationale, typed `target_scope_class`,
  typed `intent_origin_class`, typed `linked_review_relation_class`,
  ordered `expected_side_effects` list, ordered `required_approvals`
  list, ordered `validation_evidence_references` list, the typed
  `change_intent_lifecycle_class` (`local_only_draft_intent` →
  `proposed_pending_review` → `under_review_with_review_link` →
  `awaiting_publish_preview` → `pinned_to_publish_preview` →
  `applied_publish_now` / `admitted_publish_later_queue` /
  `routed_browser_handoff` / `captured_offline_pending_drain`, with
  `withdrawn_by_user` / `superseded_by_change_intent_record` /
  `archived_after_close` as terminal lanes), and lineage refs back
  to the bound external publish preview, status-transition packet,
  publish-later queue item, browser-handoff packet, offline-handoff
  packet, account-mapping binding, change object / patch stack,
  review workspace / pack / evaluation result / anchors, branch /
  worktree, and superseded predecessor;
- one `external_publish_preview_record` per previewed publish over a
  change-intent row — with the typed `object_target_block` (target
  account class, target actor subject, target actor class, connected
  provider, target object identity), the typed `publish_timing_class`,
  the projected-provider-field-change list (with before / after
  labels), the projected-provider-action-change list (with
  notification side effects and irreversibility class), the
  local-only-artifact list, the `deferred_publish_consequences` block
  (deferred publish class, notification truth class, freshness
  invalidation class, revoke-before-drain admissibility, deferred
  publish window closes-at), the preview hash, and lineage refs back
  to the bound provider-consequence preview, publish-later queue
  item, browser-handoff packet, offline-handoff packet, account-
  mapping binding, status-transition packet, release artifact, and
  observed callback envelopes;
- one closed audit-event vocabulary per record family;
- one closed denial-reason vocabulary per record family.

## Out of scope

- Implementing issue-provider adapters (GitHub Issues, Linear, Jira,
  Azure Boards, Asana, Pivotal, Shortcut, on-prem trackers). This
  document freezes the contract those adapters will satisfy.
- Live provider HTTP / OAuth / webhook / signature-verification
  protocols. Auth callbacks stay on the auth-callback packet;
  provider-object callbacks land on the provider-mode contract's
  `provider_callback_envelope_record`.
- The publish-later drain service, the offline-handoff queue
  drainer, the browser-handoff launcher, or the admin reconciliation
  console. The contract is the vocabulary those services read and
  write.
- Final user-facing copy. The schemas freeze the typed vocabulary;
  copy lives in the design system.
- Cross-tracker mirroring, dedup heuristics, or cross-provider
  identity reconciliation.
- Release-publish workflow execution; the contract carries the
  `release_artifact_publish` action class and the matching
  release-manager approval and release-class irreversibility gates,
  but the release-publish workflow itself lives elsewhere.

## 1. Change-intent record

Every authored change against a work item resolves through one
`change_intent_record`. The record carries:

- a `change_intent_lifecycle_class` from the twelve-value frozen
  vocabulary (`local_only_draft_intent`, `proposed_pending_review`,
  `under_review_with_review_link`, `awaiting_publish_preview`,
  `pinned_to_publish_preview`, `applied_publish_now`,
  `admitted_publish_later_queue`, `routed_browser_handoff`,
  `captured_offline_pending_drain`, `withdrawn_by_user`,
  `superseded_by_change_intent_record`, `archived_after_close`);
- an `intent_origin_class` from the seven-value frozen vocabulary
  (`human_authored_local_draft`,
  `ai_proposed_pending_user_confirmation`,
  `imported_from_change_object`, `imported_from_review_pack`,
  `imported_from_offline_handoff`, `imported_from_support_bundle`,
  `policy_admin_minted`);
- a `target_scope_class` from the ten-value frozen vocabulary
  (`local_workspace_only_no_provider_target`,
  `provider_object_field_mutation`,
  `provider_object_lifecycle_transition`,
  `provider_object_relation_change`,
  `provider_object_comment_or_subscriber_change`,
  `provider_object_validation_evidence_attach`,
  `release_artifact_publish`,
  `repository_branch_or_worktree_change`,
  `cross_object_blocking_relationship_change`,
  `multi_object_batch_intent`);
- a reviewable `rationale_summary` (<= 1024 graphemes);
- a `linked_review_relation_class` from the seven-value frozen
  vocabulary (re-uses the `linked_review_class` axis from the
  work-item detail header so the relation is the same axis the
  header reads);
- ordered `expected_side_effects`, `required_approvals`, and
  `validation_evidence_references` lists;
- a `next_safe_action_class` from the thirteen-value frozen
  vocabulary;
- lineage refs to the bound external publish preview,
  status-transition packet, publish-later queue item, browser-handoff
  packet, offline-handoff packet, account-mapping binding, change
  object / patch stack, review workspace / pack / evaluation result
  / anchors, branch / worktree, and superseded predecessor;
- the `origin_disclosure`, `policy_context`, and `redaction_class`
  blocks shared with the work-item detail / status-transition /
  external publish preview records.

The record never carries raw provider URLs, raw provider issue
bodies, raw comment bodies, raw delegated tokens, raw branch /
commit URLs, raw author identity strings, raw absolute paths, or
raw notification payloads. All identity crosses the boundary as
opaque refs and reviewable labels (<= 1024 graphemes).

### 1.1 Lifecycle invariants

The `change_intent_lifecycle_class` is monotonic forward through:

```
local_only_draft_intent
  -> proposed_pending_review
    -> under_review_with_review_link
      -> awaiting_publish_preview
        -> pinned_to_publish_preview
          -> applied_publish_now
          -> admitted_publish_later_queue
          -> routed_browser_handoff
          -> captured_offline_pending_drain
```

with `withdrawn_by_user`, `superseded_by_change_intent_record`, and
`archived_after_close` as terminal lanes admissible from any prior
state. The audit stream's lifecycle ids
(`change_intent_authored`, `change_intent_proposed_pending_review`,
`change_intent_review_link_attached`,
`change_intent_awaiting_publish_preview`,
`change_intent_pinned_to_publish_preview`,
`change_intent_applied_publish_now`,
`change_intent_admitted_to_publish_later_queue`,
`change_intent_routed_to_browser_handoff`,
`change_intent_captured_offline_pending_drain`,
`change_intent_withdrawn_by_user`, `change_intent_superseded`,
`change_intent_archived_after_close`) are the only typed
transitions; the denial reason
`change_intent_must_not_disappear_after_creation` forbids a surface
from dropping an authored intent without an explicit terminal
transition.

### 1.2 Apply-path lineage gates

The schema's `allOf` gates pin the following invariants:

- `pinned_to_publish_preview`, `applied_publish_now`,
  `admitted_publish_later_queue`, `routed_browser_handoff`, and
  `captured_offline_pending_drain` MUST cite a non-empty
  `linked_external_publish_preview_record_id_ref`.
- `applied_publish_now`, `admitted_publish_later_queue`,
  `routed_browser_handoff`, and `captured_offline_pending_drain` MUST
  cite a non-empty `linked_status_transition_packet_record_id_ref`.
- `admitted_publish_later_queue` MUST cite a non-empty
  `linked_publish_later_queue_item_record_id_ref`.
- `routed_browser_handoff` MUST cite a non-empty
  `linked_browser_handoff_packet_ref`.
- `captured_offline_pending_drain` MUST cite both a non-empty
  `linked_offline_handoff_packet_record_id_ref` AND a non-empty
  `linked_publish_later_queue_item_record_id_ref` (the offline
  carrier and the queue item the drain path will reopen).
- `under_review_with_review_link` MUST cite a
  `linked_review_relation_class` outside `no_linked_review`.
- `linked_review_workspace_local_truth_only` /
  `linked_review_workspace_with_provider_overlay` MUST cite a
  non-empty `linked_review_workspace_record_id_ref`.
- `linked_review_pack_evaluation` MUST cite a non-empty
  `linked_review_pack_record_id_ref` OR a non-empty
  `linked_review_evaluation_result_record_id_ref`.
- `linked_browser_handoff_review_token_only` MUST cite a non-empty
  `linked_browser_handoff_review_packet_ref`.
- `local_workspace_only_no_provider_target` target_scope MUST forbid
  every provider-bound apply ref (external publish preview,
  publish-later queue item, browser-handoff packet, offline-handoff
  packet); silent provider mutation under a local-only label is the
  failure mode the denial reason
  `silent_publish_under_local_only_target_scope_forbidden` exists to
  prevent.
- Any `expected_side_effects` entry whose `irreversibility_class` is
  `irreversible` or `irreversible_release_class` MUST cite a non-empty
  `linked_external_publish_preview_record_id_ref` so the user can
  never commit an irreversible side effect without a pinned preview.
- Any provider-bound `target_scope_class` (everything except
  `local_workspace_only_no_provider_target`) MUST cite at least one
  `required_approvals` entry whose `approval_role_class` is **not**
  `no_approval_required_local_only`.
- `release_artifact_publish` target_scope MUST cite at least one
  `required_approvals` entry whose `approval_role_class` is
  `release_manager`.
- `superseded_by_change_intent_record` lifecycle MUST cite a non-empty
  `superseded_by_change_intent_record_id_ref`.
- `withdrawn_by_user` lifecycle MUST cite a non-empty `withdrawn_at`.
- `archived_after_close` lifecycle MUST cite a non-empty
  `archived_at` AND resolve `next_safe_action_class` to
  `no_action_archived_after_close`.

### 1.3 Approval-state truth

The `approval_state_class` set
(`not_required`, `pending`, `granted`, `denied`,
`granted_then_revoked`) preserves the granular history of an
approval. `granted_then_revoked` is distinct from `denied` so users
see the reason a previously-admissible intent is now blocked. A
`granted_then_revoked` entry MUST cite an `approval_revoked_at`
timestamp; the revocation summary MAY name the typed reason.

### 1.4 AI-proposed intents

`intent_origin_class = ai_proposed_pending_user_confirmation` pins
an intent the AI assistant proposed and that has not yet been
confirmed by a human. The denial reason
`ai_proposed_intent_must_be_confirmed_before_apply` forbids the
lifecycle from advancing past `proposed_pending_review` without an
explicit confirmation event from a human actor. AI-proposed intents
MUST resolve every `expected_side_effects` entry to its honest
`predicted_side_effect_class` and `irreversibility_class`; the
"AI-friendly" relabel of an irreversible side effect as reversible
is forbidden.

## 2. External-publish preview record

Every previewed publish over a change-intent row resolves through
one `external_publish_preview_record`. The preview carries:

- a typed `object_target_block` naming the
  `target_account_class` (nine values:
  `connected_provider_account_resolved`,
  `connected_provider_install_or_app_grant`,
  `connected_provider_delegated_user_token`,
  `connected_provider_project_scoped_grant`,
  `connected_provider_policy_injected_service_identity`,
  `account_mapping_binding_pending_user_resolution`,
  `browser_handoff_account_session_only`,
  `managed_admin_only_account`,
  `local_only_no_provider_account`), the resolved actor subject ref,
  the resolved actor class, the connected provider record id ref,
  and the typed `target_object_identity` (re-exported);
- a `publish_timing_class` from the six-value frozen vocabulary
  (`publish_immediate_publish_now`,
  `publish_queued_for_publish_later_deferred`,
  `publish_routed_through_browser_handoff_open_in_provider`,
  `publish_captured_offline_pending_drain`,
  `publish_inspect_only_what_if`,
  `publish_blocked_pending_prerequisites`);
- the ordered `projected_provider_field_changes` list (typed
  `provider_field_class`, typed `projected_change_kind_class`,
  typed `irreversibility_class`, before / after labels);
- the ordered `projected_provider_action_changes` list (typed
  `provider_action_class`, typed `irreversibility_class`, typed
  `notification_side_effect_class`);
- the ordered `local_only_artifacts` list (what stays local);
- a typed `deferred_publish_consequences` block (typed
  `deferred_publish_class`, typed `notification_truth_class`, typed
  `freshness_invalidation_class`, `revoke_before_drain_admissible`,
  optional `deferred_publish_window_closes_at`);
- a `preview_hash` over the structured fields so tampering between
  preview and apply is detectable;
- lineage refs to the bound provider-consequence preview,
  publish-later queue item, browser-handoff packet, offline-handoff
  packet, account-mapping binding, status-transition packet, release
  artifact, and observed callback envelopes;
- the `origin_disclosure`, `policy_context`, `redaction_class`,
  `captured_at`, and `expires_at` blocks.

### 2.1 Target account / context truth

`target_account_class` is the only authoritative chip for "which
account / install / token / browser-handoff session / managed-admin
lane will receive this mutation". The chip MUST be rendered on
every surface that shows the preview (control label, surface header,
notification line, export row, queue-review row,
admin-reconciliation row). The seven `connected_provider_*`,
`browser_handoff_*`, and `managed_admin_*` lanes MUST cite a
non-empty `target_account_subject_ref` and a non-empty
`connected_provider_record_id_ref`; `local_only_no_provider_account`
MUST forbid both refs. The denial reason
`silent_publish_under_local_only_account_forbidden` exists to
forbid a preview from claiming `local_only_no_provider_account`
while citing any projected provider field or action change.

### 2.2 Affected fields and actions

The `projected_provider_field_changes` list pins per-field truth:
each entry names the typed `provider_field_class`, the typed
`projected_change_kind_class` (`field_value_overwrite`,
`field_value_append`, `field_value_redact`, `field_value_unset`,
`field_value_unchanged_local_only`, `field_value_pending_resolution`),
the typed `irreversibility_class`, and the before / after labels.
`field_value_overwrite` and `field_value_append` rows MUST cite both
a non-empty `provider_authoritative_value_label_after` AND a
non-empty `local_overlay_value_label`;
`field_value_unchanged_local_only` rows MUST cite a non-empty
`local_overlay_value_label` AND an empty
`provider_authoritative_value_label_after` so the user can see the
local edit that will *not* be published.

The `projected_provider_action_changes` list pins per-action truth:
each entry names the typed `provider_action_class` (mirrors the
`transition_kind_class` axis on the status-transition packet, with
`release_artifact_publish` added for release-class previews), the
typed `irreversibility_class`, and the typed
`notification_side_effect_class`.

`projected_provider_action_changes` entries with
`provider_action_class = release_artifact_publish` MUST cite a
non-empty `linked_release_artifact_record_id_ref` AND resolve the
entry's `irreversibility_class` to `irreversible_release_class`
through the matching `allOf` gate. The denial reason
`release_artifact_publish_must_resolve_irreversibility_to_release_class`
prevents a release-class preview from understating its
irreversibility.

### 2.3 What stays local

The `local_only_artifacts` list preserves the typed
`local_artifact_class` (`local_draft_overlay_field`,
`local_branch_or_worktree_state`, `local_review_workspace_state`,
`local_change_object_state`, `local_patch_stack_state`,
`local_validation_evidence_only`, `local_audit_journal_event`,
`local_assistant_evidence_capture`,
`local_settings_or_keybindings_change`,
`local_only_comment_or_note`) per entry plus an optional
`linked_local_record_id_ref` so the user can chase the local record
without guessing. Empty list is admissible (a publish-now preview
that publishes everything carries no local-only entries).

### 2.4 Deferred-publish consequences

The `deferred_publish_consequences` block pre-resolves what happens
if the publish is not immediate. The five-value
`deferred_publish_class` set
(`not_deferred_publish_immediate`,
`deferred_publish_admitted_to_queue`,
`deferred_publish_captured_offline_pending_drain`,
`deferred_publish_routed_through_browser_handoff`,
`deferred_publish_blocked_pending_prerequisites`,
`deferred_publish_inspect_only_what_if`) pairs with
`publish_timing_class` through `allOf` gates so the typed timing
and the typed deferred lane always agree. The
`notification_truth_class` set forces the safe-default
`notification_unknown_until_drain` for queue / browser-handoff
routes; `notify_immediately_on_publish` is admissible only when
`publish_timing_class = publish_immediate_publish_now`. The
`freshness_invalidation_class` set names the conditions under
which the preview is invalidated and MUST be re-confirmed before
the deferred apply path commits. The `revoke_before_drain_admissible`
boolean preserves whether the user can revoke the deferred publish
before drain.

### 2.5 Preview-hash truth

The `preview_hash` is a content hash over the structured fields so
tampering between preview and apply is detectable. The hash is
never a hash of a raw provider body; it is a hash of the typed
fields the user confirmed against. The publish-later queue item,
the browser-handoff packet, and the offline-handoff packet that
the apply path mints all cite the same preview hash through the
typed `preview_hash` field on the bound
`provider_consequence_preview_record`.

## 3. Relation rules

The change-intent record and the external-publish preview compose
with the upstream relation contracts:

- **Change intent ↔ work-item detail header.** Every change intent
  MUST cite a non-empty `work_item_detail_record_id_ref`. The
  intent never floats free of a work-item row.
- **Change intent ↔ linked review.** The
  `linked_review_relation_class` axis is the same axis the work-item
  detail header reads; the schema's `allOf` gates pin matching refs
  for each of the seven typed relation values. The relation
  survives export / support packets without being downgraded to a
  free-text note (the typed class IS the relation; the bound refs
  ARE the lineage; the `linked_review_workspace_record_id_ref`,
  `linked_review_pack_record_id_ref`,
  `linked_review_evaluation_result_record_id_ref`,
  `linked_review_anchor_record_id_refs`, and
  `linked_browser_handoff_review_packet_ref` are the typed
  lineage refs every export reader resolves).
- **Change intent ↔ branch / worktree / change object / patch
  stack.** The intent cites
  `linked_branch_local_locator_ref`,
  `linked_change_object_record_id_ref`, and
  `linked_patch_stack_record_id_ref` directly. The
  `repository_branch_or_worktree_change` target_scope class pins the
  intent against a branch / worktree change.
- **Change intent ↔ external publish preview.** The
  `linked_external_publish_preview_record_id_ref` is the only path
  from the intent to the preview; the preview's
  `change_intent_record_id_ref` is the only path back. The two
  records form a one-to-one (or many-previews-per-intent under
  supersession) lineage.
- **External publish preview ↔ provider-mode contract.** The
  preview cites
  `linked_provider_consequence_preview_record_id_ref`,
  `linked_publish_later_queue_item_record_id_ref`,
  `linked_browser_handoff_packet_ref`,
  `linked_offline_handoff_packet_record_id_ref`,
  `linked_account_mapping_binding_record_id_ref`,
  `linked_status_transition_packet_record_id_ref`,
  `linked_release_artifact_record_id_ref`, and
  `linked_provider_callback_envelope_record_id_refs`. The publish
  path itself (publish_now ticket, deferred_publish queue drain,
  open_in_provider browser-handoff launch, captured-offline drain)
  is recorded through the publish-later contract and the
  status-transition packet's apply-path audit ids; this contract
  introduces no new ids on the `provider_handoff` audit stream.
- **External publish preview ↔ queued / browser-handoff / offline
  publish path.** The publish path is pre-resolved through
  `publish_timing_class` on the preview and verified through the
  `allOf` gates that pin the matching ref:
  `publish_immediate_publish_now` MUST cite a
  `linked_provider_consequence_preview_record_id_ref`;
  `publish_queued_for_publish_later_deferred` MUST cite a
  `linked_publish_later_queue_item_record_id_ref`;
  `publish_routed_through_browser_handoff_open_in_provider` MUST
  cite a `linked_browser_handoff_packet_ref`;
  `publish_captured_offline_pending_drain` MUST cite both a
  `linked_offline_handoff_packet_record_id_ref` AND a
  `linked_publish_later_queue_item_record_id_ref`;
  `account_mapping_binding_pending_user_resolution` target_account
  MUST cite a `linked_account_mapping_binding_record_id_ref`.

## 4. Truthful escape hatches

Every degraded example in the fixture corpus exposes at least one
truthful escape hatch. The contract names the four canonical
hatches through the typed vocabularies:

1. **Open externally** — `linked_browser_handoff_packet_ref` on the
   external publish preview routes through a browser-handoff packet
   (ADR-0010); the user can open the work item in the system
   browser even when local write authority is blocked.
   `linked_browser_handoff_review_packet_ref` on the change intent
   routes the linked review through a browser-handoff packet on
   restricted-trust workspaces.
2. **Copy summary** — every record carries reviewable labels
   (<= 1024 graphemes), a `rationale_summary` (intent), and a
   `summary` (intent + preview); surfaces expose
   `clipboard_or_text_export_user_initiated` from the offline-handoff
   `handoff_export_route_classes` set when an offline-handoff
   packet is bound through the `captured_offline_pending_drain`
   lifecycle.
3. **Export packet** — the bound offline-handoff packet's export
   route set names support-bundle, incident-workspace,
   object-handoff, companion-browser-handoff, clipboard / text,
   CLI export, and managed-admin external-handoff lanes. Every
   degraded fixture either resolves at least one export route or
   names `local_only_no_export_path` honestly through the bound
   offline-handoff packet.
4. **View sync diagnostics** — both record families carry an
   audit-event vocabulary (`change_intent_*`,
   `external_publish_preview_*`) that lets queue-review,
   support-export, and admin-reconciliation consumers render typed
   sync diagnostics rather than generic "something failed" copy.
   The five `external_publish_preview_invalidated_*` events
   (freshness drift, account remap, policy epoch roll, provider
   object split / merge, workspace trust downgrade) name the
   typed invalidation lane that re-confirmation MUST satisfy.

## 5. Cross-cutting record relations

A change's life across all work-items records is:

- the `work_item_detail_record` is the **per-row header**;
- the `change_intent_record` is the **rationale + scope + linked
  review + side effects + approvals + evidence** record;
- the `external_publish_preview_record` is the **previewed publish
  with target account / context, before / after labels, deferred
  consequences**;
- the `status_transition_packet_record` is the **previewed mutation
  the apply path commits** (one per intent-publish action);
- the `offline_handoff_packet_record` is the **captured-but-not-yet-
  applied mutation** (one per offline capture);
- the `publish_later_queue_item_record` is the **deferred-apply
  queue item** (one per deferred / captured-offline apply);
- the `browser_handoff_packet_record` is the **system-browser
  launch envelope** (one per `open_in_provider` apply);
- the `provider_consequence_preview_record` is the **provider-mode
  preview** the user originally confirmed against on the
  publish-later contract; the external publish preview cites it
  by reference rather than re-stating the projected diff.

The change intent and the external publish preview survive across
the lifecycle: the change intent's lifecycle progresses
monotonically forward (or to a terminal `withdrawn_by_user` /
`superseded_by_change_intent_record` / `archived_after_close`),
and the external publish preview is preserved in the audit stream
even after `external_publish_preview_revoked_before_drain` /
`external_publish_preview_invalidated_*` so the queue-review,
support-export, and admin-reconciliation consumers can render the
preview the user confirmed against rather than the preview the
provider eventually accepted.

## 6. Redaction posture (frozen)

Every record (change intent, external publish preview) declares a
`redaction_class` from the ADR-0010 / ADR-0007 set
(`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`). Raw
provider URLs, raw provider issue bodies, raw comment bodies, raw
label values that disclose customer / tenant identity, raw
delegated tokens, raw branch / commit URLs, raw author identity
strings, raw absolute paths, and raw notification payloads MUST
NOT cross this boundary on any surface regardless of class.
Exports, support bundles, mutation-journal entries, evidence
packets, replay captures, and AI context captures carry opaque
refs and structured fields only.

Narrowing is permitted: admin policy MAY remove
`publish_immediate_publish_now` from a work-item surface, forbid
an actor class on the bound account-mapping, raise a freshness
floor, pin the row to `cached_read_only_shadow_inspect_only`, or
suppress the AI-proposed intent path entirely. Widening beyond the
frozen rules is forbidden.

## 7. Audit-event reuse

Local-only change-intent and external-publish preview lifecycle
events fire on the `work_items` audit stream using the closed event
ids per record family:

- `change_intent_record`:
  `change_intent_authored`,
  `change_intent_proposed_pending_review`,
  `change_intent_review_link_attached`,
  `change_intent_review_link_detached`,
  `change_intent_validation_evidence_attached`,
  `change_intent_validation_evidence_detached`,
  `change_intent_required_approval_added`,
  `change_intent_required_approval_granted`,
  `change_intent_required_approval_denied`,
  `change_intent_required_approval_revoked`,
  `change_intent_awaiting_publish_preview`,
  `change_intent_pinned_to_publish_preview`,
  `change_intent_applied_publish_now`,
  `change_intent_admitted_to_publish_later_queue`,
  `change_intent_routed_to_browser_handoff`,
  `change_intent_captured_offline_pending_drain`,
  `change_intent_withdrawn_by_user`,
  `change_intent_superseded`,
  `change_intent_archived_after_close`,
  `change_intent_audit_denial_emitted`.
- `external_publish_preview_record`:
  `external_publish_preview_authored`,
  `external_publish_preview_pinned_to_change_intent`,
  `external_publish_preview_invalidated_freshness_drift`,
  `external_publish_preview_invalidated_account_remap`,
  `external_publish_preview_invalidated_policy_epoch_roll`,
  `external_publish_preview_invalidated_provider_object_split_or_merge`,
  `external_publish_preview_invalidated_workspace_trust_downgrade`,
  `external_publish_preview_revoked_before_drain`,
  `external_publish_preview_applied_publish_now`,
  `external_publish_preview_admitted_to_publish_later_queue`,
  `external_publish_preview_routed_to_browser_handoff`,
  `external_publish_preview_captured_offline_pending_drain`,
  `external_publish_preview_superseded`,
  `external_publish_preview_audit_denial_emitted`.

Provider-side events (callback validation, queue drain, queue
rejection, handoff revocation, provider-action published / deferred
/ denied / rolled back) stay on the ADR-0010 `provider_handoff`
audit stream. This contract introduces no new ids on that stream;
the change-intent and external-publish preview records are the
*payload* those frozen events reference.

## 8. Acceptance criteria cross-walk

| Acceptance criterion                                                                                                                                                  | Where enforced                                                                                                                                                                                                                                                              |
|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Every provider mutation preview states target account/context, affected fields, and whether the action is immediate, queued, or browser-mediated.                     | Section 2 (external publish preview), `object_target_block` (`target_account_class`, `target_account_subject_ref`, `target_actor_class`, `connected_provider_record_id_ref`, `target_object_identity`), `publish_timing_class` × `deferred_publish_class` matrix gates.     |
| Linked review relations survive export/support packets without being downgraded to free-text notes.                                                                   | Section 1 (change intent), `linked_review_relation_class` × typed-ref `allOf` gates; section 3 (relation rules).                                                                                                                                                            |
| Change intent remains visible across local draft, review, and publish steps rather than disappearing after initial creation.                                          | Section 1.1 (lifecycle invariants), denial reason `change_intent_must_not_disappear_after_creation`, audit-event vocabulary on every lifecycle transition.                                                                                                                  |
| Provider mutation under a local-only label is forbidden.                                                                                                              | Section 1.2 (apply-path lineage gates), denial reason `silent_publish_under_local_only_target_scope_forbidden` on the change intent; section 2.1 (target account / context truth), denial reason `silent_publish_under_local_only_account_forbidden` on the preview.        |
| Irreversible side effects (including release-class) are guarded by a pinned preview and a release-manager approval.                                                   | Section 1.2 (apply-path lineage gates), `irreversibility_class = irreversible / irreversible_release_class` requires a pinned preview; `release_artifact_publish` target_scope requires a `release_manager` approval; section 2.2 release-class preview gate.              |

## 9. Schema-of-record posture (frozen)

Rust types in the eventual work-items crate are the source of truth.
The JSON Schema exports at
`schemas/work_items/change_intent.schema.json` and
`schemas/work_items/external_publish_preview.schema.json` are the
cross-tool boundary every non-owning surface reads.

Adding a new record kind, lifecycle class, intent-origin class,
target-scope class, linked-review-relation class, expected-side-
effect-kind class, approval-role class, approval-state class,
validation-evidence-reference class, target-account class,
publish-timing class, projected-change-kind class, provider-action
class, local-artifact class, deferred-publish class,
notification-truth class, freshness-invalidation class, denial
reason, or audit-event id is additive-minor and bumps the per-record
`*_schema_version` const. Repurposing an existing value is breaking
and requires a new decision row.

There is no external IDL or code-generator toolchain at this
revision; this mirrors the posture of the upstream contracts the
change-intent and external-publish preview records cite by
reference.
