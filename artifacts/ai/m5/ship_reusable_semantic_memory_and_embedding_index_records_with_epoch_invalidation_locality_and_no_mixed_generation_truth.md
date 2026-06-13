# Reusable Semantic-Memory and Embedding-Index Records

- Packet: `m5-semantic-recall-records:stable:0001`
- Schema: `schemas/ai/ship-reusable-semantic-memory-and-embedding-index-records-with-epoch-invalidation-locality-and-no-mixed-generation-truth.schema.json`
- Support export: `artifacts/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth/support_export.json`
- Fixture: `fixtures/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth/`
- Doc: `docs/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth.md`

## Coverage

The packet materializes reusable semantic-memory and embedding-index records
across both artifact kinds, all three epoch dimensions (graph, docs, model), and
all four locality states (local, mirrored, managed, policy-blocked), and includes
recomputing, invalidated, and mixed-blocked lanes that demonstrate explicit
labeling.

- **workspace-semantic-memory-local** (reusable_semantic_memory / local_device_only):
  on-device semantic memory bound to graph, docs, and model epochs, single
  generation, current.
- **workspace-embedding-index-mirrored** (embedding_index / workspace_mirrored):
  docs+model bound index, single generation, current.
- **managed-embedding-index-recomputing** (embedding_index / managed_hosted):
  recomputing after a model epoch bump; labeled, not served as current truth.
- **managed-semantic-memory-mixed-blocked** (reusable_semantic_memory / managed_hosted):
  spans two embedding generations; blocked from current retrieval truth.
- **workspace-embedding-index-invalidated** (embedding_index / workspace_mirrored):
  invalidated by a graph epoch bump; awaiting recompute.
- **org-semantic-memory-policy-blocked** (reusable_semantic_memory / policy_blocked):
  withheld by a region policy gate and labeled precisely, with delete and export
  remaining org-scoped.

## Epoch invalidation truth

Every record declares a graph/docs/model epoch lineage plus an embedding
generation and binds the epochs whose bump invalidates it. Each bound epoch
carries its matching bump trigger, and every embedding index binds the model epoch
and `embedding_generation_bump` so a prior generation can never silently serve as
current truth.

## No mixed-generation truth

A record with `mixed_generation_detected` set to `true` is `mixed_blocked` or
`invalidated`, never `current`. Recomputing, invalidated, stale, and policy-blocked
records degrade to a precise `degraded_label`, never one generic "retrieval
unavailable" state. Validation rejects a mixed-generation record that claims
current truth (`mixed_generation_masquerades_as_current`) and a required label that
is missing or generic (`degraded_label_missing`).

## Safety

No cross-workspace or cross-tenant recall happens by default, the four locality
states stay distinct and export-safe, every durable record declares an actionable
delete and export posture, and a graph, docs, or model epoch bump invalidates the
records bound to it. Raw prompt bodies, cached result bodies, raw embeddings, raw
provider payloads, credentials, exact token counts, and exact cost amounts never
cross the support boundary.
