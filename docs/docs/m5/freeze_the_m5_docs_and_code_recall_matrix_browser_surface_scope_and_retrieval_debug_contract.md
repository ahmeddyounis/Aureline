# M5 Docs and Code-Recall Matrix, Browser-Surface Scope, and Retrieval-Debug Contract

This document is the contract for the frozen M5 matrix that qualifies four
docs-and-code-understanding lanes. The matrix is the canonical M5 control source
for this lane: dashboards, docs, Help/About surfaces, and support exports ingest
the checked-in packet rather than cloning status text.

- Record kind: `freeze_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract`
- Schema: [`schemas/docs/freeze-the-m5-docs-and-code-recall-matrix-browser-surface-scope-and-retrieval-debug-contract.schema.json`](../../../schemas/docs/freeze-the-m5-docs-and-code-recall-matrix-browser-surface-scope-and-retrieval-debug-contract.schema.json)
- Canonical support export: [`artifacts/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract/support_export.json`](../../../artifacts/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract/support_export.json)
- Summary artifact: [`artifacts/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract.md`](../../../artifacts/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract.md)
- Fixtures: [`fixtures/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract/`](../../../fixtures/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract/)
- Producer: `aureline_docs::current_stable_m5_docs_and_code_recall_matrix_export`

## Lanes

| Lane | Qualification | Source contract |
| --- | --- | --- |
| `docs_semantic_recall` | Stable | [`schemas/docs/semantic_recall_boundary_truth.schema.json`](../../../schemas/docs/semantic_recall_boundary_truth.schema.json) |
| `codebase_explainer` | Stable | [`schemas/graph/codebase_explainer_packet.schema.json`](../../../schemas/graph/codebase_explainer_packet.schema.json) |
| `retrieval_debug` | Stable | [`schemas/search/retrieval_inspector.schema.json`](../../../schemas/search/retrieval_inspector.schema.json) |
| `scoped_browser_surface` | Beta | [`schemas/docs/docs_browser_truth_packet.schema.json`](../../../schemas/docs/docs_browser_truth_packet.schema.json) |

Each lane row binds a qualification class to its evidence requirement, required
evidence packet refs, downgrade triggers, rollback posture, source contracts, and
the consumer surfaces that must project the lane's qualification truth.

## Track invariant

Docs recall stays mirror-aware, codebase explainers stay cited, retrieval-debug
stays available, and browser surfaces stay narrow, attributable, and return-path
safe. The `trust_review` block encodes these as hard invariants — all must hold
for the matrix to validate:

- `docs_recall_mirror_aware` — pinned, signed mirrors outrank live vendor docs.
- `explainers_cited_with_source_class` and `confidence_class_preserved` — no
  heuristic claim is presented as a verified graph fact.
- `open_raw_open_source_escape_preserved` — every derived result keeps an
  open-raw / open-source escape.
- `ranking_reasons_explicit` and `retrieval_debug_available` — every recall
  result is inspectable.
- `browser_surface_narrow_and_attributable` and `browser_handoff_return_path_safe`
  — general web-mode and browser-runtime claims stay out of scope.
- `no_source_looks_more_authoritative_than_proven`,
  `downgrade_narrows_instead_of_hides`, and
  `stale_or_underqualified_blocks_promotion`.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the affected lane. The supported
downgrade triggers are `proof_stale`, `policy_blocked`, `mirror_offline`,
`source_version_mismatch`, `freshness_expired`, `trust_narrowing`,
`scope_expansion_unqualified`, and `upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract/)
show a mirror-offline recall narrowing and a held browser surface; both remain
valid packets because narrowing is explicit, not hidden.

## Boundary

Raw document bodies, raw source files, raw query text, raw provider payloads,
credentials, and live vendor-doc snapshots never cross this boundary. The packet
carries only metadata, qualification truth, and contract references.
