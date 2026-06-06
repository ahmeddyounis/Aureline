# Stabilize Hybrid Retrieval Epochs And Locality

This is the stable search-owned contract for hybrid retrieval inspector packets.
It hardens the beta retrieval inspector with stable lane, locality, epoch,
tenant-boundary, and export-parity checks.

Implementation:
[`crates/aureline-search/src/hybrid_retrieval/`](../../../crates/aureline-search/src/hybrid_retrieval/).
Stable schema:
[`schemas/search/hybrid-retrieval-inspector.schema.json`](../../../schemas/search/hybrid-retrieval-inspector.schema.json).
Stable fixture corpus:
[`fixtures/search/m4/stabilize-hybrid-retrieval-epochs-and-locality/`](../../../fixtures/search/m4/stabilize-hybrid-retrieval-epochs-and-locality/).
Checked-in packet:
[`artifacts/search/m4/hybrid_retrieval_inspector_packet.json`](../../../artifacts/search/m4/hybrid_retrieval_inspector_packet.json).

## Stable Contract

Stable packets set `governance_track = stable` and must classify the query as
`exact`, `structural`, `conceptual`, or `mixed`. The inspector keeps lexical,
structural, graph, embedding, and fusion lanes explicit so exact answers can
render before higher-latency embedding recall completes.

Embedding manifests identify the workspace or signed pack, snapshot,
retrieval epoch, embedder model and version, tokenizer, chunker, trust
boundary, policy scope, and retention policy. Changing any embedder-generation
component advances the retrieval epoch and invalidates affected indexes. Stable
validation blocks stale, unavailable, incompatible, or mixed-generation
embedding recall from publishing as current.

Managed or remote embedding recall is optional and must disclose tenant scope,
region, policy scope, and route policy. Local-only routing remains the default;
the packet must not silently widen to managed recall when local semantic state
is cold. Mirrored packs may carry precomputed embeddings only when the signed
pack ref and compatibility ref are visible.

## Policy-Hidden Omissions

Policy-hidden omitted lanes are first-class packet rows. Each omission carries
an omitted lane class, support-safe reason, and disclosure ref. Result cards,
retrieval inspectors, AI context packets, review workspaces, docs/help, and
support exports consume the same packet id instead of inventing local omission
summaries.

## Mutation Boundary

Hybrid recall may only produce candidates. Every row keeps
`mutating_actions_require_live_resolution = true`, so refactors, edits, and
approval surfaces must re-resolve final targets against live buffers, graph
identity, and current trust state.
