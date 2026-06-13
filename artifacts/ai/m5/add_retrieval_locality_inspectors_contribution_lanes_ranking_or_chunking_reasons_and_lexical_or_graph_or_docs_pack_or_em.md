# Retrieval Locality Inspector

- Packet: `retrieval-locality-inspector:stable:0001`
- Schema: `schemas/ai/add-retrieval-locality-inspectors-contribution-lanes-ranking-or-chunking-reasons-and-lexical-or-graph-or-docs-pack-or-em.schema.json`
- Support export: `artifacts/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em/support_export.json`
- Contract doc: `docs/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em.md`
- Fixture: `fixtures/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em/`

## Coverage

The inspector explains a produced recall result across search, docs recall, and
AI context packs. Each surface labels its contribution lanes (lexical, graph,
docs-pack, embedding, provider-overlay) with a per-lane ranking-or-chunking
reason, retrieval locality, and generation, plus the surface's hidden-scope
count, degraded lanes, provider-overlay posture, and completeness claim.

- **Search** is complete: lexical, graph, and embedding lanes each contribute
  current, workspace-local candidates with ranking reasons; the provider overlay
  is labeled empty and the overlay posture is local-only.
- **Docs recall** is a partial-hidden-scope result: lexical and docs-pack lanes
  contribute with chunking reasons while the embedding lane is shown degraded on
  a stale generation, three in-scope candidates are disclosed as hidden, and the
  surface preserves its labels for replay and support export.
- **AI context pack** is complete: graph, embedding, and docs-pack lanes
  contribute with ranking and chunking reasons, mixed embedding generations are
  labeled, and a region-pinned managed provider overlay is merged with disclosure.

## Guardrails

No cross-workspace or cross-tenant recall by default; mixed or stale generations
are labeled and never masquerade as current; degraded lanes never appear in a
result claimed complete; a contributing provider overlay is always disclosed;
replay and support exports preserve lane and locality labels; and hidden in-scope
counts are disclosed rather than silently dropped.
