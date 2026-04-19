# Search readiness, ranking-reason, hidden-scope, result-truth, and deep-link drift vocabulary

This document is the **copy-guidance companion** to ADR 0014
(`docs/adr/0014-search-readiness-ranking-result-truth.md`) and the
boundary schema at
`schemas/search/search_result_truth.schema.json`. It exists so
product, docs, and support writers describe search readiness,
ranking reason, hidden scope, result truth, partial-truth cause,
semantic fallback, and deep-link drift with **one shared
vocabulary** across the command palette, full search, symbol jump,
docs search, graph overlay, AI-explanation overlay, and
support-export surfaces.

If this document and the ADR disagree, the ADR wins and this file
MUST be updated in the same change. Adding a token requires a
schema bump and a parallel ADR / decision-row update.

The three companion artifacts are:

- [`docs/adr/0014-search-readiness-ranking-result-truth.md`](../adr/0014-search-readiness-ranking-result-truth.md)
  — the frozen decision. Read it first if you are changing
  anything mechanical.
- [`schemas/search/search_result_truth.schema.json`](../../schemas/search/search_result_truth.schema.json)
  — the boundary schema every downstream surface reads.
- [`artifacts/search/result_truth_labels.yaml`](../../artifacts/search/result_truth_labels.yaml)
  — the machine-readable vocabulary and worked-examples corpus.

## Who reads this document

- **Product writers** who decide the palette / full-search /
  symbol-jump / docs-search / graph-overlay / AI-overlay /
  support-export copy. Use the frozen summary sentences below —
  do not mint your own.
- **Docs writers** who describe search readiness in release notes,
  runbooks, and onboarding. Use the same sentences so the docs
  pack and the surfaces agree word-for-word.
- **Support writers** who produce support-export summaries and
  service-health copy. Support exports are downstream; they
  quote canonical owners, they do not re-derive them.

## One vocabulary, one owner per result family

Every row rendered on a search / navigation surface resolves to
exactly one canonical owner. Copy-only shadow surfaces are
forbidden: a downstream surface **quotes the canonical owner by
id**, it does not re-derive the payload. This is the
"no-shadow-search" rule and mirrors the no-shadow rules in
ADR-0008 and ADR-0013.

| Result family                         | Canonical owner                |
|---------------------------------------|--------------------------------|
| Palette command rows                  | `palette_command_registry`     |
| Workspace lexical and graph rows      | `search_indexer`               |
| Symbol jump rows                      | `symbol_jump_resolver`         |
| Graph overlay rows                    | `graph_authority`              |
| Docs search rows                      | `docs_pack_registry`           |
| AI-explanation overlay rows           | `derived_explanation_session`  |
| Support export rows                   | `support_export_pipeline`      |
| Deep-link resolutions                 | `deep_link_resolver`           |

## Readiness-state copy

Surfaces MUST render the state that matches the readiness the
canonical owner emits. The table below pins the human-readable
copy writers use when a state is surfaced; the machine vocabulary
is frozen on `readiness_state` in the schema.

| State                | Writer-facing label                                    | When to use                                                                                   |
|----------------------|--------------------------------------------------------|-----------------------------------------------------------------------------------------------|
| `not_indexed`        | "This scope has not been indexed yet."                 | The surface has not started indexing this scope.                                              |
| `hot_set_ready`      | "Indexed the hot set — working on the rest."           | The hot / recent set is ready and returning exact matches while the tail is still being built.|
| `partial_index`      | "Still indexing — results are partial."                | Partway through a full index build; results skew to what is ready.                            |
| `warm_index`         | "Warm index — results are stable but not freshest."    | Full index that has not been refreshed against recent churn.                                  |
| `fully_indexed`      | (no chip — this is the baseline)                       | Only state that renders a result set as complete for the requested scope.                     |
| `stale_index`        | "Showing stale index — reindex is pending or failed."  | Reindex has failed or is overdue. Must carry `stale_index_served` as a partial-truth cause.   |
| `reindexing`         | "Reindexing — results are partial."                    | A reindex is in progress; intermediate results may shift until it completes.                  |
| `index_unavailable`  | "Index is unavailable — results are degraded."         | The surface cannot reach its indexer at all.                                                  |

**Rule**: any state other than `fully_indexed` MUST carry a
non-null `stale_or_partial_explanation` whose
`human_readable_summary` is one of the frozen sentences in the
[Frozen summary sentences](#frozen-summary-sentences) section
below.

## Result-truth-class copy

`result_truth_class` is the most important label on a search row.
Do not paraphrase it; surfaces and docs MUST use the same four
labels.

| Class       | Writer-facing label                      | What it means                                                                                                          |
|-------------|------------------------------------------|------------------------------------------------------------------------------------------------------------------------|
| `exact`     | (no chip — this is the baseline)         | Proven from the current graph or lexical index for this scope.                                                          |
| `imported`  | "Imported" chip                          | Fact pulled from a mirrored pack, docs pack, manifest, or provider overlay. Cite the anchor.                            |
| `heuristic` | "Heuristic" chip                         | Probabilistic, naming-similarity, or semantic-embedding derived. Must not render alone as authoritative.                |
| `hybrid`    | "Hybrid" chip                            | Combines at least two of the above and MUST enumerate its contributors in `ranking_reason_classes`.                     |

**Rule**: `heuristic` and `hybrid` rows MUST carry a non-null
`stale_or_partial_explanation`. `hybrid` rows MUST list ≥ 2
ranking reasons. A semantic contributor may appear only when
`semantic_fallback_state` = `semantic_available_as_supplement`.

## Hidden-result disclosure copy

When any rows are hidden — for any reason, including
zero-hits-for-this-user under admin policy — surfaces MUST emit a
disclosure. Never silently drop the count.

- If `hidden_result_count` is exact, say
  "`<n>` results hidden".
- If `hidden_result_count` is an upper bound
  (`count_is_approximate = true`), say
  "up to `<n>` results hidden".
- Enumerate reasons using the labels in the table; do not
  paraphrase them.

| `hidden_scope_reason`            | Writer-facing reason                                 |
|----------------------------------|------------------------------------------------------|
| `trust_state_excludes_surface`   | Workspace is restricted.                             |
| `policy_narrows_source`          | Admin policy narrows the source.                     |
| `sparse_scope_excludes_root`     | Scope does not include the root.                     |
| `outside_loaded_scope`           | Outside the loaded scope.                            |
| `excluded_by_user_filter`        | Excluded by your filter.                             |
| `client_scope_excludes_surface`  | Not available on this surface.                       |
| `provider_overlay_unauthorised`  | Connected-provider authorisation is missing.         |
| `pack_quarantined`               | Pack is quarantined.                                 |
| `redaction_narrowed`             | Redacted under the active redaction class.           |
| `remote_shard_unreachable`       | Some remote results are not reachable.               |

## Deep-link drift copy

Every deep-link resolution on bookmark restore, session restore,
AI evidence packet, support export, or cross-surface navigation
carries one drift state.

| `deep_link_drift_state`      | Writer-facing label                           |
|------------------------------|-----------------------------------------------|
| `resolved_exact`             | (no chip — the baseline)                      |
| `resolved_remapped`          | "Link followed a rename."                     |
| `resolved_ambiguous`         | "More than one target matched the saved link."|
| `target_missing`             | "Target no longer exists."                    |
| `target_moved`               | "Target moved."                               |
| `target_renamed`             | "Target renamed."                             |
| `target_branch_drifted`      | "Saved link is on a different branch."        |
| `target_policy_blocked`      | "Target is blocked by policy."                |
| `target_scope_excluded`      | "Target is outside the loaded scope."         |
| `index_not_ready_for_target` | "Index is not ready for this target yet."     |
| `unresolvable`               | "This link cannot be resolved."               |

**Rule**: `resolved_*` states MUST carry a non-null
`resolved_target_ref_opaque`. `resolved_remapped` MUST list the
predecessor link ids in `remap_chain_refs`. Never silently repaint
a drifted link as exact.

## Semantic-fallback copy

The product falls back to lexical-only ranking by default. Writers
MUST NOT describe semantic scoring as always-on.

| `semantic_fallback_state`              | Writer-facing label                                  |
|----------------------------------------|------------------------------------------------------|
| `semantic_unavailable_lexical_only`    | "Semantic scoring is unavailable — showing lexical results only." |
| `semantic_degraded_lexical_preferred`  | "Semantic scoring is degraded — lexical results are preferred." |
| `semantic_available_as_supplement`     | "Semantic suggestions are shown alongside exact matches — suggestions are not authoritative." |
| `semantic_disabled_by_policy`          | "Semantic scoring is disabled by admin policy."      |
| `semantic_not_applicable`              | (no chip — state is not applicable to this surface)  |

## Frozen summary sentences

Every `stale_or_partial_explanation.human_readable_summary`
emitted on a packet MUST quote one of the sentences below verbatim.
Surfaces MAY add scope-specific detail only through typed fields
(the `hidden_scope_reason`, `partial_truth_cause`, etc.
enumerations), never by rewriting the summary sentence.

| Copy id                                          | Sentence                                                                                       |
|--------------------------------------------------|------------------------------------------------------------------------------------------------|
| `copy.partial_index_still_loading`               | "Still indexing the rest of the workspace — these results are partial."                        |
| `copy.manifest_only_pending_materialisation`     | "Some scopes are listed but not yet loaded — results from those scopes are not included."      |
| `copy.cache_only_offline`                        | "Showing cached results — live index is not reachable right now."                              |
| `copy.outside_loaded_scope`                      | "Results outside the loaded scope are not included."                                           |
| `copy.semantic_unavailable_lexical_only`         | "Semantic scoring is unavailable — showing lexical results only."                              |
| `copy.remote_shard_unreachable`                  | "Some remote results are not reachable — results are partial."                                 |
| `copy.provider_overlay_unreachable`              | "Connected-provider results are unavailable — showing workspace results only."                 |
| `copy.policy_blocked_shard`                      | "Admin policy narrows the source of results — some scopes are excluded."                       |
| `copy.freshness_floor_unmet`                     | "Results are below the required freshness — showing them with a staleness note."               |
| `copy.stale_index_served`                        | "Showing stale index — reindex is pending or failed."                                          |
| `copy.heuristic_only`                            | "Some results are heuristic — graph confirmation is pending."                                  |
| `copy.hybrid_semantic_supplement`                | "Showing exact matches plus semantic suggestions — suggestions are not authoritative."         |

## Per-surface copy guidance

### Command palette (`command_palette`)

- `result_truth_class` is always `exact` (palette rows are the
  palette registry's own canonical rows) or `imported` (when the
  palette federates an external command).
- `semantic_fallback_state` is always `semantic_not_applicable`.
- The palette MUST render a typed chip for every non-`exact`,
  non-`fully_indexed` state; do not stuff the state into the
  subtitle.

### Full search (`full_search`)

- `scope_filter_class` must name the exact scope the query ran
  against (`current_root`, `named_workset`, `sparse_slice`,
  `full_workspace`, `policy_limited_view`).
- A non-zero `hidden_result_count` MUST be visible on the primary
  surface — never only in a tooltip.
- If semantic scoring is degraded or off, quote
  `copy.semantic_unavailable_lexical_only` (or the degraded
  variant).

### Symbol jump (`symbol_jump`)

- When the graph authority has not confirmed the relation, use
  `structural_fallback` as a ranking reason and mark the row
  `heuristic`.
- Semantic scoring MUST NOT drive a symbol-jump row on its own.

### Docs search (`docs_search`)

- `citation_anchor_refs` MUST be non-empty; the row quotes the
  docs-pack anchor. Writers do not paraphrase the anchor copy.
- If the docs pack is mirrored (not project-authoritative), use
  `imported` and render a "Mirrored" chip sourced from the docs
  badge vocabulary (ADR-0013).

### Graph overlay (`graph_overlay`)

- Render `result_truth_class` next to every edge / node rendered
  in the overlay — this is the primary surface on which
  `exact` / `imported` / `heuristic` / `hybrid` is visually
  distinguished.
- Heuristic edges MUST NOT be drawn with the same line weight as
  exact edges.

### AI-explanation overlay (`ai_explanation_overlay`)

- `citation_anchor_refs` MUST be non-empty. Every AI row cites the
  anchor it reads from.
- Hybrid rows: cite every contributor in the inline citation
  footer, not only the first.
- `semantic_embedding_neighbour` may appear ONLY when
  `semantic_fallback_state = semantic_available_as_supplement`.

### Support export (`support_export`)

- Quote packets from the canonical owners above; never synthesise
  new copy for the export.
- Preserve `stale_or_partial_explanation`,
  `hidden_result_disclosure`, and every `ranking_reason_classes`
  entry — the support desk reads them as-is.

## Writing about service degradation

When the semantic service is off, `service_contract_state` on the
paired service-health badge will be `degraded` or `unavailable`
(see ADR-0013). Search surfaces MUST:

1. Engage `semantic_fallback_state = semantic_unavailable_lexical_only`
   (or the `semantic_degraded_lexical_preferred` variant).
2. Add `semantic_service_unavailable` to the packet's
   `partial_truth_causes`.
3. Quote `copy.semantic_unavailable_lexical_only`.

Do not paint the surface as broken. Lexical-only results are a
first-class mode, not a failure mode.

## Writing about admin policy narrowing

Admin policy may narrow semantic scoring, narrow source classes,
raise the freshness floor, or hide scopes. Writers MUST:

1. Emit `hidden_scope_reason = policy_narrows_source` on any
   hidden rows the policy removes.
2. Use the neutral sentence in `copy.policy_blocked_shard`:
   "Admin policy narrows the source of results — some scopes are
   excluded." Do not blame the user; do not offer a workaround.

## Non-goals

- Writers MUST NOT invent surface-local readiness labels, result-
  truth chips, ranking-reason strings, hidden-scope reasons, or
  drift labels. Extending the vocabulary requires an ADR change.
- Writers MUST NOT quote raw query strings, raw document bodies,
  raw symbol definitions, or raw URLs back to the user through a
  search row. Those never cross the RPC boundary and never belong
  in a summary string.
