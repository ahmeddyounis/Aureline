# AI memory, embedding-store invalidation, delete / export, and offline-reconciliation contract

This document is the **product-wide contract** for how AI memory and
deferred AI state are classified, invalidated, deleted, exported, and
reconciled. It freezes one memory-object shape, one set of
invalidation triggers and outcomes, one delete-request shape, one
export-assembly shape, and one deferred-intent outbox / reconciliation
shape every AI-adjacent surface reads, so AI memory does not become
a hidden retention layer that violates local-first truth, delete
honesty, or offline behavior.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream AI / composer / branch-
agent / review / support / clear-data / export surface's mint of
its own memory copy, this document wins and the surface is non-
conforming.

The companion artifacts are:

- [`/schemas/ai/memory_object.schema.json`](../../schemas/ai/memory_object.schema.json)
  — boundary schema for the `ai_memory_object_record`,
  `ai_memory_invalidation_event_record`,
  `ai_memory_delete_request_record`,
  `ai_memory_export_assembly_record`, and
  `ai_memory_audit_event_record` shapes.
- [`/schemas/ai/deferred_intent_outbox.schema.json`](../../schemas/ai/deferred_intent_outbox.schema.json)
  — boundary schema for the `deferred_intent_record`,
  `deferred_intent_state_transition_record`,
  `deferred_intent_reconciliation_record`, and
  `deferred_intent_audit_event_record` shapes.
- [`/fixtures/ai/memory_delete_export_cases/`](../../fixtures/ai/memory_delete_export_cases/)
  — worked-example corpus covering delete-thread, delete-workspace-
  AI-state, embedder-change invalidation, evidence-packet override,
  per-class export, and replay-forbidden deferred-intent cases.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/ai/context_assembly_contract.md`](./context_assembly_contract.md) —
  context-assembly, segment, mention, attachment, route-plan,
  spend-plan, route-receipt, spend-receipt, tool-call lineage,
  evidence-packet, tainted-content-fence, and evidence-source-
  reference shapes every memory row and deferred intent quotes by
  id.
- [`/docs/ai/evidence_replayability_contract.md`](./evidence_replayability_contract.md) —
  retained-evidence-packet capture-class / capture-posture /
  omission-reason / replay-grade vocabulary. A
  `retained_evidence_packet_copy` memory row binds to the
  `ai_retained_evidence_packet` record-class (registry row
  authored on `record_class_governance.md`) and follows the
  evidence-packet contract's delete / export / hold posture
  rather than re-minting it.
- [`/docs/ai/provider_model_registry_contract.md`](./provider_model_registry_contract.md) —
  provider entry, model entry, local-model pack, and external-tool
  identity. Memory rows and deferred intents bind to these refs
  rather than re-mint them.
- [`/docs/ai/prompt_composer_contract.md`](./prompt_composer_contract.md) —
  prompt-pack and tool-pack manifests. The fingerprint kinds
  `prompt_pack_ref` and `tool_pack_ref` quote the manifest refs
  authored there.
- [`/docs/ai/model_graduation_and_budget_contract.md`](./model_graduation_and_budget_contract.md) —
  rollout-state pin, budget-routing policy, route-selection
  disclosure. A deferred intent's `rollout_state_ref` cites the
  model-rollout-state pin; rollout demotion / withdrawal forces
  reconciliation through `revalidate_full_environment_pin` or
  `replay_forbidden_against_changed_environment`.
- [`/docs/governance/record_class_governance.md`](../governance/record_class_governance.md)
  and `artifacts/governance/record_class_registry.yaml` — the
  `ai_retained_evidence_packet` row authoritatively governs
  retained evidence packet copies; this contract does not re-mint
  retention / hold / delete / export posture for those rows.
- [`/docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md)
  and `artifacts/runtime/storage_classes.yaml` — the storage-class
  authority vocabulary (`user_authored_durable_truth`,
  `user_owned_recovery_state`, `disposable_derived_cache`,
  `admin_or_control_artifact`) that AI memory rows narrow to but
  never widen.
- [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md) —
  delete-honesty, hold-class, and destruction-receipt vocabulary.
- [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md) —
  workspace-trust state, deployment profile, policy epoch.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md) —
  the broker-owned redaction pass.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md) —
  approval-ticket vocabulary; `revalidate_user_consent_required`
  and `revalidate_admin_approval_required` on a deferred intent
  pause it until a typed approval ticket is minted.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md) —
  `freshness_class`, `client_scope`, and `redaction_class`
  re-exported without modification.

If this document disagrees with those sources, those sources win
and this document plus the schemas are updated in the same change.

This document does not ship a live memory store or synchronization
engine. It freezes the row shape those implementations will read and
write. The eventual memory-runtime crate's Rust types are the schema
of record; the JSON Schema exports at
`schemas/ai/memory_object.schema.json` and
`schemas/ai/deferred_intent_outbox.schema.json` are the cross-tool
boundaries every non-owning surface reads.

## Why freeze this now

Without one frozen contract the product is free to invent a per-
surface notion of "AI history", a per-lane notion of "embedding
cache", a per-feature notion of "memory row", and a per-caller
notion of what to do with a queued AI action whose environment
shifted. Each divergence widens a different axis silently:

1. *A "Clear AI history" control deletes the user-visible thread
   and leaves the embedding rows, the retrieval cache, and the
   evidence-packet copies untouched.* Delete honesty fails: the
   user thinks state is gone but downstream AI surfaces still
   read it.
2. *An export packet bundles conversation text, embedding bytes,
   and retained evidence packets into one opaque "AI history"
   blob.* Reviewers cannot tell user-visible content apart from
   derived caches; redaction-class floors collide; the export
   ships secret-adjacent bytes into a sink that should never see
   them.
3. *A queued AI action sends after the user changed providers,
   the policy epoch rolled, the workspace lost trust, or the
   model was demoted.* Replay against a changed environment
   produces a result the original turn never authorised.
4. *An embedder version bump invalidates one half of the
   embedding rows but the cache surface still serves the stale
   half.* Provider routing and retrieval read drift between the
   embedding identity the rows were derived under and the
   embedding identity the live route uses.

This contract closes that gap with **one memory-object shape, one
invalidation-trigger / outcome vocabulary, one per-class delete /
export shape, and one deferred-intent reconciliation shape**.

## Who reads this document

- **AI / prompt-composer / retrieval / embedding / agent-runtime
  authors** minting memory rows and deferred intents at turn time.
- **Clear-data, export, support-bundle, admin offboarding, and
  legal-hold surface authors** reading memory rows to apply per-
  class delete / export posture.
- **Reconciliation / outbox / sync-engine authors** transitioning
  deferred intents through the eight-state reconciliation
  vocabulary.
- **Evidence / replay / claim-manifest authors** quoting the
  memory-object refs and deferred-intent refs on every retained
  evidence packet.

## 1. The AI memory-class matrix

### 1.1 Seven memory classes

Every AI-derived row that survives the active turn binds to exactly
one `ai_memory_class`:

| Memory class                            | What it holds                                                                | Storage authority                          |
|-----------------------------------------|------------------------------------------------------------------------------|--------------------------------------------|
| `turn_state_ephemeral`                  | In-flight scratch state for one active turn; never durably retained.          | (does not survive past turn end)           |
| `conversation_history_user_visible`     | Threaded user-visible record of past turns; export form a user reads.         | `user_authored_durable_truth` adjacent.    |
| `derived_cache_regeneratable`           | Retrieval / lookup cache (segment results, summaries) bound to identity refs. | `disposable_derived_cache`.                |
| `embedding_row_regeneratable`           | Vector / embedding row bound to a specific embedder identity.                 | `disposable_derived_cache`.                |
| `reusable_repo_fact_workspace_scope`    | Workspace-scoped derived fact (graph index, language snapshot summary).       | `disposable_derived_cache`.                |
| `retained_evidence_packet_copy`         | Local copy of an AI retained-evidence packet.                                 | `ai_retained_evidence_packet_class`.       |
| `explicit_saved_memory_user_owned`      | User-pinned "remember this" note authored explicitly by the user.             | `user_owned_recovery_state`.               |

A surface that collapses two memory classes into one row denies with
`ai_memory_class_collapse_forbidden`. The schema's `allOf` gates
enforce the storage-authority binding mechanically.

### 1.2 Why classes are separated

- **Conversation history is user content.** A user reads it, exports
  it, and deletes it as if it were any other authored note. The AI
  memory layer never re-authors it.
- **Derived caches and embeddings are regeneratable.** They are
  reproducible from primary sources (the workspace tree, the
  retrieved-document corpus, the conversation history). They MUST
  carry an `invalidation_fingerprint` so any change to the bound
  identity (model, embedder, prompt-pack, tool-pack, workspace,
  policy-epoch) forces re-derivation.
- **Reusable repo facts are workspace-scoped.** They survive across
  conversations but die when the workspace identity dies; deleting
  the workspace clears every reusable repo fact bound to it.
- **Retained evidence packets are governed by the
  `ai_retained_evidence_packet` record class.** The AI memory layer
  carries a copy reference and follows the packet's delete / export /
  hold posture; it does not invent a separate posture.
- **Explicit saved memory is user-pinned.** Only the user can unpin
  it; ordinary cache GC and policy retention cannot remove it.

### 1.3 Per-class default postures

| Memory class                            | Default delete posture                                  | Default export posture                | Default retention posture       |
|-----------------------------------------|---------------------------------------------------------|---------------------------------------|---------------------------------|
| `turn_state_ephemeral`                  | `delete_clears_local_only`                              | `not_exportable_local_authority_only` | `transient_session_only`        |
| `conversation_history_user_visible`     | `delete_request_supported_with_destruction_receipt`     | `exportable_as_user_visible`          | `conversation_scoped`           |
| `derived_cache_regeneratable`           | `delete_clears_local_only`                              | `not_exportable_regeneratable_only`   | `workspace_scoped`              |
| `embedding_row_regeneratable`           | `delete_clears_local_only`                              | `not_exportable_regeneratable_only`   | `workspace_scoped`              |
| `reusable_repo_fact_workspace_scope`    | `delete_clears_local_only`                              | `exportable_metadata_only`            | `workspace_scoped`              |
| `retained_evidence_packet_copy`         | `delete_denied_evidence_packet_overrides`               | `exportable_as_evidence_packet`       | `packet_expiry_or_case_close`   |
| `explicit_saved_memory_user_owned`      | `delete_request_supported_with_destruction_receipt`     | `exportable_as_user_visible`          | `user_pin_only`                 |

Admin policy MAY narrow these defaults (e.g. `delete_blocks_on_hold`
for any class under a legal hold). Admin policy MAY NOT widen them
(e.g. flipping a `derived_cache_regeneratable` row to
`exportable_as_evidence_packet`).

## 2. Invalidation rules

### 2.1 Sixteen invalidation triggers

A change that may make a derived / cached / embedding memory row
stale fires exactly one trigger from the closed
`invalidation_trigger_class` set:

- `model_identity_changed`
- `embedder_identity_changed`
- `prompt_pack_changed`
- `tool_pack_changed`
- `workspace_trust_changed`
- `workspace_identity_changed`
- `policy_epoch_rolled`
- `retention_policy_changed`
- `delete_request_received`
- `manual_admin_invalidation`
- `schema_version_bumped`
- `eligibility_revoked_pack_quarantined`
- `eligibility_revoked_provider_disabled`
- `eligibility_revoked_workspace_trust`
- `eligibility_revoked_policy`
- `rollout_state_demoted_or_withdrawn`

A surface that proceeds against a stale row without firing the
matching trigger denies with
`stale_memory_row_used_without_invalidation_fired`.

### 2.2 Six invalidation outcomes

Every fired trigger emits an
`ai_memory_invalidation_event_record` whose
`invalidation_outcome_class` is one of:

- `row_invalidated_and_purged` — the row is removed and a
  `destruction_receipt_ref` is minted.
- `row_invalidated_and_marked_for_regeneration` — the row is marked
  stale; the next read forces re-derivation.
- `row_quarantined_pending_review` — held pending an admin review;
  reads deny with `row_quarantined_pending_review`.
- `row_partially_invalidated_subsegment` — only a sub-range of the
  row was invalidated (e.g. one conversation turn).
- `row_blocked_on_hold_invalidation_deferred` — a legal / support
  hold blocks invalidation; the row stays present but is flagged.
- `row_left_intact_invalidation_not_required` — the trigger fired
  but no fingerprint matched; the row is unaffected.

`row_invalidated_and_purged` MUST cite a `destruction_receipt_ref`;
missing the ref denies through the schema's `allOf` gate.

### 2.3 Required fingerprints

Every `derived_cache_regeneratable`, `embedding_row_regeneratable`,
or `reusable_repo_fact_workspace_scope` row MUST carry at least one
`invalidation_fingerprint` from the closed
`invalidation_fingerprint_kind` set:

`model_identity_ref`, `embedder_identity_ref`, `prompt_pack_ref`,
`tool_pack_ref`, `workspace_identity_ref`, `workspace_trust_state`,
`policy_epoch_ref`, `producing_schema_version`,
`aureline_compatible_version_range`, `retention_policy_ref`,
`rollout_state_ref`.

A row missing every required fingerprint denies with
`derived_cache_missing_invalidation_fingerprint`.

### 2.4 Embedder change is the canonical case

When the embedder identity changes:

1. Every `embedding_row_regeneratable` row whose
   `invalidation_fingerprints` listed `embedder_identity_ref` for
   the prior identity fires `embedder_identity_changed`.
2. The matching outcome is
   `row_invalidated_and_marked_for_regeneration` for live workspaces
   (the next retrieval re-derives) or `row_invalidated_and_purged`
   for closed workspaces (the row is removed with a destruction
   receipt).
3. Any deferred intent that depended on those rows transitions to
   `queued_revalidate_before_send_required` (read-only intents) or
   `replay_forbidden_against_changed_environment` (mutating intents
   under `embedder_identity_changed_non_replayable`).

## 3. Delete and export rules

### 3.1 Delete distinguishes user-visible history, derived caches, and evidence packets

An `ai_memory_delete_request_record` MUST carry a
`request_outcome_per_class` set with one row per affected
`ai_memory_class`. Each row's `outcome_class` is one of:

`cleared`, `preserved_evidence_packet`, `preserved_user_pin`,
`preserved_legal_hold`, `preserved_admin_owned`,
`regenerated_after_clear`.

Every preserved outcome MUST cite a `preserved_reason_class` from
the closed seven-value set (`evidence_packet_overrides`,
`user_pin_only_user_can_unpin`, `active_legal_hold`,
`active_support_hold`, `admin_owned_class_immutable`,
`retention_minimum_window_open`, `regenerated_from_primary_source`).
Collapsing two classes into one row denies with
`ai_memory_class_collapse_forbidden`.

The user-facing flow then answers two questions verbatim from the
schema:

- **Which AI memory class is affected?** → the per-class rows in
  `request_outcome_per_class`.
- **Which classes are intentionally preserved or discarded?** → the
  `outcome_class` and `preserved_reason_class` on each row.

### 3.2 Export separates the same three families

An `ai_memory_export_assembly_record` MUST list:

- `included_classes` — one row per included `ai_memory_class`,
  carrying `export_posture_class`, a coarse `row_count_class`
  bucket, and the opaque `memory_object_refs`.
- `excluded_classes` — one row per intentionally-excluded class,
  citing one `exclusion_reason_class` from the closed six-value
  set (`regeneratable_excluded_by_design`,
  `secret_adjacent_excluded_with_disclosure`,
  `local_authority_only_not_in_managed_export`,
  `evidence_packet_export_separate_path`,
  `policy_redaction_class_exceeds_target_floor`,
  `no_rows_present`).

`export_class_mismatch_forbidden` denies an export that bundles
incompatible classes (e.g. `derived_cache_regeneratable` ridingunder
`exportable_as_evidence_packet`). `export_payload_raw_body_forbidden`
denies an export that would project raw conversation text, raw
prompt bodies, raw retrieved-document bodies, raw embeddings, raw
URLs, raw paths, or raw credential material across the boundary.

### 3.3 Five export targets

`export_target_class` is one of:

- `user_self_service_export` — the in-product "Export my AI data"
  flow; admits user-visible history, user-pinned saved memory, and
  evidence packets the user owns.
- `managed_offboarding_package` — the managed offboarding flow;
  admits everything the user-self-service path admits plus
  workspace-scoped reusable facts.
- `support_bundle_export` — admits metadata-only forms of derived
  caches and embedding rows for diagnostics; never raw bodies.
- `admin_legal_hold_export` — admits whatever the active legal hold
  requires under broker-redaction; never widens beyond hold scope.
- `evidence_packet_export` — the retained-evidence-packet export
  path governed by the `ai_retained_evidence_packet` record class.

### 3.4 Evidence packets override

A `retained_evidence_packet_copy` memory row's delete posture is
fixed at `delete_denied_evidence_packet_overrides` by the schema's
`allOf` gate. The packet's own delete posture (governed by the
`ai_retained_evidence_packet` record-class registry row) is the
authority. The AI memory layer never overrides an evidence packet's
hold or retention.

## 4. Deferred-intent and reconciliation states

### 4.1 Seven deferred-intent kinds

A queueable AI action is one of:

- `provider_call_invocation` — a buffered AI provider call.
- `tool_call_invocation` — a buffered external-tool / MCP
  invocation.
- `agent_step_invocation` — a paused step in an agent chain.
- `evidence_packet_dispatch` — a buffered dispatch of a retained
  evidence packet to a managed sink.
- `memory_row_invalidation_cascade` — a queued cascade of memory-
  row invalidations.
- `embedder_reindex_invocation` — a queued embedder re-derivation
  pass.
- `mutation_journal_replay_invocation` — a queued replay of a
  mutation-journal entry.

### 4.2 Eight reconciliation states

`current_state_class` is one of:

- `queued_pending_send`
- `queued_blocked_on_eligibility`
- `queued_blocked_on_hold`
- `queued_revalidate_before_send_required`
- `replay_forbidden_against_changed_environment`
- `fully_reconciled_completed`
- `fully_reconciled_dropped_with_typed_disclosure`
- `fully_reconciled_superseded_by_newer_intent`

A `queued_revalidate_before_send_required` intent MUST cite at least
one `revalidation_posture_class`; a
`replay_forbidden_against_changed_environment` intent MUST cite a
`replay_forbidden_reason_class`. Silent re-send is forbidden.

### 4.3 The bound environment pin

Every deferred intent records a `bound_environment_pin` at queue
time covering provider, model, prompt-pack, tool-pack, workspace
identity, workspace-trust state, policy-epoch, optional rollout-
state, and optional embedder identity.

A reconciliation pass compares each axis against the current
environment. Every axis that changed appears in the
`environment_axes_changed` set on the
`deferred_intent_reconciliation_record`. The closed eleven-value set
is:

- `provider_entry_changed`
- `model_entry_changed`
- `embedder_identity_changed`
- `prompt_pack_changed`
- `tool_pack_changed`
- `workspace_identity_changed`
- `workspace_trust_changed`
- `policy_epoch_rolled`
- `rollout_state_demoted_or_withdrawn`
- `depended_on_memory_invalidated`
- `no_axis_changed`

`no_axis_changed` is admissible only on
`sent_against_unchanged_environment`. Any other outcome MUST list
every changed axis or deny with
`silent_send_against_changed_environment_forbidden`.

### 4.4 Mutating actions narrow further

A `mutating_first_party_route` or `mutating_byok_route` deferred
intent whose `model_entry_changed`, `prompt_pack_changed`,
`tool_pack_changed`, or `workspace_trust_changed` axis fired denies
with `mutating_intent_replay_against_changed_binding_forbidden`
unless a fresh consent / approval ticket admits the new binding.
Read-only intents may proceed under `revalidate_provider_route_only`.
`agent_chained_invocation` always requires
`revalidate_full_environment_pin`; the schema's `allOf` gate
enforces this mechanically.

### 4.5 Ten reconciliation outcomes

`reconciliation_outcome_class` is one of:

`sent_against_unchanged_environment`,
`sent_against_revalidated_environment`,
`dropped_eligibility_revoked`,
`dropped_replay_forbidden`,
`dropped_superseded_by_newer_intent`,
`dropped_user_cancelled`,
`dropped_admin_cancelled`,
`dropped_consent_withdrawn`,
`dropped_retention_window_closed_before_send`,
`deferred_again_revalidation_pending`.

Every `dropped_*` outcome carries a `drop_reason_label`. Every
`sent_*` outcome carries the produced `route_receipt_ref`,
`evidence_packet_ref`, and (for mutating sends)
`mutation_journal_entry_ref`. Silent send / silent drop is forbidden.

## 5. Composition with other contracts

- **Evidence-replayability contract.** A retained-evidence-packet
  copy memory row's lifecycle is governed by the evidence-packet
  contract. The AI memory layer never overrides evidence-packet
  hold, retention, redaction, or export posture.
- **Provider / model registry contract.** The bound environment
  pin's `provider_entry_ref`, `model_entry_ref`, and embedder
  identity quote the registry rows. Changes to those rows
  (lifecycle demotion, withdrawal, quarantine) fire
  `eligibility_revoked_*` triggers.
- **Prompt-composer contract.** The fingerprint kinds
  `prompt_pack_ref` and `tool_pack_ref` quote the manifest refs
  authored on the composer contract. A pack version bump fires
  `prompt_pack_changed` or `tool_pack_changed`.
- **Graduation-packet / rollout-state contract.** A deferred
  intent's `rollout_state_ref` cites the model-rollout-state pin.
  Demotion to `deprecated` or withdrawal to `disabled` fires
  `rollout_state_demoted_or_withdrawn` and forces reconciliation
  through `revalidate_full_environment_pin` or
  `replay_forbidden_against_changed_environment`.
- **Record-class governance.** The
  `ai_retained_evidence_packet` record class is the authority for
  retained evidence packet copies. The AI memory layer carries the
  `originating_evidence_packet_ref` and follows the class.
- **Storage-class artifact.** The `storage_authority_class` axis on
  every memory row narrows but never widens the runtime storage-
  classes vocabulary
  (`artifacts/runtime/storage_classes.yaml`).
- **Background queue lane matrix.** Memory-row invalidation
  cascades and embedder reindex invocations ride the lanes named
  in `artifacts/runtime/queue_lane_matrix.yaml`; this contract
  does not re-author lane assignment.

## 6. Redaction posture

Every memory-object record, invalidation event, delete request,
export assembly, deferred intent, state transition, reconciliation
record, and audit event declares a `redaction_class` from the
ADR-0011 set (`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`).

Raw conversation text, raw prompt bodies, raw retrieved-document
bodies, raw tool return bytes, raw embedding vectors, raw URLs, raw
absolute paths, raw user identifiers, raw billing-account ids, raw
API keys, raw OAuth tokens, raw mTLS material, raw model weights,
raw pack bytes, and raw provider payloads never cross either
boundary on any surface. Exports, support bundles, mutation-journal
entries, evidence packets, replay captures, deferred-intent records,
and reconciliation records carry opaque refs and structured fields
only.

Narrowing is permitted: admin policy MAY remove a memory class, an
export-target, an invalidation outcome, or a reconciliation outcome
from a deployment profile. Widening beyond the frozen rules is
forbidden.

## 7. Audit-event reuse

Every memory-object mint, invalidation, purge, quarantine, delete-
request, export-assembly, and deferred-intent / state-transition /
reconciliation event fires on its dedicated audit stream:

**`ai_memory` stream:**

- `ai_memory_object_minted`
- `ai_memory_object_invalidated`
- `ai_memory_object_purged`
- `ai_memory_object_quarantined`
- `ai_memory_object_export_assembled`
- `ai_memory_object_delete_requested`
- `ai_memory_object_delete_completed`
- `ai_memory_object_delete_blocked_on_hold`
- `ai_memory_object_delete_denied_class_immutable`
- `ai_memory_object_schema_version_bumped`

**`ai_deferred_intent` stream:**

- `deferred_intent_minted`
- `deferred_intent_state_transitioned`
- `deferred_intent_revalidation_required`
- `deferred_intent_replay_forbidden`
- `deferred_intent_reconciled_sent`
- `deferred_intent_reconciled_dropped`
- `deferred_intent_superseded`
- `deferred_intent_audit_denial_emitted`
- `deferred_intent_outbox_schema_version_bumped`

No new audit-event id is introduced on the existing
`ai_provider_registry`, `ai_model_registry`, `ai_context`,
`ai_replay`, or `ai_graduation` streams; those streams keep their
existing ids.

## 8. Acceptance-criteria cross-walk

| Acceptance criterion                                                                                                                          | Where enforced                                                                                                                                                                                                                                                                                              |
|-----------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Deleting a thread or workspace AI state invalidates matching durable cache keys where feasible and leaves no unlabeled retained copies.       | Section 1.1 + 2.3 + 3.1. Schema: `invalidation_fingerprints` `minItems: 1` for derived / embedding / reusable rows; `request_outcome_per_class` covering every affected class; denial reason `derived_cache_missing_invalidation_fingerprint`.                                                              |
| Exports can distinguish user-visible history, derived cache material, and evidence packets with separate redaction and retention posture.     | Section 1.3 + 3.2. Schema: `ai_memory_export_assembly_record` `included_classes` per-class rows + `excluded_classes` per-class reason rows + `export_posture_class` per row; denial reasons `export_class_mismatch_forbidden`, `export_payload_raw_body_forbidden`.                                          |
| Deferred AI work cannot silently replay against a changed provider, workspace trust, or policy epoch.                                          | Section 4.3 + 4.4. Schema: `bound_environment_pin` required on every `deferred_intent_record`; `environment_axes_changed` required non-empty (other than `no_axis_changed`) when reconciliation outcome is anything but `sent_against_unchanged_environment`; denial reason `silent_send_against_changed_environment_forbidden`. Mutating-action narrowing through `mutating_intent_replay_against_changed_binding_forbidden`. |
| Delete/export flows can answer which AI memory class is affected and which classes are intentionally preserved or discarded.                  | Section 3.1 + 3.2. Schema: `request_outcome_per_class` per-class rows on `ai_memory_delete_request_record`; `included_classes` + `excluded_classes` per-class rows on `ai_memory_export_assembly_record`; per-class `outcome_class` and `preserved_reason_class`; `exclusion_reason_class` vocabulary.       |
| Storage classes normalize to the shared AI memory-class matrix so turn state, conversation threads, reusable repo facts, retained evidence copies, and explicit saved memory are not mixed implicitly. | Section 1.1. Schema: `ai_memory_class` enum (seven values); `storage_authority_class` allOf gates binding `derived_cache_regeneratable` / `embedding_row_regeneratable` to `disposable_derived_cache`, `retained_evidence_packet_copy` to `ai_retained_evidence_packet_class`, `explicit_saved_memory_user_owned` to `user_owned_recovery_state`. |

## 9. Schema-of-record posture

Rust types in the eventual memory-runtime crate are the source of
truth. The JSON Schema exports at
`schemas/ai/memory_object.schema.json` and
`schemas/ai/deferred_intent_outbox.schema.json` are the cross-tool
boundaries every non-owning surface reads.

Adding a new `ai_memory_class`, `storage_authority_class`,
`delete_posture_class`, `export_posture_class`,
`retention_posture_class`, `invalidation_trigger_class`,
`invalidation_outcome_class`, `invalidation_fingerprint_kind`,
`deferred_intent_kind`, `current_state_class`,
`revalidation_posture_class`, `deferred_action_class`,
`reconciliation_outcome_class`, `replay_forbidden_reason_class`,
`audit_event_id`, or `denial_reason` value is additive-minor and
requires a paired `ai_memory_object_schema_version` /
`deferred_intent_outbox_schema_version` bump; repurposing an
existing value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
revision; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR 0007,
ADR 0008, ADR 0009, ADR 0010, ADR 0011, the AI context-assembly
contract, the AI provider / model registry contract, the AI
prompt-composer contract, the AI evidence-replayability contract,
and the AI graduation-packet / rollout-state contract.

## 10. Out of scope at this revision

- Building the memory store. This contract freezes the row shape;
  the live store is not in this revision.
- Building the synchronization engine for the deferred-intent
  outbox. This contract freezes the row shape and reconciliation
  semantics; the live sync engine is not in this revision.
- Wire formats for memory rows in transit (RPC envelopes, CRDT
  encodings, replication transports).
- Long-term retention rules for memory rows beyond what the
  retention-posture vocabulary classifies. Concrete retention
  windows live on the eventual support / service retention rows.
- Concrete embedder runtime, retrieval index, or vector-store
  implementation. Each will read this contract; this contract does
  not pin one.
