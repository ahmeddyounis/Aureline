# Result identity, ranking reasons, and row-level partiality

The lexical query path in `aureline-search` decides *which* workspace files
match a query and *how those rows are sorted*. This contract decides *how each
row is explained* to the user — and how that explanation survives projection
through the search shell, quick open, support exports, and CLI replay.

The runtime owner is the `aureline_search::results` module. Companion
artifacts:

- [`/crates/aureline-search/src/results/identity.rs`](../../crates/aureline-search/src/results/identity.rs)
  — `ResultIdentity`, `RankingReasonClass`, `ResultPartialityClass`, the
  builders the lexical query path calls.
- [`/fixtures/search/result_identity_cases/`](../../fixtures/search/result_identity_cases/)
  — worked records covering the ready, warming/partial, stale, and
  generated-artifact-deprioritized cases.
- [`/crates/aureline-search/tests/result_identity_cases.rs`](../../crates/aureline-search/tests/result_identity_cases.rs)
  — fixture-driven coverage that loads every JSON case and asserts the
  identity packet matches the live projection.

## What the row identity owns

Every search row materialized through `aureline_search::lexical::query::run_query`
carries a `ResultIdentity`:

| Field | Meaning |
|---|---|
| `result_id` | Stable, deterministic URN-style id of the form `wsearch:{workspace_id}:{source_class_token}:{relative_path}`. Surfaces MUST quote this verbatim when persisting selection, exporting a support bundle, or reopening a row from a deep link. |
| `workspace_id`, `relative_path` | Trimmed echoes of the inputs so a support reviewer can verify the id without parsing the URN. |
| `source_class` | Which lexical lane produced the row (`lexical_filename` vs. `lexical_path`). |
| `match_kind` | Strongest reason the row matched (the lexical match-kind taxonomy). |
| `ranking_reasons` | Ordered list of `RankingReasonClass` tokens explaining why the row ranked where it did. |
| `partiality_class` | Row-level partiality (`authoritative` / `warming` / `partial` / `stale` / `unavailable`). Travels on each row so a row from a degraded provider keeps its caveat after sorting, pagination, and dedup. |

Two rows for the same path on different lanes (filename vs. path) MUST receive
distinct `result_id`s — the source-class token is part of the URN. One row
materialized in two ranking passes MUST receive the same `result_id`; the
fixture-driven test pins this property.

## Ranking-reason vocabulary

`RankingReasonClass` is the closed vocabulary surfaces quote when explaining a
row. M1 ships only the lexical lanes; future semantic / symbol / graph lanes
own their own tokens.

| Token | Emitted when |
|---|---|
| `exact_basename_match` | Basename equals the (normalized) query. |
| `prefix_basename_match` | Basename starts with the query. |
| `substring_basename_match` | Basename contains the query but did not match exactly or as a prefix. |
| `substring_path_match` | Workspace-relative path contains the query and the basename did not match. |
| `generated_artifact_deprioritized` | Row carries a generated-artifact lineage hint. Surfaces SHOULD route edits to the canonical source rather than treating this row as the primary edit target. Always paired with one of the match-kind reasons above. |
| `partial_coverage_caveat` | Active provider readiness is anything other than `ready`. Always paired with one of the match-kind reasons above. |

The list is ordered: the match-kind reason comes first, followed by
`generated_artifact_deprioritized` (when applicable), then
`partial_coverage_caveat` (when applicable). Fixtures rely on this order.

## Partiality vocabulary

`ResultPartialityClass` is a strict subset of `ReadinessClass` plus an
`unavailable` token reserved for support-replay records:

| Token | Source readiness | Row chrome |
|---|---|---|
| `authoritative` | `ready` | No row caveat. |
| `warming` | `warming` | Surface `Warming` badge directly on the row. |
| `partial` | `partial` | Surface `Partial` badge directly on the row. |
| `stale` | `stale` | Surface `Stale` badge directly on the row. |
| `unavailable` | `unavailable` / `out_of_scope` | Captured-snapshot replay only — live sessions never emit visible rows for these readinesses. |

Surfaces MUST NOT collapse `warming` and `partial` into a generic loading
badge. Both tokens stay visible to the user; the [search readiness vocabulary
contract](./search_readiness_vocabulary.md) and the [search query session
contract](./search_query_session_contract.md) require the same disambiguation.

## Live-shell consumer

The protected-row consumer is the workspace lexical search shell.
`crates/aureline-shell/src/search_shell/state.rs` projects the canonical
`ResultIdentity` into the chrome-facing
`WorkspaceSearchSurfaceResultIdentity`, which carries the `result_id`, the
ranking-reason tokens, the partiality class token, the `Authoritative` /
`Warming` / `Partial` / `Stale` badge, and the `must_show_row_caveat` flag.
The card is the same record support bundles and replay tools read; the chrome
quotes it directly without re-deriving labels.

## Failure drill

The `warming_partial_label_identity` fixture is the failure drill. It seeds a
`partially_ready` workspace whose lexical scan has streamed at least one row.
The contract requires the row to:

1. Keep the URN-style `result_id` so the row survives a re-render;
2. Carry both `prefix_basename_match` and `partial_coverage_caveat` as
   ordered ranking reasons;
3. Project `partiality_class = partial` and `partiality_row_badge = Partial`
   so the user sees the partiality chip directly on the row instead of a
   generic spinner.

Surfaces that drop or rename any of those tokens fail the fixture-driven
test.

## Out of scope

This contract does not introduce semantic ranking, fuzzy boost weighting,
provider attribution, or hidden-result classes — those live in the broader
[search query session contract](./search_query_session_contract.md) and the
[search explainability panel contract](./search_explainability_contract.md)
and remain pre-implementation in M1. Lexical rows quote only the closed
vocabulary above.
