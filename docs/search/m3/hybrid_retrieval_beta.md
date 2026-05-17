# Hybrid retrieval and retrieval inspector beta

This reviewer doc defines the beta contract for hybrid retrieval across search,
AI context, review evidence, docs/help, and support export surfaces.

Implementation:
[`crates/aureline-search/src/hybrid_retrieval/`](../../../crates/aureline-search/src/hybrid_retrieval/).
Boundary schema:
[`schemas/search/retrieval_inspector.schema.json`](../../../schemas/search/retrieval_inspector.schema.json).
Protected fixture:
[`fixtures/search/hybrid_retrieval_beta/`](../../../fixtures/search/hybrid_retrieval_beta/).
Checked-in packet:
[`artifacts/search/m3/hybrid_retrieval_beta_packet.json`](../../../artifacts/search/m3/hybrid_retrieval_beta_packet.json).
AI consumer wrapper:
[`crates/aureline-ai/src/context_inspector/mod.rs`](../../../crates/aureline-ai/src/context_inspector/mod.rs).

## Product contract

Every claimed hybrid retrieval row must expose the same packet to in-product
inspection and export. The packet carries:

- lexical, vector, and graph contributions on the fused row;
- locality for each lane and contribution (`local_workspace`, `local_cache`,
  `mirrored_pack`, `managed_tenant_scoped`, or `provider_remote`);
- readiness and freshness for each lane;
- embedder model id, tokenizer/chunker generation, trust boundary, and
  retrieval epoch whenever vector recall contributes;
- graph epoch whenever graph recall contributes;
- a local-first policy disclosure that names remote fallback or remote block
  states instead of hiding them behind generic "semantic" wording;
- consumer projections proving search, AI context, and support export read the
  same `retrieval_inspector_beta_packet`; and
- promotion blockers when a row overclaims locality, readiness, vector epoch,
  or export parity.

The packet is metadata-only. It must not contain raw vectors, raw source bodies,
provider payloads, secrets, raw URLs, ambient authority, or private numeric rank
weights. Support export preserves the packet id and embeds the exact packet that
the product inspector showed.

## Local-first policy

The default route is local. A remote or managed vector lane is allowed only when
the packet records all of the following:

- `route_policy` explains the route, such as
  `local_first_remote_fallback` or `remote_allowed_by_policy`;
- `preferred_locality` remains visible;
- `active_locality` shows where recall actually ran;
- `fallback_reason` explains why local recall was not enough; and
- `remote_route_disclosed = true`.

Provider writes are not part of retrieval. The Rust validator blocks packets
with `provider_write_allowed = true`, and every row must declare that mutating
actions require live target re-resolution against current buffers, graph
identity, and trust state.

## Embedding index baseline

Vector contributions cite an `EmbeddingIndexManifest`. The manifest isolates
derived vectors by:

- workspace id;
- snapshot or revision ref;
- retrieval epoch;
- embedder model id and version;
- tokenizer id;
- chunker id;
- trust boundary; and
- policy scope.

Changing the embedder, tokenizer, chunker, or trust boundary requires a new
retrieval epoch. A vector contribution without a matching manifest blocks beta
promotion with `missing_embedding_manifest`.

## Validation behavior

`RetrievalInspectorPacket::materialize` derives validation findings and a
`promotion_state`.

Blockers include:

- missing lexical, vector, or graph contribution;
- missing vector manifest or embedder identity;
- undisclosed managed or remote retrieval;
- missing product/support projection preserving the same packet;
- provider write permission on retrieval; and
- rows that allow mutation without live re-resolution.

Warnings include partial, warming, hot-set, or stale lanes that do not carry a
visible cause. The fixture keeps graph partiality labeled with
`graph_slice_partial`, while remote vector fallback remains explicit through
`local_vector_index_unavailable`.

## Consumer parity

Search is the owner of the packet. AI does not re-derive retrieval truth from
context rows; it wraps the same search packet with
`AiContextRetrievalExport`, which validates that the packet includes an
`ai_context` projection and that the embedded packet itself still validates.
Support export wraps the same packet with `RetrievalInspectorSupportExport`.

The protected tests round-trip the fixture, artifact, support export, and AI
wrapper so docs, product inspection, and export cannot diverge silently.
