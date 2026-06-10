# Docs and Code Semantic Recall: Query-Session Ledger, Ranking Reasons, and Provenance Export

This document is the contract for the M5 unified semantic-recall feature. A
recall is a ranked query over both the mirror-aware docs packs and the workspace
/ dependency code, scoped to one reader **session**. The session's queries are
recorded in a query-session ledger; the active ranking is one result row per
hit; and a provenance export preserves the source/confidence/citation truth that
support, AI evidence, and review surfaces ingest rather than cloning status text.
The docs browser, search shell, codebase-explainer panel, AI context, retrieval
inspector, CLI/headless output, support exports, and Help/About all consume the
checked-in packet.

- Record kind: `docs_and_code_semantic_recall_query_session_ledger`
- Schema: [`schemas/docs/implement-docs-and-code-semantic-recall-with-query-session-ledger-ranking-reasons-and-provenance-export.schema.json`](../../../schemas/docs/implement-docs-and-code-semantic-recall-with-query-session-ledger-ranking-reasons-and-provenance-export.schema.json)
- Canonical support export: [`artifacts/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export/support_export.json`](../../../artifacts/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export/support_export.json)
- Summary artifact: [`artifacts/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export.md`](../../../artifacts/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export.md)
- Fixtures: [`fixtures/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export/`](../../../fixtures/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export/)
- Producer: `aureline_docs::current_stable_semantic_recall_ledger_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_code_semantic_recall`

## The query-session ledger

`query_session_ledger.entries` is the ordered list of queries the reader issued
in one session. Each entry carries:

- `sequence` — 1-based, strictly increasing from 1;
- `query_digest_ref` / `query_label` — an opaque digest and a human label,
  **never** the raw query text;
- `refinement` — the relation to the prior query: `initial` (only the first
  entry), `narrowed`, `broadened`, `reformulated`, `pivoted_subject`;
- `subject_scope` — `docs_only`, `code_only`, or `docs_and_code`;
- `surfaced_result_ids` — the result rows the entry surfaced.

The first entry must be `initial` and no later entry may be; a surfaced result id
that is absent from the rows is an orphan. Both block promotion.

## The chip set and ranking reasons

Every `result_row` points at a `subject_kind` (`docs_node`, `code_symbol`,
`code_file`, `code_snippet`) and carries a `chips` block — the five chips a
consumer projects verbatim:

| Chip | Tokens |
| --- | --- |
| `source_class` | `project_docs`, `generated_reference`, `mirrored_official_docs`, `curated_knowledge_pack`, `support_runbook`, `extension_docs_pack`, `workspace_code`, `dependency_source` |
| `version_match` | `exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build` |
| `freshness` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`, `refresh_pending` |
| `locality` | `local`, `mirrored_pack`, `remote_helper`, `managed` |
| `confidence` | `high`, `medium`, `low`, `heuristic` |

Each row carries an explicit `ranking_reason` backed by a `ranking_signals`
breakdown — one or more `{signal, contribution, note}` entries (signals:
`lexical_overlap`, `semantic_similarity`, `symbol_exact_match`,
`graph_proximity`, `recency`, `pin_boost`, `authority_boost`, `path_affinity`;
contribution: `primary`, `supporting`, `minor`, `penalty`). A row with no reason
or no signals blocks promotion, so a ranking can never present an unexplained
order. Each row also carries `origin_query_sequence` (the ledger entry that first
surfaced it) and the `open_raw_escape_ref` / `open_source_escape_ref` escapes.

## Provenance and citation

Each row's inline `provenance` records the `pack_id_ref`, pin/signature state, a
`derivation` (`verbatim_node`, `extracted_snippet`, `derived_summary`,
`inferred_explanation`), and the `cited` flag. A derived or inferred result must
stay cited (`code_result_not_cited` otherwise), and an inferred explanation may
not be presented as `high` confidence (`inferred_result_looks_authoritative`
otherwise). These keep codebase explainers honest.

The `provenance_export` is the cited projection support and AI evidence surfaces
ingest. It declares `scope` and the `preserves_*` invariants, and carries one row
per result mirroring `source_class`, `confidence`, `derivation`, `cited`, and the
escapes. A provenance row whose source class or confidence disagrees with the
result row's chip, a result without a provenance row, or an export that drops a
preservation flag all block promotion — provenance can never quietly upgrade a
result.

## Promotion and downgrade

`promotion_state` is computed from the worst severity across the validation
findings and the attached `recall_degradations`:

- a `blocking` finding → `blocks_stable`;
- otherwise a `narrowing` finding → `narrowed_below_stable`;
- otherwise → `stable`.

Degradations (`embedder_unavailable_lexical_fallback`, `code_graph_stale`,
`mirror_offline_snapshot`, `partial_index`, `session_truncated`,
`quarantined_pack`, `broken_anchor`) carry their own severity, so a degraded but
honest recall narrows rather than hides. The fixtures show an embedder-offline
recall narrowing (`narrowed_below_stable`, no blocking findings) and two blocked
cases (`uncited_code_explainer`, `inferred_explanation_over_authoritative`).
`current_stable_semantic_recall_ledger_export` re-materializes the checked-in
packet and fails if the recorded promotion state drifts from the freshly computed
one, so a stale or under-attributed recall cannot be promoted silently.

## Boundary

Raw query text, raw document bodies, raw source files, raw provider payloads, and
credentials never cross this boundary. The packet carries only metadata, chip
truth, ranking reasons, provenance, finding summaries, and contract references;
`query_digest_ref` and `session_digest_ref` are opaque digests, never raw text.

## Out of scope

This feature does not broaden general web-mode or browser-runtime claims beyond
the narrow docs/review/light-edit surfaces qualified in M5. Browser handoff and
scoped browser-surface qualification stay in their own contracts.
