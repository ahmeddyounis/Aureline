# M5 AI Recall Matrix

- Packet: `m5-ai-recall-matrix:stable:0001`
- Schema: `schemas/ai/freeze-the-m5-ai-memory-prompt-result-cache-hybrid-retrieval-and-retrieval-locality-matrix.schema.json`
- Support export: `artifacts/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix/support_export.json`
- Fixture: `fixtures/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix/`

## Coverage

The matrix freezes the M5 recall qualification for every claimed AI or recall
surface. Each row names the surface's memory/cache classes, retrieval lanes,
locality posture, cache-invalidation classes, delete/export posture,
budget/receipt expectation, downgrade triggers, required evidence, and consumer
parity.

- **Composer assist** is Stable: reusable semantic memory plus a content-hash
  prompt-result cache, workspace-local, user-deletable/exportable, with
  route-and-spend receipts on every call.
- **Docs/browser recall** is Stable: a workspace-local embedding index and
  result cache with cited provenance, locality disclosure, and embedding-
  generation invalidation.
- **Code understanding** is Beta: graph and embedding retrieval over a
  workspace-local index with budget-capped fallback.
- **Semantic search** is Stable: labeled lexical, semantic, and hybrid lanes
  with content-hash and embedding-generation cache keys.
- **Support/export** is Stable: a no-durable-memory projection that names
  locality, delete/export behavior, and receipts without raw bodies.
- **Managed/offline** is Beta: region-pinned locality with local-no-spend
  receipt truth and auto-expiring ephemeral state.

## Auto-narrowing

A surface that omits its memory/cache classes, retrieval lanes, cache-
invalidation classes, downgrade triggers, evidence, or a delete/export posture
for durable memory is not recall-complete. A declared Stable claim on an
incomplete surface narrows to Preview, and the packet fails validation with
`stable_claim_exceeds_recall_evidence` so the claim never publishes ahead of its
evidence.

## Safety

The matrix proves there is no cross-workspace or cross-tenant recall by default,
mixed-generation embeddings are always labeled, stale hybrid retrieval never
masquerades as current truth, every durable artifact declares retention, caches
are not used as shadow telemetry stores, and spend or route failures resolve to
a precise fallback rather than a generic provider error. Raw prompt bodies,
cached result bodies, raw embeddings, raw provider payloads, credentials, exact
token counts, and exact cost amounts never cross the support boundary.
