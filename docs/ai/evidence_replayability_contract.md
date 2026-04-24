# AI evidence replayability, audit-storage, and retained-artifact review contract

This document is the **product-wide contract** for what an
AI-assisted action MUST capture, retain, omit-with-disclosure, or
discard so a reviewer can later explain or replay that action.
It defines a graded replayability posture (not an all-or-nothing
promise), names what may be intentionally omitted, and binds the
retained-artifact classes back to the record-class registry,
consent ledger, telemetry / support-export schema registry, and
exact-build identity rather than inventing a separate storage
vocabulary.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream AI / composer / branch-
agent / review / support surface's mint of its own replay copy,
this document wins and the surface is non-conforming.

The companion artifacts are:

- [`/schemas/ai/evidence_replay_packet.schema.json`](../../schemas/ai/evidence_replay_packet.schema.json)
  — boundary schema every non-owning surface reads for the
  `ai_replay_packet_record`,
  `ai_replay_capture_coverage_record`,
  `ai_replay_provider_availability_record`,
  `ai_replay_mutation_lineage_record`, and
  `ai_replay_audit_event_record` shapes.
- [`/schemas/ai/audit_storage_manifest.schema.json`](../../schemas/ai/audit_storage_manifest.schema.json)
  — boundary schema for the
  `ai_audit_storage_manifest_record`,
  `ai_audit_storage_artifact_record`, and
  `ai_audit_storage_audit_event_record` shapes.
- [`/fixtures/ai/replay_cases/`](../../fixtures/ai/replay_cases/)
  — worked-example corpus covering at least one full-replay
  case, one partial-replay case when the provider is
  unavailable, and one non-replayable case that still preserves
  evidence honesty.

This contract rides alongside — it does not re-mint — the
vocabularies already frozen in:

- [`docs/ai/context_assembly_contract.md`](./context_assembly_contract.md)
  — the assembly, segment, mention, attachment, route-plan,
  spend-plan, route-receipt, spend-receipt, tool-call lineage,
  branch-agent dispatch, evidence-packet, tainted-content-fence,
  and evidence-source-reference shapes every replay packet
  quotes by id.
- [`docs/governance/record_class_governance.md`](../governance/record_class_governance.md)
  and `artifacts/governance/record_class_registry.yaml` — the
  class-level retention, hold, delete, export, and offboarding
  posture every retained replay artifact binds to. The
  `ai_retained_evidence_packet` row is the authoritative class
  for retained AI evidence packets.
- [`docs/governance/telemetry_and_support_schema_registry.md`](../governance/telemetry_and_support_schema_registry.md)
  and `artifacts/governance/consent_ledger_seed.yaml` — the
  build-flavor / consent / endpoint-class posture every
  retained replay artifact quotes rather than re-deriving.
- [`docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — the broker-owned redaction pass; no replay packet may
  bypass it, and every export rewrite re-runs it.
- [`docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — `freshness_class`, `client_scope`, `redaction_class`
  re-exported without modification.
- The running-build identity pin from the exact-build identity
  contract (D-0011) — every replay packet names exactly one
  `running_build_identity_ref` so a later reviewer can tell
  which product build minted the original turn.

## Who reads this document

- **AI / prompt-composer / tool-adapter authors** minting the
  retained capture set at turn time.
- **Review / diff / support / evidence / parity-audit surfaces**
  reading a retained packet later to explain or replay the
  original turn.
- **Retention / offboarding / delete / export surfaces** binding
  retained-artifact behavior to the record-class registry rather
  than hard-coding per-surface lifetimes.
- **Claim-manifest / release-evidence tooling** citing replay
  coverage as evidence that an AI-assisted action can be
  explained after the fact.

## Replayability is graded, not binary

A replay packet MUST name exactly one `replay_grade` from the
frozen vocabulary below. Silent downgrade to "best-effort" is
non-conforming; every degraded state names its cause so the
reviewer reads the limitation rather than guessing at it.

### Frozen `replay_grade` vocabulary

- `full_replay`
- `partial_replay_provider_unavailable`
- `partial_replay_model_version_lost`
- `partial_replay_tool_unavailable`
- `partial_replay_context_narrowed_by_policy`
- `partial_replay_context_expired_by_retention`
- `partial_replay_redaction_widened`
- `partial_replay_mutation_already_applied`
- `non_replayable_mutation_lost`
- `non_replayable_context_lost`
- `non_replayable_raw_byte_dependent`
- `non_replayable_cross_session_branch`
- `non_replayable_consent_withdrawn`

Rules (frozen):

1. `full_replay` is reachable only when every required
   capture class below was captured, the provider identity and
   model identity are reachable at the graded posture
   `reachable_same_identity_same_version`, every tool call the
   turn invoked is re-invokable, and no retained context
   segment has since been narrowed or expired. Downstream
   determinism is still provider-bounded; the packet names the
   provider_class and cost_visibility_class so reviewers know
   what "same output" means.
2. `partial_replay_*` grades admit at least one missing input
   while every other capture class remains available. The
   packet names the degraded axis and the
   `degraded_capture_class` that forced the grade.
3. `non_replayable_*` grades admit that reconstructing the
   original output is not possible; the packet still names
   every captured reference so the reviewer has **evidence
   honesty** — what is known, what was omitted, and why.
4. Any unresolved replay grade denies with
   `replay_grade_unresolved`; a surface that guesses a grade
   from partial data is non-conforming.
5. A replay packet for a `support_replay_only`
   `dispatch_target_class` never dispatches; its grade is
   assessed against the retained artifact set alone.

## Capture-class contract

A replay packet names, per `capture_class`, whether the class
was `required_captured`, `required_captured_reference_only`,
`required_omitted_with_disclosure`,
`required_missing_deny`, `optional_captured`,
`optional_omitted_without_loss`, or `not_applicable`. The packet
MUST address each class separately — collapsing them into one
"captured" boolean is non-conforming.

### Frozen `capture_class` vocabulary

Required axes (every replay packet reads each of these):

- `input_user_turn_prompt`
- `composer_plan_directive`
- `system_scaffold_fragment`
- `instruction_bundle`
- `check_bundle`
- `context_segment_set`
- `mention_set`
- `attachment_set`
- `tool_call_lineage`
- `tool_call_return_value`
- `route_plan`
- `spend_plan`
- `route_receipt`
- `spend_receipt`
- `provider_identity`
- `model_identity`
- `approval_ticket`
- `mutation_journal_entry`
- `save_manifest`
- `interaction_safety_packet`
- `downstream_handoff`
- `evidence_packet`
- `tainted_content_fence`
- `running_build_identity`
- `policy_context`

### Rules (frozen)

1. Every `capture_class` row names exactly one `capture_posture`
   value from the closed set above.
2. `required_captured` and `required_captured_reference_only`
   differ only in whether the retained object is a raw-byte
   capture (forbidden on every boundary for prompt text,
   document bodies, log bodies, terminal bodies, generated
   artifact bytes, request / response payloads, user-supplied
   text, credential material, raw paths, and raw URLs) or a
   typed reference into the context-assembly / evidence-packet
   record set. Reference-only is the default for every raw-body
   source class.
3. `required_omitted_with_disclosure` names a capture class the
   contract **refuses to store raw** — the packet names an
   `omission_reason` and MUST NOT silently substitute a raw
   capture.
4. `required_missing_deny` names a capture class the packet
   MUST have carried but did not; this denies the packet with
   `required_capture_missing`. A surface that downgrades the
   grade without emitting the denial is non-conforming.
5. `optional_captured` and `optional_omitted_without_loss`
   cover axes the contract does not require on every packet
   (e.g. `mutation_journal_entry` when the turn produced no
   mutation); the replay grade MUST NOT be affected by an
   optional axis being absent.
6. `not_applicable` is reachable only when the capture class
   cannot apply (e.g. `spend_receipt` for a
   `support_replay_only` reconstruction, `approval_ticket` on
   an inline read-only turn).

### Frozen `omission_reason` vocabulary

- `raw_body_forbidden_on_boundary`
- `raw_url_forbidden_on_boundary`
- `raw_prompt_text_forbidden_on_boundary`
- `raw_credential_material_forbidden`
- `raw_user_supplied_text_forbidden_unless_separately_retained`
- `retention_window_expired`
- `user_deleted_local_copy`
- `managed_copy_withdrawn_by_policy`
- `redaction_class_exceeds_sink`
- `policy_quarantined_source_body_forbidden`
- `provider_did_not_disclose`
- `never_captured_by_design`

Rules (frozen):

1. Every `required_omitted_with_disclosure` and
   `optional_omitted_without_loss` capture row names exactly
   one `omission_reason`.
2. `never_captured_by_design` is the reason every raw body
   (prompt text, document body, terminal body, log body,
   generated artifact bytes, request / response payloads,
   user-supplied text, credential material, raw paths, raw
   URLs) is marked; it is not a bug, it is the contract.
3. `retention_window_expired` and
   `user_deleted_local_copy` fire when the
   `ai_audit_storage_manifest_record` for the artifact has
   advanced into a terminal retention / delete state; the
   replay packet quotes the manifest by id so downstream
   reviewers see why the axis is absent.
4. `managed_copy_withdrawn_by_policy` fires when an admin
   policy bundle has quarantined the managed copy; the replay
   packet MAY still carry the local axis if it is still
   available.

## Provider / model availability

Every replay packet names a
`provider_availability` row that grades whether the original
provider / model identity can be reached **now** for re-invocation.
The grading drives the replay grade but is recorded separately so
reviewers can tell "the provider is gone" from "the model was
deprecated but the vendor still exists."

### Frozen `provider_availability` vocabulary

- `reachable_same_identity_same_version`
- `reachable_same_identity_newer_version`
- `reachable_different_identity`
- `unreachable_temporary`
- `unreachable_retired`
- `unreachable_policy_denied`
- `unreachable_entitlement_expired`
- `not_applicable_mocked`

Rules (frozen):

1. `full_replay` requires
   `reachable_same_identity_same_version`. Any other value
   forces `partial_replay_provider_unavailable` (or a more
   specific degradation) or a `non_replayable_*` grade.
2. `not_applicable_mocked` is reachable only for
   `provider_class = mocked_test_provider` assemblies.
3. `unreachable_policy_denied` surfaces the typed denial on
   the `ai_replay` audit stream so support and parity audits
   can tell a policy block from a provider outage.

## Mutation lineage

Every replay packet names a `mutation_status`. A turn that
produced no mutation names `not_a_mutation_turn`; a mutation
turn names its `mutation_journal_entry_refs` and whether the
mutation is still applied, reverted, or lost. A replay that
would otherwise be a `full_replay` downgrades to
`partial_replay_mutation_already_applied` when re-running
would attempt to re-apply an already-applied change.

### Frozen `mutation_status` vocabulary

- `not_a_mutation_turn`
- `mutation_plan_captured_and_applied`
- `mutation_plan_captured_not_applied`
- `mutation_plan_captured_reverted`
- `mutation_plan_lost`
- `mutation_plan_superseded`

Rules (frozen):

1. `mutation_plan_lost` forces `non_replayable_mutation_lost`;
   the packet still exists for evidence honesty and names
   which journal entry id was lost.
2. `mutation_plan_superseded` names the superseding
   mutation-journal entry so reviewers see the chain.
3. `mutation_plan_captured_and_applied` is the only status on
   which re-run would attempt to re-apply; the replay grade
   MUST reflect that.

## Audit-storage manifest

The `ai_audit_storage_manifest_record` is the typed register
describing **which retained artifacts exist for this turn**,
**what storage-object kind each is**, **what sensitivity /
retention / export / delete posture it carries**, and **which
record-class registry row governs it**. The manifest is the
cross-boundary shape every replay / support / parity / claim
surface reads before inventing private retention rules.

### Frozen `storage_object_kind` vocabulary

- `conversation_history`
- `derived_cache`
- `first_class_evidence_packet`
- `receipt_packet`
- `audit_event_stream`
- `replay_packet`

Rules (frozen):

1. `conversation_history` names the raw-reference-only store of
   user turns, AI responses, tool-call envelopes, and related
   session metadata. Raw prompt text, raw AI response bodies,
   raw tool-call payloads never appear; the history stores ids
   and typed vocabulary only.
2. `derived_cache` names caches the composer or provider
   adapter may keep for latency reasons; caches are
   authoritative for nothing, subject to broker redaction on
   materialization, and never a stand-in for a retained
   evidence packet.
3. `first_class_evidence_packet` names a retained
   `ai_evidence_packet_record` with the full
   `tainted_fence` / `source_reference` / `citation_anchor`
   set addressable separately.
4. `receipt_packet` names a retained
   `ai_route_receipt_record` or `ai_spend_receipt_record`
   whose presence is evidence the turn dispatched.
5. `audit_event_stream` names the local / managed audit-event
   backing for the `ai_context` / `ai_evidence` / `ai_replay` /
   `ai_audit_storage` event streams.
6. `replay_packet` names the retained
   `ai_replay_packet_record` itself; the manifest lists it so a
   reviewer can tell the packet set is complete.
7. Silent re-classification between the six kinds is
   non-conforming; the kind is frozen at mint time.

### Sensitivity, retention, export, delete posture

Each retained artifact row on the manifest names:

- `sensitivity_class` — one of the frozen
  `redaction_class` values re-exported from ADR-0011
  (`metadata_safe_default`, `operator_only_restricted`,
  `internal_support_restricted`, `signing_evidence_only`).
- `record_class_ref` — opaque pointer into
  `artifacts/governance/record_class_registry.yaml`
  (`ai_retained_evidence_packet` for first-class evidence
  packets; other rows for telemetry contracts, support
  bundles, offboarding packets, etc.). A retained artifact
  without a `record_class_ref` is non-conforming because its
  retention / hold / delete / export / offboarding posture
  would be invented per-surface.
- `retention_posture_class` — closed set:
  `local_ephemeral_session_scoped`,
  `local_retained_until_user_reset`,
  `local_retained_until_case_close`,
  `managed_retained_until_packet_expiry`,
  `managed_retained_until_case_close`,
  `retained_under_legal_hold`,
  `destruction_receipt_required_on_delete`.
- `export_posture_class` — closed set:
  `packet_is_export`, `exportable_on_request`,
  `local_only_not_exportable`, `export_denied_by_policy`.
- `delete_posture_class` — closed set:
  `request_supported`, `request_blocks_on_hold`,
  `request_denied_by_policy`, `not_applicable`.
- `redaction_posture` — closed set:
  `broker_redaction_pass_applied_at_capture`,
  `broker_redaction_pass_applied_at_export`,
  `broker_redaction_pass_to_be_reapplied_on_replay`,
  `no_redaction_required_metadata_only`.
- `exact_build_identity_ref` — the running-build identity the
  artifact was minted under.
- `provider_identity_ref` and `model_identity_ref` for
  `conversation_history`, `first_class_evidence_packet`, and
  `receipt_packet` kinds; absent (null) for the other kinds.

Rules (frozen):

1. Retention / hold / delete / export / offboarding posture is
   inherited from the referenced record-class row; the
   manifest quotes the row rather than re-minting the posture.
   A manifest that widens any posture beyond the row denies
   with `retention_posture_unresolved` (or the matching
   export / delete denial) rather than silently applying the
   wider posture.
2. `conversation_history` rows MUST carry
   `provider_identity_ref` and `model_identity_ref`;
   conversation history without both refs is non-conforming
   because a later reviewer could not tell which provider /
   model produced the retained turns.
3. Every `first_class_evidence_packet` row MUST carry
   `broker_redaction_pass_applied_at_capture` or
   `broker_redaction_pass_applied_at_export`; a packet with
   `no_redaction_required_metadata_only` is non-conforming —
   packets always pass through the broker pass on write.
4. `packet_is_export` artifacts MUST emit a
   `manifest_required = true` companion manifest per the
   record-class row; the replay packet quotes that companion
   manifest by id.

## Review-packet examples (lifecycle sketches)

### Full replay

```
 ┌────────────────────────────────┐
 │ inline composer turn (T1)      │
 │ provider: first_party_managed  │
 │ model: coder-large@v7          │
 │ mutation: plan captured + applied
 └─────────────┬──────────────────┘
               │ retention window open
               ▼
 ┌────────────────────────────────┐
 │ ai_replay_packet_record        │
 │ replay_grade = full_replay     │
 │ provider_availability =        │
 │   reachable_same_identity_same_
 │   version                      │
 │ mutation_status =              │
 │   mutation_plan_captured_and_
 │   applied                      │
 │ every required capture_class = │
 │   required_captured or         │
 │   required_captured_reference_ │
 │   only; raw bodies marked      │
 │   required_omitted_with_       │
 │   disclosure /                 │
 │   never_captured_by_design     │
 └─────────────┬──────────────────┘
               │
               ▼
 ┌────────────────────────────────┐
 │ ai_audit_storage_manifest_     │
 │ record lists:                  │
 │ conversation_history,          │
 │ first_class_evidence_packet,   │
 │ receipt_packet,                │
 │ audit_event_stream,            │
 │ replay_packet                  │
 └────────────────────────────────┘
```

### Partial replay — provider unavailable

```
 ┌────────────────────────────────┐
 │ composer turn from an older    │
 │ release; model identity has    │
 │ since been retired by vendor    │
 └─────────────┬──────────────────┘
               ▼
 ┌────────────────────────────────┐
 │ ai_replay_packet_record        │
 │ replay_grade =                 │
 │   partial_replay_provider_     │
 │   unavailable                  │
 │ provider_availability =        │
 │   unreachable_retired          │
 │ degraded_capture_classes =     │
 │   [provider_identity,          │
 │    model_identity]             │
 │ every other capture still      │
 │   captured or reference-only;  │
 │   mutation lineage preserved   │
 └────────────────────────────────┘
```

### Non-replayable — raw-byte dependent

```
 ┌────────────────────────────────┐
 │ composer turn whose behavior    │
 │ depended on a user-supplied    │
 │ file the user pasted inline.   │
 │ The user did not re-offer the  │
 │ file; the contract never       │
 │ retained the raw bytes.        │
 └─────────────┬──────────────────┘
               ▼
 ┌────────────────────────────────┐
 │ ai_replay_packet_record        │
 │ replay_grade =                 │
 │   non_replayable_raw_byte_     │
 │   dependent                    │
 │ capture_class rows:            │
 │   attachment_set =             │
 │     required_captured_reference_
 │     only,                      │
 │   user_supplied_file body =    │
 │     required_omitted_with_     │
 │     disclosure                 │
 │     (raw_user_supplied_text_   │
 │     forbidden_unless_          │
 │     separately_retained)       │
 │ mutation_status =              │
 │   not_a_mutation_turn          │
 │ evidence honesty preserved:    │
 │ the packet names what          │
 │ existed, not a guessed body    │
 └────────────────────────────────┘
```

## Per-turn-class projection requirements

| Turn class                 | Required replayability capture classes                                                                                                                                                                                                                                          | Grade when captured in full                             |
|----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------|
| `inline_composer`          | Full assembly, mention / attachment set, tool-call lineage, route / spend plan + receipts, provider / model identity, evidence packet, tainted-content fences, interaction-safety packet on apply, mutation-journal entry if a mutation was produced, running-build identity.     | `full_replay` when provider reachable same-identity-same-version and mutation status does not force a downgrade. |
| `background_branch_agent`  | Same plus branch-agent dispatch record, inherited tainted-usage constraints, inherited scope, inherited redaction class, downstream evidence packet the branch-agent produced.                                                                                                      | `full_replay` when every inherited constraint survived and provider reachable; otherwise a degraded grade. |
| `review_handoff`           | Same plus review-handoff packet.                                                                                                                                                                                                                                                 | `full_replay` when the review-handoff packet is still addressable. |
| `tool_follow_up`           | Same plus non-empty tool-call lineage; every `invoked_tainted_return` tool-call return recorded with a matching tainted-content fence.                                                                                                                                              | `full_replay` when every tool identity is re-invokable. |
| `support_replay_only`      | Assembly reconstructed for support / parity audit; no dispatch; `route_receipt` / `spend_receipt` absent by construction; `approval_ticket` `not_applicable`.                                                                                                                       | `full_replay` disallowed; grade reflects which retained artifacts remain. |

## Audit, redaction, and boundary posture

Process-boundary constraints (frozen):

1. `ai_replay_packet_record`,
   `ai_replay_capture_coverage_record`,
   `ai_replay_provider_availability_record`,
   `ai_replay_mutation_lineage_record`,
   `ai_replay_audit_event_record`,
   `ai_audit_storage_manifest_record`,
   `ai_audit_storage_artifact_record`, and
   `ai_audit_storage_audit_event_record` cross the RPC
   boundary as typed payloads (ADR-0004). Raw prompt text,
   raw document bodies, raw terminal / log bodies, raw
   generated artifact bytes, raw request / response payloads,
   raw user-supplied text, raw credential material, raw
   paths, and raw URLs never cross.
2. The broker-owned redaction pass (ADR-0007) runs on every
   write and on every export rewrite; a retained artifact
   whose `redaction_posture` is
   `no_redaction_required_metadata_only` is legal only for
   metadata-only rows.
3. A replay packet that quotes a retained artifact whose
   `ai_audit_storage_manifest_record` marks it as deleted /
   expired / released-from-hold MUST mark the matching
   capture-class row as
   `required_omitted_with_disclosure` with the appropriate
   `omission_reason`; silently treating a missing artifact as
   "captured" is non-conforming.
4. Replay packets themselves are governed by the
   `ai_retained_evidence_packet` record-class row; they
   inherit its retention / hold / delete / export /
   offboarding posture. A surface that re-invents the posture
   on the replay packet is non-conforming.

Redaction defaults (frozen):

| Sink                                 | Default inclusion                                                                                                                                                                                                                                                                                |
|--------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `logs_local`                         | Replay packet / capture-coverage / provider-availability / mutation-lineage / audit-storage-manifest / audit-storage-artifact ids, replay grade, provider availability, mutation status, storage object kind, sensitivity class, redaction posture, audit-event ids. No raw bodies or URLs.      |
| `traces_local`                       | Same as `logs_local`; span names MUST NOT include raw bodies or paths.                                                                                                                                                                                                                           |
| `support_bundle`                     | Full per-axis values, full capture-coverage enumeration, full audit-storage-manifest enumeration, full tainted-fence / citation-anchor enumeration via quoted evidence packet refs. Raw bodies excluded.                                                                                         |
| `evidence_packet`                    | Release-relevant fields: `running_build_identity_ref`, `replay_grade`, `provider_availability`, `mutation_status`, full capture-coverage set, full audit-storage-manifest set. Raw bodies never included.                                                                                          |
| `claim_manifest`                     | Full per-axis values. Raw bodies never included.                                                                                                                                                                                                                                                  |
| `mutation_journal_entry`             | Ids, `replay_grade`, `mutation_status`, `provider_availability`. No raw bodies or URLs.                                                                                                                                                                                                          |
| `crash_dump`                         | Opt-in only; broker-redaction pass precedes packaging; denied by default for packets whose `policy_context` references a managed policy bundle.                                                                                                                                                   |
| `profile_export` / `sync`            | Replay packet id, audit-storage manifest id, `running_build_identity_ref` only. Raw bodies forbidden.                                                                                                                                                                                              |

Overrides are narrowing only; admin policy MAY reduce inclusion
further, but MAY NOT widen beyond the frozen exclusion rules.

## Schema-of-record posture

The eventual AI / prompt-composer / retention-service crate's
Rust types are the source of truth. The JSON Schema exports at
`schemas/ai/evidence_replay_packet.schema.json` and
`schemas/ai/audit_storage_manifest.schema.json` are the
cross-tool boundary every non-owning surface reads. Adding a new
`replay_grade`, `capture_class`, `capture_posture`,
`omission_reason`, `provider_availability`, `mutation_status`,
`storage_object_kind`, `retention_posture_class`,
`export_posture_class`, `delete_posture_class`,
`redaction_posture`, `audit_event_id`, or `denial_reason` value
is additive-minor and bumps the relevant `*_schema_version` const;
repurposing an existing value is breaking and requires a new
decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors the context-assembly contract and ADR
0004 through ADR 0014.

## Non-goals at this milestone

Out of scope until a superseding decision row opens:

- Shipping a working replay engine. This contract reserves the
  packet shape; the replay runtime wires execution later.
- Broad AI feature set. This contract is about what a reviewer
  can explain after the fact, not about what the composer can
  do before the fact.
- Provider-side record-linkage APIs; the provider exposes what
  it exposes, and `provider_did_not_disclose` names the axis
  when it does not.
- Cross-tenant replay. Every replay packet carries a
  `policy_context` and a `running_build_identity_ref`; a
  reviewer on a different tenant or a different build reads
  the packet as a read-only artifact, not as a dispatchable
  work item.

These lines move only by opening a new decision row, not by
editing this contract.

## Reuse guarantee

This contract is reusable by review, support, parity-audit, and
claim-manifest flows without redefining replay or retention
semantics. A new AI-facing surface MUST:

1. Quote the `replay_grade`, `capture_class`, `capture_posture`,
   `omission_reason`, `provider_availability`, `mutation_status`,
   `storage_object_kind`, `retention_posture_class`,
   `export_posture_class`, `delete_posture_class`, and
   `redaction_posture` vocabularies above verbatim.
2. Emit `ai_replay_packet_record` on every retained turn —
   including `support_replay_only` reconstructions — and emit
   `ai_audit_storage_manifest_record` naming every retained
   artifact that backs the replay packet.
3. Preserve each axis as a separately addressable field even
   when the UI folds them into one chip.
4. Honour the redaction / retention / export / delete / hold
   posture inherited from the referenced record-class rows;
   widening any posture on handoff is non-conforming.
