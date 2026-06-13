# Reusable Semantic-Memory and Embedding-Index Records

This contract describes the export-safe packet that materializes the reusable
*derived retrieval artifacts* the M5 recall surfaces read: reusable semantic
memory and embedding indexes. Where the recall matrix
([`freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md`](freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md))
qualifies whole surfaces and the materialized memory-class lane
([`implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth.md`](implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth.md))
records the per-scope memory objects, this packet records the concrete reusable
semantic-memory and embedding-index records those objects depend on, one record
per artifact, with explicit epoch lineage, retrieval generation state, locality,
and delete/export posture.

The canonical Rust type is
`aureline_ai::ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth::SemanticRecallRecordsPacket`.
The boundary schema is
[`schemas/ai/ship-reusable-semantic-memory-and-embedding-index-records-with-epoch-invalidation-locality-and-no-mixed-generation-truth.schema.json`](../../../schemas/ai/ship-reusable-semantic-memory-and-embedding-index-records-with-epoch-invalidation-locality-and-no-mixed-generation-truth.schema.json).
The checked support export is
[`artifacts/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth/support_export.json`](../../../artifacts/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth/support_export.json).

## Artifact kinds

Each record materializes exactly one derived retrieval artifact.

| Artifact kind | Notes |
| --- | --- |
| `reusable_semantic_memory` | Reusable semantic memory derived from prior recall. |
| `embedding_index` | Embedding index built over a corpus; governed by embedding generation. |

## Epoch lineage and invalidation

Every record declares a graph epoch, a docs epoch, a model epoch, and an
embedding generation, and binds at least one `epoch_kind` (`graph`, `docs`, or
`model`) whose bump invalidates it. For each bound epoch the record MUST carry the
matching invalidation trigger (`graph_epoch_bump`, `docs_epoch_bump`, or
`model_epoch_bump`). An `embedding_index` MUST additionally bind the `model`
epoch and carry `embedding_generation_bump`, so a prior generation can never
silently serve as current truth.

## No mixed-generation retrieval truth

A record's `generation_state` is one of `current`, `recomputing`, `invalidated`,
`stale`, or `mixed_blocked`. Only `current` may be served as current retrieval
truth. When `mixed_generation_detected` is `true`, the record MUST be
`mixed_blocked` or `invalidated`, never `current` — mixed-generation embeddings
never masquerade as current truth. A `recomputing`, `invalidated`, `stale`, or
policy-blocked record degrades to a precise `degraded_label`, never one generic
"retrieval unavailable" state; validation rejects a record whose required label is
missing or generic.

## Local-versus-managed locality cues

Locality is one of four distinct, export-safe states that never collapse into one
generic semantic-search state:

| Locality | Meaning |
| --- | --- |
| `local_device_only` | Built and held on the local device only. |
| `workspace_mirrored` | Mirrored within the workspace; no cross-workspace recall. |
| `managed_hosted` | Managed-hosted and region-pinned within the tenant. |
| `policy_blocked` | Blocked by a region or policy gate; retrieval withheld, with a precise label. |

## Deletion and export truth

Both artifact kinds are durable derived artifacts, so every record MUST declare an
actionable delete and export posture (`user_scoped`, `workspace_scoped`,
`tenant_scoped`, or `org_scoped`). A policy-blocked record still names its
org/tenant-scoped delete and export posture so operators retain control over what
is retained.

## Consumer surfaces

Each record names the surfaces that read it — `composer_assist`,
`docs_browser_recall`, `code_understanding`, `semantic_search`, `support_export`,
and `managed_offline_report`. The consumer projection block records that composer
shows memory and generation, docs/browser shows provenance and locality, search
shows the retrieval generation state, support export shows locality and epoch
lineage, managed/offline reporting shows locality truth, and invalidated or
recomputing lanes are labeled below current.

## Guardrails

- No cross-workspace or cross-tenant recall happens by default.
- Mixed-generation embeddings never masquerade as current truth.
- Invalidated or recomputing lanes are always labeled explicitly.
- Every durable record declares its delete and export posture.
- The four locality states stay distinct.
- A graph, docs, or model epoch bump invalidates the records bound to it.

## Boundary

Raw prompt bodies, cached result bodies, raw embeddings, raw provider payloads,
credentials, exact token counts, and exact cost amounts never cross this
boundary. Consumers project the typed packet directly rather than re-deriving
epoch lineage, generation state, locality, or delete/export posture locally.
