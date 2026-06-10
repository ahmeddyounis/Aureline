# Richer M5 Prompt Composer with Intent Modes, Typed Attachments, Context Pinning, and Omitted-Context Truth

This document is the **M5 contract** for the richer prompt composer that unifies the M3 beta conformance and M4 stable stabilization lanes into one depth-qualified packet.

## Scope

The richer prompt composer adds four M5-specific richness layers:

1. **Intent modes with behavior constraints** — Each [`IntentModeClass`](../../context_inspector/mod.rs) carries a closed set of [`IntentModeBehaviorConstraint`](../../crates/aureline-ai/src/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/mod.rs) values that govern what the mode may do, what it must prove before send, and what approval posture applies.
2. **Typed attachments with semantic roles and provenance** — Every attachment carries a [`AttachmentSemanticRoleClass`](../../crates/aureline-ai/src/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/mod.rs) and an [`AttachmentProvenanceClass`](../../crates/aureline-ai/src/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/mod.rs) so the composer surface can explain *why* the object is present and *what role* it plays in the turn.
3. **Context pinning with policies and auto-refresh** — Pinned context is no longer a static state. Each pin carries a [`PinPolicyClass`](../../crates/aureline-ai/src/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/mod.rs) and a [`PinAutoRefreshClass`](../../crates/aureline-ai/src/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/mod.rs) that determines how staleness is handled and whether send is blocked until the operator refreshes or removes the pin.
4. **Omitted-context truth with restoration paths** — Every omitted source carries an [`OmittedContextRestorationClass`](../../crates/aureline-ai/src/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/mod.rs) and an [`ExclusionFreshnessClass`](../../crates/aureline-ai/src/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/mod.rs) so the operator knows whether the exclusion is permanent, session-scoped, or reversible, and whether the reason still holds.

## Boundaries

- The packet carries **metadata, state tokens, refs, and labels only**.
- It does **not** carry raw prompt bodies, raw file contents, raw provider payloads, endpoint URLs, credentials, exact token counts, exact prices, or billing account ids.
- It references the M3 conformance packet and M4 stabilization packet by id rather than embedding their content.

## Companion Artifacts

- [`schemas/ai/implement-a-richer-prompt-composer-with-intent-modes-typed-attachments-context-pinning-and-omitted-context-tru.schema.json`](../../schemas/ai/implement-a-richer-prompt-composer-with-intent-modes-typed-attachments-context-pinning-and-omitted-context-tru.schema.json) — boundary schema for the `richer_prompt_composer_packet` shape.
- [`artifacts/ai/m5/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/support_export.json`](../../artifacts/ai/m5/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/support_export.json) — checked-in support export.
- [`fixtures/ai/m5/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/`](../../fixtures/ai/m5/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/) — protected fixture directory.

## Upstream Contracts

This contract composes with and does not replace:

- [`docs/ai/prompt_composer_contract.md`](../prompt_composer_contract.md) — the product-wide prompt-composer contract.
- [`docs/ai/context_assembly_contract.md`](../context_assembly_contract.md) — the ordered-segment, trust-posture, and fence vocabularies.
- M3 beta conformance artifact at `artifacts/ai/m3/prompt_composer_conformance/support_export.json`.
- M4 stable stabilization artifact at `artifacts/ai/m4/prompt_composer_stabilization/support_export.json`.

## Validation Invariants

A valid richer prompt-composer packet MUST:

- Carry the correct `record_kind` and `schema_version`.
- Reference a valid M3 conformance packet and M4 stabilization packet with matching ids.
- Include all required source contract refs (this doc, the base prompt-composer contract, the boundary schema, and the upstream conformance/stabilization artifacts).
- Include an intent mode row whose `behavior_constraints` covers every constraint returned by `IntentModeBehaviorConstraint::required_for_mode` for the selected mode.
- Include at least one attachment row for every required `StableAttachmentSourceClass`.
- Ensure every pinned context row with `stale_after_duration` policy carries a `stale_after_duration_seconds` value.
- Surface `pinned_but_stale` freshness with a `drift_source` and `blocks_send_until_resolved` when the underlying object changed.
- Ensure every omitted context row carries `inspectable_after_send`, `replay_explains_exclusion`, and a `restoration_action_ref`.
- Ensure budget overflow carries at least one decision row with a non-empty `reason_token` and a non-empty `explanation_label`.
- Prove cross-surface consistency for `editor_attached`, `sidebar`, and `detached` surfaces, including reachability for pinned-context review and intent-mode constraint visibility.
- Preserve complete evidence lineage with all four required packet classes.
- Contain no raw boundary material (URLs, API keys, bearer tokens, raw prompt bodies, billing account ids, or user paths).

## Downgrade and Rollback

If any of the following conditions hold, the richer prompt composer lane MUST narrow its qualification rather than hide the defect:

- The promoted conformance or stabilization packet fails validation or mismatches on identity.
- Required behavior constraints are missing for the claimed intent mode.
- Attachment source class coverage is incomplete.
- Pinned stale context is not surfaced before reuse.
- Omitted context is not inspectable or restorable after send.
- Cross-surface consistency is unproven.
- Evidence lineage is incomplete.
- The checked-in support export contains raw boundary material.

## Consumer Surfaces

The richer prompt-composer packet is consumed by:

- Desktop composer (editor-attached, sidebar, detached)
- CLI/headless support-export tooling
- Release-evidence review tooling
- Compliance audit replay

## M5 Qualification Claim

This lane is claimed as **Stable** for Milestone 5 under the condition that the checked-in implementation, fixtures, schema, and proof packet are current on the mainline branch and referenced by the canonical M5 evidence index.
