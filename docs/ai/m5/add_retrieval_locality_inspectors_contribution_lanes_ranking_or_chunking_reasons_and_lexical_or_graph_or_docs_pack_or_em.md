# Retrieval Locality Inspector — Contribution Lanes, Ranking-or-Chunking Reasons, and Lane Labeling

This document defines the canonical retrieval-locality inspector that explains a
single produced recall result across **search**, **docs recall**, and **AI
context packs**. Where the frozen recall matrix
(`docs/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md`)
qualifies a whole surface, the inspector explains *how a result was built*: which
lanes contributed, why each candidate was selected, where the data lived, and how
honest the result's completeness claim is.

The three inspected surfaces are:

1. **Search** — Workspace search results.
2. **Docs recall** — Docs and in-app browser recall pane.
3. **AI context pack** — The context pack assembled for a model.

## Packet

The machine-readable packet is owned by
`crates/aureline-ai/src/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em/`.

- Record kind: `retrieval_locality_inspector`
- Schema version: `1`
- Schema: `schemas/ai/add-retrieval-locality-inspectors-contribution-lanes-ranking-or-chunking-reasons-and-lexical-or-graph-or-docs-pack-or-em.schema.json`
- Checked-in export: `artifacts/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em/support_export.json`
- Conformance dump: `cargo run -p aureline-ai --example dump_retrieval_locality_inspector`

## Contribution Lanes

Each surface row labels its contribution lanes. A contribution lane binds:

- **Lane** — `lexical_keyword`, `graph_traversal`, `docs_pack`, `embedding_vector`, or `provider_overlay`.
- **State** — `contributed`, `degraded`, `empty`, or `suppressed`. Only `contributed` and `degraded` are active.
- **Selection reason kind** — `ranking`, `chunking`, `overlay`, or `not_applicable`.
- **Selection reason** — the concrete ranking-or-chunking reason: `rank_by_semantic_similarity`, `rank_by_lexical_score`, `rank_by_graph_proximity`, `rank_by_hybrid_fusion`, `chunk_by_semantic_boundary`, `chunk_by_doc_structure`, `chunk_by_fixed_window`, `provider_overlay_merge`, or `not_applicable`.
- **Locality** — where the lane's data lived: `local_device_only`, `workspace_local`, `tenant_region_pinned`, `managed_hosted_region_pinned`, or `provider_remote`.
- **Generation state** — `current`, `recomputing`, `stale`, or `mixed_generation_labeled`.
- **Reason summary** — a review-safe description of what the lane contributed and why.

A lane row is rejected unless its concrete reason matches its declared kind, an
inactive lane declares a `not_applicable` reason (and an active one does not), a
contributing `provider_overlay` lane uses an overlay reason with a remote or
managed locality, and no other lane claims an overlay reason. A lane whose
generation is `recomputing` or `stale` must present as `degraded`, never as a
clean contribution.

## Surface Rows

Each surface row also records:

- **Hidden scope count** — in-scope candidates withheld from the result.
- **Degraded lanes** — the lanes the surface reports as degraded; this must match the lanes whose state is `degraded` exactly.
- **Provider-overlay posture** — `no_overlay`, `local_only_no_overlay`, `overlay_disclosed`, or `overlay_degraded`; a contributing overlay lane must be disclosed.
- **Completeness claim** — `complete`, `partial_hidden_scope`, or `degraded_subset`; a surface that hides scope or degrades a lane may not claim `complete`.
- **Replay label parity** — whether exported and replayed labels match in-product labels; required for any surface a support export or replay packet can reach.
- **Downgrade triggers** — `lane_degraded`, `mixed_generation_unlabeled`, `hidden_scope_undisclosed`, `provider_overlay_undisclosed`, `replay_label_drift`, `locality_unavailable`, `proof_stale`.
- **Source contract refs** and **consumer surfaces** — the contracts a surface projects against and the surfaces that must show its labels.

`RetrievalInspectorSurfaceRow::effective_completeness` narrows a dishonest
`complete` claim to `degraded_subset` (when a lane degraded) or
`partial_hidden_scope`, so a claim never outruns what the row can prove.

## Guardrail Invariants

The packet enforces seven guardrail invariants, all of which must hold:

1. `no_cross_workspace_recall_by_default` — No recall crosses workspace boundaries by default.
2. `no_cross_tenant_recall_by_default` — No recall crosses tenant boundaries by default.
3. `mixed_generation_labeled_never_masquerades` — Mixed or stale generations are labeled and never presented as current.
4. `degraded_lanes_never_implied_complete` — A degraded lane never appears in a `complete` result.
5. `provider_overlay_always_disclosed` — A contributing provider overlay is always disclosed.
6. `replay_preserves_lane_and_locality_labels` — Replay and support exports preserve lane and locality labels.
7. `hidden_scope_counts_disclosed` — Hidden in-scope counts are disclosed, not silently dropped.

## Consumer Projection

Search labels every contribution lane; docs recall labels lanes and locality; the
AI context pack labels lanes with ranking-or-chunking reasons; diagnostics shows
hidden-scope counts and degraded lanes; support exports and replay packets
preserve the labels the user saw; and any result below `complete` is visibly
labeled rather than presented as complete.

## Boundary

Raw query bodies, document bodies, raw embeddings, raw provider payloads,
credentials, exact scores, and exact token or cost amounts never cross the
support boundary — only labels, classes, and counts do. The validator rejects any
export that contains obvious credential or secret material.
