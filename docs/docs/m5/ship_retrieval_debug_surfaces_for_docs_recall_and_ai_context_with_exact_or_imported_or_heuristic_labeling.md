# Retrieval-Debug Surfaces (docs, recall, AI context)

This document is the contract for the M5 retrieval-debug inspector — the surface
that lets a reader see *why* a result was retrieved and *how trustworthy* it is.
Three retrieval lanes are explained through one shared vocabulary:

- a **docs search** entry — a lexical / symbol hit from the docs index;
- a **semantic recall** entry — an embedding-backed recall hit;
- an **AI context** entry — a fragment that was assembled into a prompt context.

Every entry carries the same source/version/freshness/locality/confidence chip
set the other docs-recall lanes use, an explicit **derivation reason**, a
non-empty list of **ranking signals** (the ranking reasons), citation state, and
the open-raw / open-source escapes. Crucially, every entry carries one
**derivation label** — `exact`, `imported`, or `heuristic` — that tells the
reader whether the result is a verbatim match from a verified local source
(`exact`), came in through an imported / mirrored pack (`imported`), or is an
inferred / fuzzy / lexical-fallback match (`heuristic`). An export preserves the
lane / label / source / confidence / ranking-reason / escape truth that support,
AI evidence, and diagnostics surfaces ingest rather than cloning status text. The
retrieval-debug inspector, docs browser, semantic-recall panel, AI-context panel,
CLI/headless output, support exports, diagnostics, and Help/About all consume the
checked-in packet.

- Record kind: `retrieval_debug_surfaces_for_docs_recall_and_ai_context`
- Schema: [`schemas/docs/ship-retrieval-debug-surfaces-for-docs-recall-and-ai-context-with-exact-or-imported-or-heuristic-labeling.schema.json`](../../../schemas/docs/ship-retrieval-debug-surfaces-for-docs-recall-and-ai-context-with-exact-or-imported-or-heuristic-labeling.schema.json)
- Canonical support export: [`artifacts/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling/support_export.json`](../../../artifacts/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling/support_export.json)
- Summary artifact: [`artifacts/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling.md`](../../../artifacts/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling.md)
- Fixtures: [`fixtures/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling/`](../../../fixtures/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling/)
- Producer: `aureline_docs::current_stable_retrieval_debug_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_retrieval_debug_surfaces`

## The entries and their chips

`entries` is the set of retrieved-and-explained results for one query. Every
entry points at a `subject_ref`, carries a `lane` (`docs_search`,
`semantic_recall`, `ai_context`), a `subject_kind`, a `title`, a `headline`, and a
`chips` block — the five chips a consumer projects verbatim:

| Chip | Tokens |
| --- | --- |
| `source_class` | `workspace_code`, `dependency_source`, `graph_index`, `project_docs`, `generated_reference`, `mirrored_official_docs`, `imported_pack`, `ai_assembled_context` |
| `version_match` | `exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build` |
| `freshness` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`, `refresh_pending` |
| `locality` | `local`, `mirrored_pack`, `remote_helper`, `managed` |
| `confidence` | `high`, `medium`, `low`, `heuristic` |

Every packet must include at least one entry on each of the three lanes — a
partial set (docs without AI context, say) is `required_lane_missing` and blocks
promotion, so the inspector stays the full docs + recall + AI-context surface
rather than a slice that overstates coverage.

## The exact / imported / heuristic label

Each entry carries one `derivation_label`:

| Label | Meaning |
| --- | --- |
| `exact` | A verbatim, exact match from a verified local / pinned source. The only label that may back a high-confidence live claim without a downgrade. |
| `imported` | A match that came in through an imported / mirrored pack or external import. Must stay cited. |
| `heuristic` | An inferred, fuzzy, or lexical-fallback match. Must stay cited and may **never** be presented at high confidence. |

Each entry also carries an explicit `derivation_reason` that says *why* it earned
its label; an entry with no reason is `derivation_reason_missing` and blocks
promotion, so a label can never be presented unexplained.

The guardrails enforced by the validator:

- An `imported` or `heuristic` entry that is not cited is `entry_not_cited`.
- A `heuristic` entry presented at `high` confidence is
  `heuristic_label_looks_authoritative`.
- A non-current `version_match` presented as a confident live match (high
  confidence + `authoritative_live`) is `version_truth_collapsed`.

All three block promotion, so no imported, heuristic, or drifted result can read
as more authoritative than it is.

## Ranking reasons

Every entry carries a non-empty `ranking_signals` list — the ranking reasons. Each
signal is a `{signal_kind, contribution, weight_label, note}` entry
(`signal_kind`: `lexical_match`, `semantic_similarity`, `symbol_exact_match`,
`path_proximity`, `recency_boost`, `pinned_source_boost`, `freshness_penalty`,
`imported_source_penalty`, `heuristic_penalty`; `contribution`: `boost`,
`penalty`, `neutral`). An entry with no signals is `ranking_signals_missing`; a
signal with no note is `ranking_signal_note_missing`. Both block promotion, so the
inspector always shows *why* a result ranked where it did. Raw scores never cross
the boundary — `weight_label` carries a human-readable label only.

## Export, degradations, and promotion

The `export` mirrors each entry into a row that preserves lane, derivation label,
source class, confidence, citation state, ranking-signal count, and the escapes.
A row whose lane, label, source class, or confidence disagrees with its entry is a
`export_*_mismatch`; a missing or orphan row is `export_coverage_missing` /
`export_row_orphan`; dropping a preservation flag is `export_drops_preservation`.

`retrieval_degradations` carry a `severity` (`blocking`, `narrowing`, `advisory`)
and a class (`embedder_unavailable_lexical_fallback`, `mirror_offline_snapshot`,
`index_stale`, `imported_pack_unverified`, `partial_index`, `quarantined_pack`,
`broken_anchor`). A narrowing degradation moves the packet to
`narrowed_below_stable`; a blocking degradation (or any validation finding) moves
it to `blocks_stable`. An otherwise-clean set with only advisory degradations is
`stable`. The downgrade **narrows the claim — it does not hide the lane.**

`consumer_projections` record how each surface projects the set; every packet must
cover the retrieval-debug inspector, docs browser, semantic-recall panel,
AI-context panel, and support export, and every projection must preserve the
chips, lanes, derivation labels, ranking reasons, and escapes.

## Boundary

The packet is metadata only. Raw query text, raw document bodies, raw source
files, raw provider payloads, and credentials never cross the boundary; an export
that carries forbidden material is `raw_boundary_material_present` and blocks
promotion.
