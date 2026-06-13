# M5 AI Recall Matrix — Memory, Prompt-Result Cache, Hybrid Retrieval, and Retrieval Locality

This document freezes the canonical M5 recall qualification for every claimed AI
or recall surface. It is the single source of truth that AI, search, docs/browser,
graph, support/export, and managed/offline lanes reference instead of
re-describing cache, memory, retrieval, or locality behavior per surface.

The six governed surfaces are:

1. **Composer assist** — Composer recall over reusable semantic memory and the prompt-result cache.
2. **Docs/browser recall** — Docs and in-app browser recall with cited provenance.
3. **Code understanding** — Codebase-understanding recall over the workspace graph and embedding index.
4. **Semantic search** — Semantic and hybrid search with labeled retrieval lanes.
5. **Support/export** — Support and export projection of recall posture.
6. **Managed/offline** — Managed and offline usage and locality reporting.

## Packet

The machine-readable packet is owned by `crates/aureline-ai/src/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix/`.

- Record kind: `freeze_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix`
- Schema version: `1`
- Schema: `schemas/ai/freeze-the-m5-ai-memory-prompt-result-cache-hybrid-retrieval-and-retrieval-locality-matrix.schema.json`
- Checked-in export: `artifacts/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix/support_export.json`

## Surface Rows

Each surface row binds:

- **Qualification class** — `stable`, `beta`, `preview`, `experimental`, `unavailable`, or `held`.
- **Scope summary** — A review-safe description of what the surface recalls and how it is bounded.
- **Memory/cache classes** — `prompt_result_cache`, `reusable_semantic_memory`, `embedding_index`, `ephemeral_session_state`, or `no_durable_memory`.
- **Retrieval lanes** — `lexical_keyword`, `semantic_embedding`, `hybrid_fusion`, `graph_traversal`, or `no_retrieval`.
- **Locality posture** — `local_device_only`, `workspace_local`, `tenant_region_pinned`, or `managed_hosted_region_pinned`.
- **Delete/export posture** — `user_deletable_exportable`, `workspace_deletable_exportable`, `tenant_deletable_exportable`, `ephemeral_auto_expire`, or `not_applicable`.
- **Budget/receipt expectation** — `route_and_spend_receipt_required`, `spend_receipt_required`, `local_no_spend_receipt`, or `budget_capped_with_fallback`.
- **Cache-invalidation classes** — `ttl_expiry`, `content_hash_key`, `policy_epoch_bump`, `trust_narrowing`, `embedding_generation_bump`, or `manual_purge`.
- **Evidence requirement** and **required evidence packet refs** — The upstream qualification packets that must be current.
- **Downgrade triggers** — Closed set of conditions that automatically narrow the surface.
- **Source contract refs** — The frozen contracts this surface projects against.
- **Consumer surfaces** — The surfaces that must show recall truth.

## Guardrail Invariants

The packet enforces seven guardrail invariants, all of which must hold:

1. `no_cross_workspace_recall_by_default` — No recall crosses workspace boundaries by default.
2. `no_cross_tenant_recall_by_default` — No recall crosses tenant boundaries by default.
3. `mixed_generation_embeddings_labeled` — Mixed-generation embeddings are always labeled, never silently merged.
4. `stale_hybrid_retrieval_never_current_truth` — Stale hybrid retrieval never masquerades as current truth.
5. `every_durable_artifact_declares_retention` — Every durable artifact declares its retention/delete/export posture.
6. `caches_are_not_shadow_telemetry` — Prompt-result caches are not used as shadow telemetry stores.
7. `spend_or_route_failures_have_precise_fallback` — Spend or route failures resolve to a precise fallback, not a generic provider error.

## Consumer Projection

All seven projection invariants must hold so each surface shows recall truth:

- Composer shows which memory and retrieval lanes were used.
- Docs/browser shows provenance and locality.
- Search shows which retrieval lanes contributed.
- Support export shows locality posture and receipts.
- Diagnostics shows cache and budget state.
- Managed/offline reporting shows locality truth.
- Surfaces below Stable are visibly labeled, never presented as Stable.

## Auto-Narrowing

A surface is **recall-complete** only when it declares at least one memory/cache
class, at least one retrieval lane, at least one cache-invalidation class, at
least one downgrade trigger, at least one required evidence packet ref, and — for
any durable memory — a real delete/export posture (not `not_applicable`).

A declared Stable surface that is not recall-complete narrows to Preview via
`effective_qualification`, and `validate` rejects the packet with
`stable_claim_exceeds_recall_evidence`. This is how a missing locality,
invalidation, or delete/export row automatically narrows the claim before docs,
product surfaces, or release packets publish it.

## Proof Freshness

- Proof-freshness SLO: 168 hours.
- Last refresh tracked per packet.
- Auto-narrow on stale proof is enabled.

## Source Contracts

The matrix consumes the following frozen contracts:

- `docs/ai/memory_class_matrix.md` — Memory class taxonomy.
- `docs/ai/ai-memory-delete-export.md` — Delete and export posture.
- `docs/ai/spend_and_route_receipt_contract.md` — Spend and route receipts.
- `docs/ai/context_assembly_contract.md` — Context assembly / retrieval input.
- `docs/search/result_identity_and_ranking.md` — Retrieval identity and ranking.

## Validation

The Rust module validates:

- All six surfaces are present.
- Record kind and schema version match the constants.
- Identity fields are non-empty.
- All required source contracts are cited.
- Each surface declares at least one memory/cache class and retrieval lane.
- Each surface declares at least one cache-invalidation class.
- Durable-memory surfaces declare a real delete/export posture.
- Stable surfaces carry at least one required evidence packet ref and are recall-complete.
- Every surface has at least one downgrade trigger and one consumer surface.
- All guardrail and consumer-projection booleans are true.
- Proof-freshness SLO is non-zero and last refresh is non-empty.
- Export JSON contains no raw boundary material (credentials, secrets, API keys, bearer tokens).
