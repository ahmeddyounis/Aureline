# Provider-mode, callback-envelope, and publish-later continuity contract

This document freezes how provider-linked surfaces in Aureline behave as
typed local objects with explicit draft / publish / handoff semantics
instead of as bespoke per-surface UI shortcuts. It binds together three
things the product must not invent twice:

1. the **mutation-mode** the user saw at the point of intent,
2. the **callback / event envelope** the provider returns through, and
3. the **publish-later continuity record** that keeps a provider-linked
   action alive across offline, outage, account-switch, status-preview,
   and queue-review transitions without losing origin, scope, or
   next-safe-action truth.

The machine-readable schemas live at:

- [`/schemas/providers/provider_callback_envelope.schema.json`](../../schemas/providers/provider_callback_envelope.schema.json)
  — `provider_callback_envelope_record`,
  `callback_dedup_key_record`,
  `import_session_record`,
  `imported_provider_snapshot_record`,
  `callback_deny_audit_event_record`.
- [`/schemas/providers/publish_later_record.schema.json`](../../schemas/providers/publish_later_record.schema.json)
  — `publish_later_queue_item_record`,
  `publish_later_queue_review_record`,
  `provider_consequence_preview_record`,
  `account_mapping_binding_record`,
  `provider_object_relation_record`.

Worked fixtures live at:

- [`/fixtures/providers/provider_mode_cases/`](../../fixtures/providers/provider_mode_cases/)

This contract **composes with and does not replace** the
provider-plane approval-ticket and browser-handoff contracts frozen in
[`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md).
Mutation modes, actor classes, destination classes, reason codes,
replay postures, grant-resolution reasons, audit-event ids, and
redaction classes all reuse the frozen vocabularies exported from
[`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
and
[`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json).
This contract extends them with the callback-envelope and
publish-later-queue shapes every provider-linked surface reads.

It also composes with the runtime authority-ticket and external-effect
lineage contract in
[`/docs/governance/runtime_authority_contract.md`](../governance/runtime_authority_contract.md),
the auth callback packet in
[`/docs/auth/system_browser_callback_packet.md`](../auth/system_browser_callback_packet.md)
(auth callbacks stay on that packet; provider callbacks land here),
and the object-specific issue handoff packet in
[`/docs/support/object_handoff_packet.md`](../support/object_handoff_packet.md)
(support exports cite publish-later refs and callback-envelope ids
without inlining provider payloads).

If this document disagrees with those sources, those sources win and
this document plus the schemas are updated in the same change.

This document does not ship a live callback listener, a live queue
drainer, or a provider-specific adapter. It freezes the contract those
implementations will read and write. The eventual provider-mode crate's
Rust types are the schema of record; the JSON Schema exports are the
cross-tool boundary every non-owning surface reads.

## Why freeze this now

The product has to answer the same six questions on every
provider-linked control on every surface that talks to a code host,
an issue / planning tracker, a CI / checks provider, a docs / portal
provider, or an artifact registry:

1. *Will this stay local, queue for later, require browser handoff,
   or mutate provider state immediately?*
2. *Under whose authority on the provider (human, install / bot,
   delegated, project-scoped, or policy-injected) is it proposing to
   act?*
3. *If the provider returns asynchronously, how will the callback be
   deduped, origin-validated, replay-gated, and bound back to the
   originating Aureline object?*
4. *If network is offline, the browser is blocked, or the account is
   switched, how does the pending action survive without silently
   becoming "already done" or "already lost"?*
5. *What imported provider state is currently rendered, and what is
   its source, freshness, actor scope, and trust posture?*
6. *What is the next safe action a queue-review, support export, or
   admin reconciliation consumer can take?*

Without one frozen contract: the code-host surface invents a
local-draft shape, the issues surface invents another, CI invents a
third, docs invent a fourth, and artifact registry invents a fifth.
Support exports see five incompatible pending-action shapes, the
callback listener invents its own dedup key space, and a single
"Publish" button on one surface means something different from a
"Publish" button on another.

This contract closes that gap with **one mutation-mode vocabulary,
one callback envelope, and one publish-later queue shape** that every
provider-linked surface and every post-incident consumer reads.

## Scope

Frozen at this revision:

- the five **mutation modes** disclosed at the point of intent, pinned
  to the frozen ADR-0010 vocabulary (`local_draft`, `publish_now`,
  `open_in_provider`, `deferred_publish`, `inspect_only`);
- the **provider-linked surface classes** the contract covers
  (code host, issue / planning tracker, CI / checks, docs / portal,
  artifact registry, release publisher, identity provider, AI
  provider, managed admin);
- the **callback / event envelope** shape every inbound provider
  callback or webhook is wrapped in before it touches local state;
- the **import-session record** shape every inbound inspect-only fetch
  is grouped under, so imported provider state carries source,
  freshness, actor scope, and trust posture everywhere it is rendered;
- the **publish-later queue-item record** that keeps a local-draft,
  deferred-publish, queued browser-handoff, or interrupted publish-now
  alive across offline, outage, account-switch, and status-preview
  transitions;
- the **queue-review**, **provider consequence preview**,
  **account-mapping**, and **offline-capture continuity** hooks queue
  reviewers and offline-recovery surfaces read;
- the **frozen relation set** between local draft, queued publish,
  browser handoff, provider-authoritative object, and cached
  read-only shadow;
- the **redaction posture** that keeps raw URLs, raw callback bodies,
  raw webhook bodies, raw delegated-token bodies, and raw provider
  payloads off this boundary.

## Out of scope

- Live provider integrations (the actual code-host, issues, CI, docs,
  registry, release-publisher, identity, AI, and managed-admin
  adapters). This document freezes the contract those adapters will
  satisfy.
- OAuth / SSO / device-code protocol profiles. Auth callbacks stay on
  the auth-callback packet; only *provider-object* callbacks land on
  this envelope.
- Webhook signature-verification libraries and provider-specific
  idempotency-key mapping. This document freezes the invariants
  (signature-required, origin-validated, dedup-keyed, replay-gated);
  each adapter implements them against its provider.
- The live queue-drain service, retry policy engine, and admin
  reconciliation console. The contract is the vocabulary those
  services will read and write.

## 1. Mutation modes on provider-linked surfaces

Every provider-linked surface MUST name exactly one of the five
frozen mutation modes on every control that could plausibly mutate
provider state. The mode is disclosed at the point of intent, not
after the fact, and is visible in headers, notifications, and
exports — not only on the originating button.

### 1.1 Frozen vocabulary (reused from ADR-0010)

| Mode               | Provider contact | User-visible meaning                                                                                 |
|--------------------|------------------|-------------------------------------------------------------------------------------------------------|
| `local_draft`      | none             | Stays local. Carries remote object identity, freshness floor, target scope, intended actor, conflict policy. |
| `publish_now`      | immediate        | Commits to the provider when the approval ticket spends. Names actor class and irreversibility.      |
| `open_in_provider` | browser handoff  | Routes through a typed `browser_handoff_packet`; the mutation happens on the provider, not locally.  |
| `deferred_publish` | queued           | Admitted into the publish-later queue; publishes when prerequisites, scope, freshness, and actor all align. |
| `inspect_only`     | read             | Reads provider state into a typed `imported_provider_snapshot_record`; never mutates.                |

### 1.2 Covered surface classes

The contract applies to every provider-linked surface in these
classes. Each class reuses the same five modes; surface-local shortcuts
are forbidden.

| Surface class              | Mutation modes it MAY render                                                                                            | Typical provider-side objects                             |
|----------------------------|-------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------|
| `code_host_surface`        | `local_draft`, `publish_now`, `open_in_provider`, `deferred_publish`, `inspect_only`                                    | pull requests, review comments, branches, merge actions   |
| `issue_or_planning_surface`| `local_draft`, `publish_now`, `open_in_provider`, `deferred_publish`, `inspect_only`                                    | issues, work items, comments, labels, relationships       |
| `ci_or_checks_surface`     | `open_in_provider`, `deferred_publish`, `inspect_only` (rerun requests queue through `deferred_publish` or `open_in_provider`) | check runs, pipeline runs, rerun requests                 |
| `docs_or_portal_surface`   | `local_draft`, `publish_now`, `open_in_provider`, `deferred_publish`, `inspect_only`                                    | docs pages, portal entries, docs feedback                 |
| `artifact_registry_surface`| `open_in_provider`, `deferred_publish`, `inspect_only` (publish rides release lane, not a raw button)                   | package versions, release artifacts, registry metadata    |
| `release_publisher_surface`| `open_in_provider`, `deferred_publish` (publish always flows through release evidence; immediate `publish_now` forbidden) | release publish requests, signing attestations            |
| `identity_provider_surface`| `open_in_provider`, `inspect_only`                                                                                      | consent flows, grant details                              |
| `ai_provider_surface`      | `local_draft`, `open_in_provider`, `inspect_only`                                                                       | conversation records, provider-side settings              |
| `managed_admin_surface`    | `open_in_provider`, `inspect_only`                                                                                      | admin consoles, policy / billing pages                    |

### 1.3 Rules (frozen)

1. A control whose effect could reach provider state MUST name its
   mutation mode on the control itself *and* reflect that mode in the
   surface header, notification line, and export row — not only on
   the button label.
2. `publish_now` and `local_draft` MAY NOT collapse into one control;
   mode is disclosed at the point of intent.
3. `open_in_provider` MUST route through a typed
   `browser_handoff_packet` (ADR-0010). A raw URL launch from a
   provider-linked control on a protected surface is forbidden.
4. `deferred_publish` MUST mint a `publish_later_queue_item_record`
   with a stable queue id; the queue item is revocable and inspectable
   before drain.
5. `inspect_only` fetches MUST wrap their result in an
   `imported_provider_snapshot_record` (below); imported state is
   never re-rendered as locally authored.
6. A surface that cannot name a mutation mode MUST route to
   `inspect_only` rather than default to `publish_now`.
7. The same mutation mode means the same thing on every surface. A
   surface MAY narrow (admin policy may forbid a mode); no surface
   MAY widen, redefine, or rename a mode.

## 2. Callback / event envelope

Every inbound provider callback, webhook, or event delivery crosses
into Aureline through a typed `provider_callback_envelope_record`.
Raw URLs, raw callback bodies, raw webhook bodies, and raw delegated
tokens never cross this boundary; the envelope carries opaque refs,
structured fields, and signature / origin fingerprints only.

### 2.1 Required fields

Every `provider_callback_envelope_record` names:

- `envelope_id` — opaque, stable, safe to log.
- `callback_envelope_schema_version` — pinned integer.
- `callback_class` — one of `provider_event`, `provider_webhook`,
  `provider_callback_return`, `import_session_refresh`,
  `publish_later_drain_result`, `provider_side_retract`. (Auth-flow
  return callbacks stay on the auth-callback packet.)
- `connected_provider_record_id` — opaque ref to the provider
  registry row the envelope was received under.
- `originating_packet_ref` — opaque ref to the originating
  `browser_handoff_packet_record`, `publish_later_queue_item_record`,
  `import_session_record`, or provider-event subscription. Empty only
  for unsolicited webhook classes, which MUST still name their
  subscription id.
- `dedup_key` — the frozen `callback_dedup_key_record` fingerprint
  (section 2.2) used to recognise redelivery.
- `return_anchor` — the originating Aureline object the envelope
  returns to (reuses the ADR-0010 `return_anchor` shape: review
  anchor, issue draft anchor, selected diff anchor, docs citation
  anchor, object link anchor).
- `actor_scope` — the provider-actor class the delivering provider
  is acting as on this envelope. Reuses the ADR-0010 actor-class
  vocabulary (`human_account`, `installation_or_app_grant`,
  `delegated_user_token`, `project_scoped_grant`,
  `policy_injected_service_identity`, `unknown_actor_class`).
- `replay_posture` — one of `single_use`, `bounded_reuse`,
  `read_only_resumable`. Inherited from the originating packet when
  present; defaulted to `single_use` for fresh webhook deliveries.
- `freshness` — structured freshness block (observed-at timestamp,
  freshness floor ref, staleness class: `fresh`, `bounded_stale`,
  `unbounded_stale`).
- `trust_posture` — `trusted` or `restricted` (ADR-0001 workspace
  trust state the envelope is being accepted under).
- `origin_disclosure` — canonical host identity, workspace id, actor
  subject, execution-context id, policy epoch on the envelope. A
  callback whose origin disclosure does not verify is denied with
  `browser_handoff_origin_mismatch`.
- `intent_signature_status` — one of `verified`, `unsigned_expected`,
  `signature_missing`, `signature_invalid`. Only `verified` or
  `unsigned_expected` (for subscription classes that contractually
  carry no signature) progress past envelope validation.
- `policy_context` — policy epoch, trust state, execution-context id
  at validation time. A policy-epoch roll between issue and
  validation re-evaluates the envelope; continued silent use is
  forbidden.
- `redaction_class` — declared redaction class (reuses ADR-0010
  `metadata_safe_default`, `operator_only_restricted`,
  `internal_support_restricted`, `signing_evidence_only`).
- `received_at` — monotonic timestamp.

### 2.2 Dedup key

The envelope carries a `callback_dedup_key_record` describing the
fingerprint the listener uses to recognise redelivery. Redelivery
refreshes freshness markers; it does **not** duplicate side effects
or re-bind a new queue item.

Fields:

- `dedup_key_id` — opaque stable id.
- `dedup_scope_class` — one of `provider_delivery_id`,
  `provider_event_id`, `packet_correlator`, `publish_later_queue_id`,
  `import_session_id`.
- `scope_ref` — opaque reference to the scoping object.
- `observed_delivery_count` — integer, bounded.
- `first_observed_at` / `last_observed_at` — monotonic timestamps.

Dedup is authoritative: two envelopes with the same dedup key are the
same delivery. The listener MUST refuse to mutate local state twice
for the same dedup key; the second delivery updates freshness only.

### 2.3 Replay policy

Every envelope inherits replay posture from its originating packet
when present. The listener enforces:

- `single_use` — the envelope is consumed on first validation. A
  second validation against the same correlator is denied with
  `browser_handoff_replay_invalid`.
- `bounded_reuse` — validation permitted up to the bounded-reuse
  counter; the counter is recorded on the audit stream; exceeding the
  counter is denied.
- `read_only_resumable` — the envelope carries no mutation authority.
  It may refresh an `imported_provider_snapshot_record` but MAY NOT
  mutate queued items, browser handoffs, or provider-authoritative
  objects.

### 2.4 Deny / audit events

When an envelope fails validation, the listener emits a
`callback_deny_audit_event_record` on the `provider_handoff` audit
stream. The deny record carries the envelope id (never the body), the
connected-provider id, the actor class the envelope claimed, the
replay posture, the policy epoch, the trust state, and a typed
denial reason from the ADR-0010 set:

- `browser_handoff_replay_invalid`
- `browser_handoff_origin_mismatch`
- `browser_handoff_revoked`
- `origin_signature_invalid`
- `denied_scope_missing`
- `denied_policy_bundle`
- `denied_workspace_trust`
- `denied_actor_class_forbidden`
- `denied_target_conflict`
- `denied_freshness_floor`
- `denied_revoked`
- `denied_suspended`
- `denied_host_mismatch`
- `denied_unreachable`
- `denied_unknown_actor_class`

Silent retry is forbidden; silent downgrade is forbidden. A denied
envelope is visible on the originating object's state row (queue
review, support export, admin reconciliation) with the typed reason
and a repair hook.

### 2.5 Import-session record

An `inspect_only` fetch wraps its result in an
`imported_provider_snapshot_record` grouped under an
`import_session_record`. The session record names:

- `import_session_id` — opaque stable id.
- `connected_provider_record_id` — the provider registry row.
- `actor_scope` — provider-actor class used for the fetch.
- `trust_posture` — `trusted` or `restricted` at fetch time.
- `freshness` — fetched-at timestamp, freshness-floor ref, staleness
  class, partial-vs-full class (`full_snapshot`,
  `bounded_partial_snapshot`, `unbounded_partial_snapshot`).
- `inbound_envelope_refs` — envelopes that refreshed the session.
- `redaction_class` — declared redaction posture.

Every imported snapshot (code-host PR, issue, check run, docs page,
artifact version, and so on) MUST carry an
`import_session_record_ref` and render its source, freshness,
actor scope, and trust posture everywhere it appears — including
support exports, queue-review rows, and admin reconciliation. A
stale or partial import MUST visibly degrade; it MAY NOT masquerade
as authoritative provider truth.

## 3. Publish-later continuity

A `publish_later_queue_item_record` keeps a provider-linked action
alive across:

- network offline / provider outage,
- browser-handoff blocked (managed workstation, no system browser),
- status-preview (the user is inspecting consequences but has not yet
  confirmed),
- account switch (the actor class or subject changes between queue
  and drain),
- freshness floor drift (remote object moved past the queued
  freshness floor),
- policy-epoch roll (policy bundle moved between queue and drain),
- rollback (the local draft was rolled back after queueing).

### 3.1 Required fields

- `queue_item_id` — opaque, stable.
- `publish_later_schema_version` — pinned integer.
- `mutation_mode` — the mode the user confirmed. Reuses the ADR-0010
  vocabulary; `publish_now` items are admitted into the queue only
  when the drain is blocked (offline, browser blocked, provider
  unavailable) — a `publish_now` item in the queue MUST be
  visibly distinct from an intentionally-deferred `deferred_publish`
  item.
- `mode_admission_reason` — one of `user_deferred`, `offline_capture`,
  `provider_unavailable`, `browser_handoff_blocked`,
  `account_mapping_pending`, `approval_ticket_pending`,
  `freshness_floor_pending`, `policy_epoch_pending`,
  `step_up_pending`. Every item names exactly one.
- `connected_provider_record_id` — opaque ref.
- `surface_class` — surface that minted the item (from section 1.2).
- `target_object_identity` — `object_class`, `provider_side_id`,
  `provider_host`, `tenant_or_org_scope` (reuses ADR-0010
  `object_identity`).
- `originating_local_draft_ref` — opaque ref to the local draft
  object. Required for `local_draft` and `deferred_publish` modes.
- `originating_browser_handoff_packet_ref` — opaque ref when the item
  was queued because a browser handoff is blocked or pending.
- `originating_approval_ticket_ref` — opaque ref to the admitting
  approval ticket. A queued item carries the *request* for a drain
  ticket; the drain ticket is minted fresh at drain time with a
  `ticket_lineage` entry naming the queued request.
- `pending_prerequisites` — ordered array of typed prerequisite
  records (section 3.2). The drain MUST refuse while any required
  prerequisite is `unmet`.
- `conflict_policy_class` — one of `refuse_on_remote_change`,
  `merge_if_auto_resolvable`, `user_decides_on_drain`,
  `force_overwrite_with_preview`. `force_overwrite_with_preview`
  requires a preview ref.
- `consequence_preview_ref` — opaque ref to the
  `provider_consequence_preview_record` (section 4). Required when
  the item's `mutation_mode` is `publish_now`, `deferred_publish`,
  or `open_in_provider` for an irreversible or release-class action.
- `account_mapping_binding_ref` — opaque ref to the
  `account_mapping_binding_record` (section 5) the item was queued
  under. Required whenever the item was queued before the actor
  class / subject was fully resolved.
- `reopen_semantics` — one of `on_connectivity_restored`,
  `on_account_reselected`, `on_freshness_refresh`,
  `on_policy_epoch_stable`, `on_browser_available`,
  `on_user_reconfirm`. Every item names at least one; multiple are
  allowed for compound prerequisites.
- `queue_state` — one of `draft_in_queue`, `pending_prerequisites`,
  `awaiting_browser_handoff`, `awaiting_account_reselection`,
  `awaiting_consequence_confirm`, `drain_admitted`, `drained`,
  `superseded`, `revoked`, `rejected_at_drain`, `rolled_back`. The
  queue-review surface renders this state explicitly.
- `origin_disclosure` — host identity, workspace id, actor subject,
  execution-context id, policy epoch at queue time. Revalidated at
  drain.
- `policy_context` — policy epoch, trust state, execution-context
  id at queue time. Revalidated at drain.
- `redaction_class` — declared redaction posture.
- `queued_at` — monotonic timestamp.
- `expires_at` — monotonic timestamp. An expired item is rejected
  with `denied_freshness_floor` or `denied_approval_ticket_expired`
  (whichever applies) rather than drained silently.
- `last_review_ref` — opaque ref to the latest
  `publish_later_queue_review_record` (section 3.3).
- `supersedes_queue_item_refs` — queue items that this one
  supersedes (account remapping, action reframing).

### 3.2 Pending-prerequisite classes

The `pending_prerequisites` array carries typed records, each naming:

- `prerequisite_class` — one of `connectivity_restored`,
  `provider_health_recovered`, `browser_available`,
  `account_mapping_resolved`, `approval_ticket_issued`,
  `freshness_floor_satisfied`, `policy_epoch_stable`,
  `step_up_authenticator_cleared`, `upstream_object_refreshed`,
  `user_reconfirm_required`, `local_draft_preview_confirmed`,
  `conflict_resolved`.
- `prerequisite_state` — `unmet`, `met`, or `waived`.
- `rationale_summary` — short reviewable sentence a queue reviewer
  or support exporter can read.
- `linked_record_ref` — opaque ref to the record that will clear the
  prerequisite (health probe record, account mapping binding,
  approval-ticket record, preview record).
- `cleared_at` — monotonic timestamp, populated when
  `prerequisite_state = met`.

Drain authority is revalidated at drain: if *any* required
prerequisite has regressed from `met` to `unmet`, the drain fails
closed with a typed denial reason.

### 3.3 Queue-review record

A `publish_later_queue_review_record` snapshots a human or admin
review pass over the queue. It names:

- `review_id` — opaque stable id.
- `queue_item_refs` — queue items included in the review.
- `reviewer_actor_class` — one of the ADR-0010 actor classes, plus
  `admin_reviewer` for admin console reviews.
- `review_disposition_class` — one of `admit_for_drain`,
  `hold_for_prerequisites`, `revoke_item`, `replace_with_superseding_item`,
  `escalate_for_admin_review`.
- `notes_summary` — reviewable sentence (never raw provider body).
- `policy_context`, `origin_disclosure`, `redaction_class`,
  `reviewed_at` — same meanings as elsewhere in the contract.

Queue reviews are recorded on the `provider_handoff` audit stream
(`provider_action_deferred`, `deferred_publish_queue_drained`,
`deferred_publish_queue_rejected`) with the review id as metadata.

## 4. Provider consequence preview

A `provider_consequence_preview_record` is the user-confirmed
consequence snapshot a queue item, a `publish_now` ticket, or an
`open_in_provider` packet commits to. It is the same preview shape
the approval ticket's `preview_ref` points at for provider-plane
actions.

Fields:

- `preview_id` — opaque stable id.
- `target_object_identity` — reuses ADR-0010 `object_identity`.
- `predicted_side_effect_class` — one of the ADR-0010 side-effect
  classes (`inspect_only`, `local_reversible_edit`,
  `local_destructive_edit`, `credential_handle_projection`,
  `privileged_inspection_attach`, `external_reversible_comment`,
  `external_irreversible_publish`, `policy_or_trust_mutation`,
  `capability_widening`, `automation_admission_only`).
- `irreversibility_class` — `reversible`, `soft_reversible`,
  `irreversible`, `irreversible_release_class`.
- `projected_diff_summary` — short reviewable sentence describing
  what the provider will see change (never raw diff body).
- `projected_scope_summary` — short reviewable sentence describing
  which provider objects are affected.
- `expected_actor_class` — ADR-0010 actor class the user confirmed
  against.
- `freshness_floor_ref` — opaque ref to the freshness floor the
  preview was computed against. A queued preview whose freshness
  floor has drifted is invalidated at drain.
- `preview_hash` — content hash over the structured fields so
  tampering between preview and spend is detectable. Never a hash
  of a raw provider body.
- `redaction_class` — declared redaction posture.
- `captured_at` — monotonic timestamp.

A provider-linked action whose `predicted_side_effect_class` is
`external_irreversible_publish`, `policy_or_trust_mutation`,
`capability_widening`, or `local_destructive_edit` MUST carry a
preview ref. Queue-review surfaces render the preview summary
verbatim; the user who confirms the drain MUST see the same preview
the queue item was admitted under.

## 5. Account-mapping continuity

Account mapping is the bind between the local actor subject the user
selected, the provider-actor class that will be used on the provider,
and the connected-provider record the action runs against. When a
queue item is admitted before that bind is fully resolved (for
example, the user has not picked which of two linked human accounts
to publish under), the item carries a pending
`account_mapping_binding_record`:

- `account_mapping_id` — opaque stable id.
- `connected_provider_record_id` — opaque ref.
- `resolved_actor_subject` — may be empty string until resolved.
- `resolved_actor_class` — one of the ADR-0010 actor classes, or
  `unknown_actor_class` until resolved.
- `resolution_state_class` — one of `resolved`,
  `pending_user_selection`, `pending_account_link`,
  `pending_policy_review`, `stale_after_account_switch`,
  `denied_actor_class_forbidden`.
- `candidate_actor_subject_refs` — array of opaque refs to
  candidate `linked_account_refs` / `linked_install_refs` /
  `linked_delegated_credential_refs` on the connected-provider record.
- `last_user_visible_at` — monotonic timestamp the user last saw the
  mapping.
- `redaction_class` — declared redaction posture.

An account switch after queue time MUST transition the mapping to
`stale_after_account_switch`, park the queue item in state
`awaiting_account_reselection`, and mark the
`account_mapping_pending` prerequisite `unmet`. Drain is refused
until the user reselects; silent resolution under a different actor
class is forbidden.

## 6. Offline-capture continuity

Offline capture means the user took an action that would normally
hit the provider, but the product detected unavailability (network
down, browser blocked, managed-sync paused, provider health
`degraded` / `unavailable` / `revoked` / `suspended` / `expired`)
and converted the action into a local-first capture instead of
reporting it as "done".

An offline-capture queue item reuses
`publish_later_queue_item_record` with:

- `mode_admission_reason` — `offline_capture`,
  `provider_unavailable`, or `browser_handoff_blocked`;
- `queue_state` — `awaiting_browser_handoff`,
  `pending_prerequisites`, or `draft_in_queue`;
- `reopen_semantics` — at least one of `on_connectivity_restored`,
  `on_browser_available`, `on_policy_epoch_stable`;
- `pending_prerequisites` — at minimum
  `connectivity_restored` or `provider_health_recovered`; if the
  action class requires a preview, also `local_draft_preview_confirmed`.

Offline-capture items MUST be visible on the originating surface's
header and on the `publish_later_queue_review_record` rollup, with
the same mutation mode the user originally confirmed and the same
target object identity. They MUST NOT silently upgrade to
`publish_now` when connectivity returns; the drain runs through the
normal approval-ticket path with a typed
`deferred_publish_drain` action class.

## 7. Provider-object relations (frozen)

Provider-linked work MUST render the relation between the local
object and the provider-side object using exactly one of five frozen
relation classes. A `provider_object_relation_record` carries the
pair.

| Relation class                      | Local side                                     | Provider side                              | Mutation-mode posture                                                                                |
|-------------------------------------|------------------------------------------------|--------------------------------------------|------------------------------------------------------------------------------------------------------|
| `local_draft`                       | authored locally, not yet sent                 | none                                       | `local_draft`. Provider does not see this object. May promote to `deferred_publish` or `publish_now`. |
| `queued_publish`                    | authored locally, admitted for drain           | pending creation or update                 | `deferred_publish` (or a parked `publish_now`). Drain runs through a fresh approval ticket.          |
| `browser_handoff`                   | originating anchor retained locally            | mutation performed in the browser          | `open_in_provider`. No local authoring once handed off; callback updates the cached shadow.          |
| `provider_authoritative_object`     | typed link to the provider object              | authoritative                              | `publish_now` or `deferred_publish` updates target this object; local changes never override silently.|
| `cached_read_only_shadow`           | `imported_provider_snapshot_record`            | authoritative                              | `inspect_only`. Shadow is visibly bounded by fetch time, actor scope, and trust posture.             |

Fields:

- `relation_id` — opaque stable id.
- `relation_class` — one of the five above.
- `local_ref` — opaque ref to the local side (draft, queue item,
  handoff packet, link record, or imported snapshot).
- `provider_side_ref` — opaque ref to the provider-side object when
  the relation has one; empty string for `local_draft`.
- `last_authoritative_fetch_ref` — opaque ref to the most recent
  import-session record that refreshed the relation. Required for
  `provider_authoritative_object` and `cached_read_only_shadow`.
- `drift_state_class` — one of `in_sync`, `remote_moved`,
  `local_moved`, `both_moved`, `provider_unreachable`,
  `actor_scope_changed`.
- `next_safe_action_class` — one of `view_only`,
  `refresh_before_edit`, `promote_local_draft_to_queue`,
  `resume_browser_handoff`, `review_queue_item`,
  `reselect_account`, `escalate_for_admin_review`. Queue-review,
  support-export, and admin-reconciliation consumers render this.
- `redaction_class` — declared redaction posture.

A surface MAY NOT render a relation it cannot place in this matrix;
it routes to `cached_read_only_shadow` as the safe default.

## 8. Redaction posture (frozen)

Every envelope, queue item, preview, mapping, and relation declares
a redaction class from the ADR-0010 set. Raw URLs, raw callback
bodies, raw webhook bodies, raw delegated-token bodies, raw provider
payloads, and raw preview bodies MUST NOT cross this boundary on any
surface. Exports, support bundles, mutation-journal entries,
evidence packets, replay captures, and AI context captures carry
opaque refs and structured fields only.

Narrowing is permitted: admin policy MAY remove `publish_now` from a
surface, forbid an actor class, pin a queue item to
`awaiting_account_reselection`, or raise a preview-required floor.
Widening beyond the frozen rules is forbidden.

## 9. Audit-event reuse

Every envelope-validated, envelope-denied, queue-drained, queue-rejected,
queue-revoked, review-conducted, and relation-transition event fires on
the ADR-0010 `provider_handoff` audit stream using the frozen event
ids already in
[`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json):

- `browser_handoff_callback_validated`
- `browser_handoff_callback_rejected`
- `browser_handoff_revoked`
- `provider_action_proposed`
- `provider_action_denied`
- `provider_action_deferred`
- `provider_action_published`
- `provider_action_rolled_back`
- `deferred_publish_queue_drained`
- `deferred_publish_queue_rejected`
- `policy_epoch_rolled_invalidations`

No new audit-event id is introduced by this contract. The envelope,
queue, and relation records are the *payload* those frozen events
reference.

## 10. Acceptance criteria cross-walk

| Acceptance criterion                                                                                      | Where enforced                                                                                                                                   |
|------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| Control cannot imply server mutation without naming which provider mode it uses.                          | Section 1.3 rules 1, 2, 5, 6; schema: `publish_later_queue_item_record.mutation_mode` required; schema: every queue item and relation names a mode. |
| Imported provider state carries source, freshness, actor scope, and trust posture across surfaces.        | Section 2.5 (`import_session_record`, `imported_provider_snapshot_record`); relation class `cached_read_only_shadow`.                            |
| Browser handoff, callback, and provider-draft flows share one packet vocabulary.                          | Sections 1–7 all reuse the ADR-0010 actor, destination, reason, replay, redaction, side-effect, actor-class, and grant-resolution vocabularies.  |
| Local draft, publish now, and browser-only modes remain visible in headers, notifications, and exports.   | Section 1.3 rule 1; section 3.1 `queue_state` and `mode_admission_reason` required; redaction posture section 8.                                 |
| Continuity records survive status preview, account switch, provider outage, and queued publish.           | Section 3 (prerequisites, conflict policy, reopen semantics); section 5 (account mapping); section 6 (offline capture).                           |
| User can tell whether a provider-linked action will remain local, queue for later, require browser handoff, or mutate immediately. | Sections 1.1, 1.3, 3.1, 7.                                                                                                                      |

## 11. Schema-of-record posture (frozen)

Rust types in the eventual provider-mode crate are the source of
truth. The JSON Schema exports at
`schemas/providers/provider_callback_envelope.schema.json` and
`schemas/providers/publish_later_record.schema.json` are the
cross-tool boundary every non-owning surface reads.

Adding a new callback class, a new prerequisite class, a new
resolution-state class, a new relation class, a new queue state, a
new admission reason, or a new next-safe-action class is
additive-minor and requires the relevant `*_schema_version` bump;
repurposing an existing value is breaking and requires a new
decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR 0007,
ADR 0008, ADR 0009, and ADR 0010.
