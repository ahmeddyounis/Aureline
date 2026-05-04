# Status-transition review packet, side-effect fanout, and consequence-disclosure contract

This document freezes the cross-tool **transition-review** packet
that every Aureline surface reads before a user (or AI assistant
pending user confirmation) commits a risky work-item transition.
The goal is to keep risky transitions — status changes, assignee
changes, provider comments, reopen / close flows, and any
automation or notification fanout they trigger — gated behind one
typed review packet that pins, per side-effect row:

- *what* will fan out (provider mutation, local metadata change,
  linked review update, notification emission, queued follow-on
  automation);
- *which target account / install grant / delegated token /
  browser-handoff session / managed-admin lane* receives the
  mutation;
- *which authority source* the row resolves under (provider-
  authoritative, local-authoritative, cached read-only shadow,
  imported evidence only, managed / policy admin authority,
  pending account mapping, local-only no-provider authority);
- *which publish mode* the row applies under (`local_draft`,
  `publish_now`, `open_in_provider`, `deferred_publish`,
  `inspect_only`);
- *which undo / rollback posture* the row carries (no undo
  required, undo within window, rollback via compensating action /
  revert state token / revoke-before-drain, rollback blocked under
  irreversibility / release class / imported-evidence-only, or
  rollback unknown until callback observation);
- *which offline / deferred handling* lane the row falls into
  (publish-now, queue, captured-offline, browser-handoff, blocked
  pending prerequisites, inspect-only what-if).

The machine-readable boundary is:

- [`/schemas/work_items/transition_review.schema.json`](../../schemas/work_items/transition_review.schema.json)
  — `transition_review_record` and
  `transition_review_audit_event_record`.

Worked fixtures (local-only transition with no provider mutation;
provider-authoritative status change with assignee notification
fanout; policy-blocked transition under restricted workspace trust;
linked-review transition that triggers a code-review request and a
build-or-check-run automation) live under
[`/fixtures/work_items/status_transition_cases/`](../../fixtures/work_items/status_transition_cases/).

This contract **composes with and does not replace** the upstream
contracts it cites:

- [`/docs/work_items/work_item_contract.md`](work_item_contract.md)
  and the schemas
  [`/schemas/work_items/work_item_detail.schema.json`](../../schemas/work_items/work_item_detail.schema.json)
  and
  [`/schemas/work_items/status_transition_packet.schema.json`](../../schemas/work_items/status_transition_packet.schema.json)
  — `work_item_detail_record` is the per-row header the review
  packet binds back to; `status_transition_packet_record` is the
  previewed mutation the apply path commits, and the review packet
  is the typed user-facing review of that mutation.
- [`/docs/work_items/change_intent_and_publish_preview_contract.md`](change_intent_and_publish_preview_contract.md)
  and the schemas
  [`/schemas/work_items/change_intent.schema.json`](../../schemas/work_items/change_intent.schema.json)
  and
  [`/schemas/work_items/external_publish_preview.schema.json`](../../schemas/work_items/external_publish_preview.schema.json)
  — `change_intent_record` is the rationale / target scope / linked
  review / approvals / validation evidence record the review packet
  forwards through `linked_change_intent_record_id_ref`;
  `external_publish_preview_record` is the typed before / after
  label record the review packet forwards through
  `linked_external_publish_preview_record_id_ref`.
- [`/docs/work_items/provider_object_and_traceability_contract.md`](provider_object_and_traceability_contract.md)
  and the schemas
  [`/schemas/work_items/provider_object.schema.json`](../../schemas/work_items/provider_object.schema.json)
  and
  [`/schemas/work_items/traceability_link.schema.json`](../../schemas/work_items/traceability_link.schema.json)
  — `provider_field_class` and `traceability_link_record` are
  re-used on linked-review-update fanout rows that cite a bound
  cross-object relation.
- [`/docs/providers/provider_mode_contract.md`](../providers/provider_mode_contract.md)
  and the schemas
  [`/schemas/providers/publish_later_record.schema.json`](../../schemas/providers/publish_later_record.schema.json),
  [`/schemas/providers/provider_callback_envelope.schema.json`](../../schemas/providers/provider_callback_envelope.schema.json)
  — `mutation_mode`, `irreversibility_class`,
  `provider_actor_class`, `provider_consequence_preview_record`,
  `publish_later_queue_item_record`,
  `account_mapping_binding_record`, and
  `provider_callback_envelope_record` are re-exported / cited by
  reference from there.
- [`/docs/integration/browser_handoff_contract.md`](../integration/browser_handoff_contract.md)
  / ADR-0010 — `browser_handoff_packet_record` and the typed
  approval-ticket lifecycle. Browser-handoff dispositions cite the
  packet by reference.
- ADR-0001 / ADR-0007 / ADR-0010 / ADR-0011 / ADR-0018 — workspace
  trust, secret-broker handle / raw-secret-forbidden boundary,
  browser-handoff and approval-ticket envelope, capability
  lifecycle / freshness / client-scope / redaction, and
  workspace-trust state.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — work-
  item, provider-mode, change-intent, transition-review, and
  publish-preview passages.
- `.t2/docs/Aureline_Technical_Design_Document.md` — transition-
  review packet, side-effect fanout row, and consequence-field
  passages.
- `.t2/docs/Aureline_PRD.md` — work-item MUST / SHOULD language
  for truthful disclosure of provider mutation scope, target
  account / context, deferred-publish window, durable change-
  intent visibility across local draft / review / publish, and
  forbidden silent fanout under generic success messaging.

If this contract disagrees with those sources, those sources win
and this document plus the schema and fixtures update in the same
change.

## Why freeze this now

Without one frozen review contract: the issue / planning surface
labels every transition button "Save" and renders one undifferentiated
"Saved!" toast on success — silently fanning out into provider
state on some clicks and staying local on others; the review
surface invents a "Comment" button whose preview never discloses
that posting will notify the linked review's authors and queue a
follow-on check-run automation; the change-stack panel invents a
"Publish" button whose history view collapses queued, browser-
handoff, captured-offline, and policy-blocked transitions into one
"Pending…" row; the support-handoff packet exports the transition
as free text that doesn't survive a re-import; and a release-publish
trigger lands without disclosing that the apply is irreversible
under release-class semantics.

Worse, when the transition is a multi-kind batch (a status change
that *also* posts a provider comment, *also* re-assigns the
work-item, *also* updates the linked review), the user has no single
place to see the typed fanout truth before confirming. The status-
transition packet captures the previewed mutation, the change-intent
record captures the *why*, the external-publish preview captures the
projected before / after labels; but no single record carries the
*per-row fanout truth* (provider mutation row + local metadata row
+ linked review update row + notification emission row + queued
follow-on automation row) so that a downstream review surface,
queue-review surface, support-export, or admin-reconciliation
console can disclose mechanically what will fan out.

This contract closes that gap with **one transition-review record**
that pins the typed transition trigger, the typed authorization
class, the typed disposition class, an ordered list of side-effect
fanout rows over the closed five-kind set, and an ordered list of
automation-followon disclosures.

## Scope

Frozen at this revision:

- one `transition_review_record` per reviewed transition over a
  work-item detail row — with the typed `transition_trigger_class`,
  `transition_review_authorization_class`,
  `transition_review_disposition_class`, ordered
  `side_effect_fanout_rows` list, ordered
  `automation_followon_disclosures` list, optional
  `rationale_summary` and `block_reason_summary`, lineage refs to
  the bound status-transition packet, change intent, external-
  publish preview, publish-later queue item, browser-handoff
  packet, offline-handoff packet, account-mapping binding, and
  superseded predecessor, plus the `origin_disclosure`,
  `policy_context`, `redaction_class`, `authored_at`, `expires_at`,
  and `withdrawn_at` blocks shared with the upstream work-items
  records;
- one closed audit-event vocabulary;
- one closed denial-reason vocabulary.

## Out of scope

- Implementing issue-provider adapters (GitHub Issues, Linear,
  Jira, Azure Boards, Asana, Pivotal, Shortcut, on-prem trackers).
  This contract freezes the review-time vocabulary those adapters
  read and write.
- Provider automations, notification delivery, or status-workflow
  engines themselves. The contract pins the typed disclosure of
  what *will* fan out; firing the automation, delivering the
  notification, and walking the workflow happen on the
  publish-later contract, the provider-callback envelope, and the
  bound automation packets.
- Live provider HTTP / OAuth / webhook / signature-verification
  protocols.
- Final user-facing copy. The schema freezes the typed vocabulary;
  copy lives in the design system.

## 1. Transition-review record

Every reviewed risky transition over a work item resolves through
one `transition_review_record`. The record carries:

- a `transition_trigger_class` from the twelve-value frozen
  vocabulary (`status_change_trigger`, `assignee_change_trigger`,
  `provider_comment_trigger`, `reopen_close_trigger`,
  `automation_or_notification_fanout_trigger`,
  `linked_review_or_change_object_trigger`,
  `validation_evidence_attach_or_detach_trigger`,
  `subscriber_or_watcher_change_trigger`,
  `blocking_relationship_change_trigger`,
  `rename_or_metadata_change_trigger`, `release_publish_trigger`,
  `multi_kind_batch_trigger`);
- a `transition_review_authorization_class` from the seven-value
  frozen vocabulary (`human_actor_self_authored_admissible`,
  `human_actor_with_review_link_admissible`,
  `human_actor_pending_required_approvals`,
  `ai_proposed_pending_user_confirmation`,
  `managed_admin_authored_admissible`,
  `policy_admin_authored_admissible`,
  `imported_evidence_only_not_authoring`);
- a `transition_review_disposition_class` from the fifteen-value
  frozen vocabulary (five admissible_* lanes pinning the typed
  apply path; eight blocked_* lanes pinning the typed denial path;
  `withdrawn_before_apply` and `superseded_by_transition_review_record`
  as terminal lifecycle lanes);
- ordered `side_effect_fanout_rows` over the closed five-kind
  vocabulary (`provider_mutation_fanout`,
  `local_metadata_change_fanout`, `linked_review_update_fanout`,
  `notification_emission_fanout`,
  `queued_followon_automation_fanout`);
- ordered `automation_followon_disclosures` (at least one entry —
  if no follow-on fires, the row pins
  `automation_kind_class = no_followon_automation` so reviewers see
  typed truth instead of an empty surface);
- lineage refs to the bound status-transition packet, change
  intent, external-publish preview, publish-later queue item,
  browser-handoff packet, offline-handoff packet, account-mapping
  binding, and superseded predecessor;
- the `origin_disclosure`, `policy_context`, and `redaction_class`
  blocks shared with the work-item detail / status-transition /
  change-intent / external-publish preview records.

The record never carries raw provider URLs, raw provider issue
bodies, raw comment bodies, raw delegated tokens, raw branch /
commit URLs, raw author identity strings, raw absolute paths, raw
notification payloads, or raw automation payloads. All identity
crosses the boundary as opaque refs and reviewable labels
(<= 1024 graphemes).

### 1.1 Side-effect fanout rows

Every row pins the typed fanout truth for one observable
consequence of the transition. The five row kinds are:

- **`provider_mutation_fanout`** — a typed provider-side mutation
  will fire on apply (the field overwrite, action invocation, or
  state token update that the bound external-publish preview
  projects). Forbidden under
  `target_account_class = local_only_no_provider_account` and
  under `publish_mode_class` ∈ {`local_draft`, `inspect_only`}.
- **`local_metadata_change_fanout`** — a local-only metadata
  change will be persisted with no provider call. Pinned to
  `publish_mode_class = local_draft`,
  `target_account_class = local_only_no_provider_account`,
  `authority_source_class` ∈ {`local_authoritative_source_no_provider_overlay`,
  `local_only_no_provider_authority`}, and
  `notification_side_effect_class = no_notification_locally_only`.
- **`linked_review_update_fanout`** — a bound review workspace,
  review pack, change object, patch stack, branch, worktree, or
  validation evidence record will be updated on apply. MUST cite a
  non-empty `linked_artifact_record_id_ref`; MAY cite a
  `linked_traceability_link_record_id_ref` when the relation
  crosses the work-item boundary.
- **`notification_emission_fanout`** — a typed notification will
  be emitted on apply (or the lane is "unknown until drain" /
  "blocked pending workspace trust" / "suppressed by user
  opt-out"). MUST resolve `notification_side_effect_class` outside
  `no_notification_locally_only`.
- **`queued_followon_automation_fanout`** — a follow-on automation
  (code-review request, build-or-check-run, release-workflow run,
  policy-evaluation run, scheduled task / cron, incident-workspace
  open, notification-digest compile, managed-admin review routine,
  validation-evidence capture routine) will be queued or fired.
  MUST cite a non-empty `linked_artifact_record_id_ref`, an
  `automation_kind_class` outside `no_followon_automation`, and an
  `automation_admissibility_class` outside
  `no_followon_automation_in_scope`.

Per-row consequence fields name the typed truth the apply path
resolves under:

| Field | Re-export source | What it discloses |
| --- | --- | --- |
| `target_account_class` | `external_publish_preview.schema.json` | Which connected-provider account / install grant / delegated token / browser-handoff session / managed-admin lane / pending-mapping / local-only target receives the mutation. |
| `authority_source_class` | this schema (frozen here) | Provider-authoritative / local-authoritative / overlay-synced / cached-read-only / imported-evidence-only / managed-admin / policy-admin / pending-account-mapping / local-only truth source the row resolves under. |
| `publish_mode_class` | `publish_later_record.schema.json` (`mutation_mode`) | `local_draft` / `publish_now` / `open_in_provider` / `deferred_publish` / `inspect_only`. |
| `undo_rollback_posture_class` | this schema (frozen here) | No-undo / undo-within-window / rollback-via-compensating-action / rollback-via-revert-state-token / rollback-via-revoke-before-drain / rollback-blocked-irreversible / rollback-blocked-release-class / rollback-unknown-pending-callback / rollback-blocked-imported-evidence-only. |
| `offline_deferred_handling_class` | this schema (frozen here) | not-offline-not-deferred / queued / captured-offline / routed-through-browser-handoff / blocked / inspect-only-what-if. |
| `irreversibility_class` | `publish_later_record.schema.json` | reversible / soft-reversible / irreversible / irreversible-release-class. |
| `notification_side_effect_class` | `status_transition_packet.schema.json` | Notification fan-out lane (or "unknown until drain" / "blocked pending workspace trust" / "suppressed by user opt-out"). |

### 1.2 Automation-followon disclosures

The packet carries an ordered, non-empty
`automation_followon_disclosures` list. If no follow-on automation
fires, the list MUST carry exactly one row with
`automation_kind_class = no_followon_automation` and
`automation_admissibility_class = no_followon_automation_in_scope`
so the reviewer sees the typed "no automation" truth instead of an
empty surface. Any non-`no_followon_automation` row MUST cite a
non-empty `linked_followon_automation_record_id_ref` to the bound
automation carrier (publish-later queue item, browser-handoff
packet, scheduled-task seed, incident-workspace packet, etc.).

### 1.3 Disposition gates

The schema's `allOf` gates pin the following invariants:

- `admissible_now_publish_now`,
  `admissible_via_queue_for_publish_later`,
  `admissible_via_browser_handoff_only`, and
  `admissible_local_draft_only` MUST cite a non-empty
  `linked_status_transition_packet_record_id_ref`.
- `admissible_via_queue_for_publish_later` MUST cite a non-empty
  `linked_publish_later_queue_item_record_id_ref`.
- `admissible_via_browser_handoff_only` MUST cite a non-empty
  `linked_browser_handoff_packet_ref`.
- `blocked_account_mapping_pending` MUST cite a non-empty
  `linked_account_mapping_binding_record_id_ref`.
- `admissible_local_draft_only` and
  `admissible_inspect_only_what_if` dispositions MUST forbid any
  `provider_mutation_fanout` row so a local-only / inspect-only
  review can never silently fan out into provider state. The
  denial reason
  `silent_provider_mutation_under_local_only_review_disposition_forbidden`
  exists to prevent this.
- `admissible_inspect_only_what_if` MUST forbid any
  `queued_followon_automation_fanout` row whose
  `automation_admissibility_class` resolves outside
  `no_followon_automation_in_scope`. The denial reason
  `silent_followon_automation_under_inspect_only_disposition_forbidden`
  exists to prevent an inspect-only review from silently queueing
  follow-on automation.
- `linked_review_or_change_object_trigger` MUST cite at least one
  `linked_review_update_fanout` row.
- `automation_or_notification_fanout_trigger` MUST cite at least
  one `queued_followon_automation_fanout` OR
  `notification_emission_fanout` row.
- `release_publish_trigger` MUST cite at least one row whose
  `irreversibility_class` is `irreversible_release_class`.
- `ai_proposed_pending_user_confirmation` authorization MUST
  resolve `transition_review_disposition_class` to
  `admissible_inspect_only_what_if` or
  `admissible_local_draft_only` until a human actor confirms; the
  denial reason
  `ai_proposed_review_must_be_confirmed_before_apply` is the gate.
- `imported_evidence_only_not_authoring` MUST resolve
  `transition_review_disposition_class` to
  `admissible_inspect_only_what_if` AND forbid any
  `provider_mutation_fanout` row; the denial reason
  `imported_evidence_only_authoring_must_resolve_to_inspect_only_disposition`
  is the gate.
- A restricted-trust `policy_context` MUST forbid
  `admissible_via_browser_handoff_only` on the same packet (a
  restricted-trust workspace cannot launch a browser-handoff
  packet).
- `withdrawn_before_apply` lifecycle MUST cite a non-empty
  `withdrawn_at`.
- `superseded_by_transition_review_record` lifecycle MUST cite a
  non-empty `supersedes_transition_review_record_id_ref`.
- Every `blocked_*` disposition MUST cite a non-empty
  `block_reason_summary` so a downstream surface renders typed
  denial copy instead of generic "something failed" text.

### 1.4 Per-row gates

The schema's per-row `allOf` gates pin the following invariants:

- `linked_review_update_fanout` rows MUST cite a non-empty
  `linked_artifact_record_id_ref`.
- `queued_followon_automation_fanout` rows MUST cite a non-empty
  `linked_artifact_record_id_ref`, an `automation_kind_class`
  outside `no_followon_automation`, and an
  `automation_admissibility_class` outside
  `no_followon_automation_in_scope`.
- `notification_emission_fanout` rows MUST resolve
  `notification_side_effect_class` outside
  `no_notification_locally_only`.
- `local_metadata_change_fanout` rows MUST resolve
  `publish_mode_class = local_draft`,
  `target_account_class = local_only_no_provider_account`,
  `authority_source_class` ∈ {`local_authoritative_source_no_provider_overlay`,
  `local_only_no_provider_authority`}, and
  `notification_side_effect_class = no_notification_locally_only`.
- `provider_mutation_fanout` rows MUST resolve
  `target_account_class` outside `local_only_no_provider_account`
  AND `publish_mode_class` outside `{local_draft, inspect_only}`.
- `irreversibility_class = irreversible_release_class` rows MUST
  resolve `undo_rollback_posture_class` to
  `rollback_blocked_release_class`.
- `irreversibility_class = irreversible` (non-release) rows MUST
  resolve `undo_rollback_posture_class` to
  `rollback_blocked_irreversible`.
- `publish_mode_class = deferred_publish` rows MUST resolve
  `offline_deferred_handling_class` outside
  {`not_offline_not_deferred`,
  `deferred_publish_inspect_only_what_if`}.
- `publish_mode_class = publish_now` rows MUST resolve
  `offline_deferred_handling_class = not_offline_not_deferred`.
- `publish_mode_class = inspect_only` rows MUST resolve
  `offline_deferred_handling_class = deferred_publish_inspect_only_what_if`
  AND `notification_side_effect_class = no_notification_locally_only`.
- `publish_mode_class = open_in_provider` rows MUST resolve
  `offline_deferred_handling_class = deferred_publish_routed_through_browser_handoff`.
- `authority_source_class = imported_evidence_only_source` rows
  MUST resolve `undo_rollback_posture_class = rollback_blocked_imported_evidence_only_no_provider_path`
  AND `publish_mode_class = inspect_only`.
- `authority_source_class = pending_account_mapping_authority_source`
  MUST resolve `target_account_class = account_mapping_binding_pending_user_resolution`.

## 2. Relation rules

The transition-review packet composes with the upstream relation
contracts:

- **Review packet ↔ work-item detail header.** Every review packet
  MUST cite a non-empty `work_item_detail_record_id_ref`. The
  packet never floats free of a work-item row.
- **Review packet ↔ status-transition packet.** Every admissible_*
  disposition outside `admissible_inspect_only_what_if` MUST cite
  a non-empty `linked_status_transition_packet_record_id_ref`.
  The status-transition packet is the previewed mutation the
  apply path commits; the review packet is the typed user-facing
  review of that mutation.
- **Review packet ↔ change intent.** The
  `linked_change_intent_record_id_ref` is the path from the review
  packet to the rationale / target scope / linked review /
  approvals / validation evidence record. Empty only when the
  review packet is rendered for a transient transition that has
  no authored intent (e.g. a managed-admin-minted reopen).
- **Review packet ↔ external-publish preview.** The
  `linked_external_publish_preview_record_id_ref` is the path to
  the projected before / after labels record. Empty for
  `admissible_local_draft_only` and
  `admissible_inspect_only_what_if` dispositions that have no
  provider preview.
- **Review packet ↔ publish-later queue item / browser-handoff
  packet / offline-handoff packet / account-mapping binding.** The
  matching `linked_*_record_id_ref` is required when the
  disposition or the per-row `offline_deferred_handling_class`
  pins the matching deferred lane.

## 3. Truthful escape hatches

Every degraded disposition exposes at least one truthful escape
hatch:

1. **Open externally** — `linked_browser_handoff_packet_ref`
   routes through a typed browser-handoff packet (ADR-0010); the
   user can open the work item in the system browser even when
   local write authority is blocked. `admissible_via_browser_handoff_only`
   is the typed disposition.
2. **Queue for later** — `linked_publish_later_queue_item_record_id_ref`
   admits the transition into the publish-later queue;
   `admissible_via_queue_for_publish_later` is the typed
   disposition. Per-row `offline_deferred_handling_class = deferred_publish_admitted_to_queue`.
3. **Capture offline** — `linked_offline_handoff_packet_record_id_ref`
   captures the transition under unavailability; per-row
   `offline_deferred_handling_class = deferred_publish_captured_offline_pending_drain`.
4. **Inspect-only what-if** — `admissible_inspect_only_what_if`
   pins a structured what-if review with no apply path. Per-row
   `publish_mode_class = inspect_only` and
   `offline_deferred_handling_class = deferred_publish_inspect_only_what_if`.
5. **Withdraw before apply** — `withdrawn_before_apply` lifecycle
   pins a typed terminal state with `withdrawn_at`.

## 4. Cross-cutting record relations

A transition's life across all work-items records is:

- the `work_item_detail_record` is the **per-row header**;
- the `change_intent_record` is the **rationale + scope + linked
  review + side effects + approvals + evidence** record;
- the `external_publish_preview_record` is the **previewed publish
  with target account / context, before / after labels, deferred
  consequences**;
- the `status_transition_packet_record` is the **previewed
  mutation the apply path commits**;
- the `transition_review_record` (this contract) is the **typed
  user-facing review packet** with per-row fanout truth and
  consequence fields, gated against silent fanout under generic
  success messaging;
- the `publish_later_queue_item_record`,
  `browser_handoff_packet_record`,
  `offline_handoff_packet_record`,
  `account_mapping_binding_record`, and
  `provider_callback_envelope_record` are the **apply-path
  carriers** the deferred / browser-handoff / captured-offline /
  pending-mapping disposition cites by reference.

The review packet survives across the lifecycle: dispositions
progress through `admissibility_re_evaluated` events when the
freshness floor, account mapping, or workspace trust state
changes; terminal lifecycle lanes (`committed_publish_now`,
`committed_local_draft_only`, `admitted_to_publish_later_queue`,
`routed_to_browser_handoff`, `captured_offline_pending_drain`,
`blocked_pending_prerequisites`, `withdrawn_before_apply`,
`superseded`) are recorded through the audit-event vocabulary.

## 5. Redaction posture (frozen)

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
`admissible_now_publish_now` from a work-item surface, forbid
`admissible_via_browser_handoff_only` on a restricted-trust
workspace, raise a freshness floor, pin the row to
`admissible_inspect_only_what_if`, or suppress the AI-proposed
review path entirely. Widening beyond the frozen rules is
forbidden.

## 6. Audit-event reuse

Local-only review-packet lifecycle events fire on the `work_items`
audit stream using the closed event ids:

- `transition_review_authored`
- `transition_review_pinned_to_status_transition_packet`
- `transition_review_admissibility_re_evaluated`
- `transition_review_required_approval_added`
- `transition_review_required_approval_granted`
- `transition_review_required_approval_denied`
- `transition_review_required_approval_revoked`
- `transition_review_committed_publish_now`
- `transition_review_committed_local_draft_only`
- `transition_review_admitted_to_publish_later_queue`
- `transition_review_routed_to_browser_handoff`
- `transition_review_captured_offline_pending_drain`
- `transition_review_blocked_pending_prerequisites`
- `transition_review_withdrawn_before_apply`
- `transition_review_superseded`
- `transition_review_audit_denial_emitted`

Provider-side events (callback validation, queue drain, queue
rejection, handoff revocation, provider-action published / deferred
/ denied / rolled back) stay on the ADR-0010 `provider_handoff`
audit stream. This contract introduces no new ids on that stream;
the transition-review packet is the *payload* those frozen events
reference.

## 7. Acceptance criteria cross-walk

| Acceptance criterion | Where enforced |
| --- | --- |
| Before committing a risky transition, Aureline can state what other objects or systems will change using one typed review packet. | Section 1 (transition-review record), `side_effect_fanout_rows` over the closed five-kind vocabulary; section 1.1 (per-row consequence fields); section 1.2 (automation-followon disclosures, minItems-1 floor). |
| Deferred, blocked, and provider-authoritative transitions remain distinguishable in UI history and exports. | Section 1 (`transition_review_disposition_class` 15-value vocabulary); section 6 (audit-event vocabulary, one event id per disposition outcome); section 1.3 (per-disposition `linked_*` ref gates). |
| No status change can hide its side effects behind generic success messaging. | Section 1.3 (disposition gates: `silent_provider_mutation_under_local_only_review_disposition_forbidden`, `silent_followon_automation_under_inspect_only_disposition_forbidden`, `generic_success_messaging_under_silent_fanout_forbidden`); section 1.1 (per-row fanout kind enforces typed disclosure for provider mutation, local metadata change, linked review update, notification emission, and queued follow-on automation). |
| Required consequence fields (target account, authority source, publish mode, undo / rollback posture, offline / deferred handling) are present on every row. | Section 1.1 (consequence field table); side-effect fanout row schema requires all five fields plus `irreversibility_class` and `notification_side_effect_class`. |

## 8. Schema-of-record posture (frozen)

Rust types in the eventual work-items crate are the source of
truth. The JSON Schema export at
`schemas/work_items/transition_review.schema.json` is the
cross-tool boundary every non-owning surface reads.

Adding a new record kind, `transition_trigger_class`,
`side_effect_fanout_kind_class`, `authority_source_class`,
`publish_mode_class`, `undo_rollback_posture_class`,
`offline_deferred_handling_class`, `automation_kind_class`,
`automation_admissibility_class`,
`transition_review_authorization_class`,
`transition_review_disposition_class`, denial reason, or audit-event
id is additive-minor and bumps the per-record
`transition_review_schema_version` const. Repurposing an existing
value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
revision; this mirrors the posture of the upstream contracts the
transition-review packet cites by reference.
