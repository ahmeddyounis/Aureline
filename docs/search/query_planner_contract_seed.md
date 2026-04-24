# Unified search query-planner, shard topology, and result-fusion contract seed

This document is the **prose companion** to three machine-readable
artifacts:

- [`artifacts/search/shard_rows.yaml`](../../artifacts/search/shard_rows.yaml)
  — shard-topology registry (quick open, full search, symbol jump,
  palette, docs, semantic lookup, graph overlay, AI explanation,
  saved query, support export).
- [`schemas/search/result_fusion_record.schema.json`](../../schemas/search/result_fusion_record.schema.json)
  — boundary schema for the planner pass, the per-row fusion
  record, the shard snapshot, the streaming frame, and the planner
  audit event.
- [`schemas/search/saved_query_and_scope_binding.schema.json`](../../schemas/search/saved_query_and_scope_binding.schema.json)
  — boundary schema for saved queries, query-history entries,
  scope bindings, and search deep-link bindings.

and to the frozen result-truth vocabulary at
[`docs/search/search_readiness_vocabulary.md`](./search_readiness_vocabulary.md)
/ [`artifacts/search/result_truth_labels.yaml`](../../artifacts/search/result_truth_labels.yaml)
/ [`schemas/search/search_result_truth.schema.json`](../../schemas/search/search_result_truth.schema.json),
which remain the authoritative copy-and-label contract for every
rendered search row. This seed does **not** redefine those
vocabularies; it pins the planner vocabulary the rendered rows
project into.

Worked planner passes live under
[`fixtures/search/planner_cases/`](../../fixtures/search/planner_cases/).

If this document and the ADR (`docs/adr/0014-search-readiness-ranking-result-truth.md`)
disagree, the ADR wins and this file MUST be updated in the same
change. Adding a planner stage, shard row, duplicate-collapse
rationale, ranking-weight class, or backlink kind requires a schema
bump, a registry-row update, and a parallel ADR / decision-row
update.

## Why the planner needs its own contract

The command palette, quick open, full search, symbol jump, docs
search, graph overlay, AI-explanation overlay, and support export
surfaces all fire the same family of planner passes against the
same indexes and overlays. If each surface invents its own planner
— its own candidate-acquisition pass, its own shard-selection
rules, its own cancellation semantics, its own duplicate-collapse
rationale, its own ranking-explanation shape, its own notion of
"which planner pass produced this result set" — then:

- the palette, quick open, full search, symbol jump, and docs
  search surfaces drift into incompatible query logic;
- a heuristic row on one surface and an exact row on another
  surface citing the "same" index cannot be reconciled;
- a saved query, a search deep link, or a support export cannot
  reconstruct "which planner pass and which shard authorities
  produced the row I was looking at";
- partial, stale, and not-yet-indexed state leaks into blank
  result sets on some surfaces and into typed partial-truth
  disclosures on others.

This seed closes that gap by fixing **one planner-stage vocabulary,
one shard-topology registry, one result-fusion record shape, and
one saved-query / scope-binding vocabulary** that every lane
projects into before any production planner code lands. A later
parity audit compares emitted `result_fusion_record` instances
field-for-field without inventing planner-local semantics.

## Planner pipeline (frozen stage vocabulary)

Every planner pass MUST declare a lane grouping from
`artifacts/search/shard_rows.yaml#lane_groupings` and MUST execute
the stages the grouping declares, in order. Stages absent from the
lane grouping's ordering MUST NOT appear on the pass; stages
present in the ordering that did not run MUST appear with
`stage_outcome_class` set to `skipped_not_applicable`, `cancelled`,
or `failed_refused`.

| Stage id                      | Responsibility                                                                                                                                              |
|-------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `candidate_acquisition`       | Generate candidate rows from each shard the scope filter admits. Every candidate binds to exactly one shard row from the shard registry.                    |
| `shard_selection`             | Narrow candidates to shards the query family allows; honour admin-policy narrowing, trust-state exclusions, and client-scope filtering; emit hidden-scope disclosures. |
| `cancellation`                | Cooperative-cancellation checkpoint. Every shard whose `cancellation_participation` is `cooperative_cancellation` MUST deliver an ack at this stage.        |
| `streaming_partial_results`   | Stream partial results from `streams_partial_with_marker` shards. Every streamed frame carries a `partial_truth_cause`.                                     |
| `duplicate_merge`             | Collapse duplicates across shards into one fused row per canonical entity. Every collapsed input retains its shard row id and a typed collapse rationale.   |
| `ranking_explanation`         | Emit `ranking_reason_classes` for each fused row. Hybrid rows enumerate ≥ 2 contributors.                                                                   |
| `final_presentation`          | Project the fused row into a `search_result_packet_record`. This stage owns the readiness / freshness / truth-class / hidden-scope ceilings.                |

The planner's cancellation and streaming stages are **shared
across surfaces** by design: a cancellation message into the
palette's planner pass cancels every shard the pass consulted on
that user's behalf, and a streaming frame emitted by the full-
search planner reuses the same `streaming_frame_record` shape as
the graph-overlay planner. Surface-local variants of cancellation
or streaming are forbidden.

## Shard topology

The shard registry at
`artifacts/search/shard_rows.yaml` freezes one row per index /
overlay / pack / projection the planner may consult. Every row
declares:

- `shard_class` — the shard family (`per_root_lexical`,
  `hot_set_lexical`, `per_root_graph`, `semantic_embedding`,
  `docs_pack`, `provider_overlay`, `imported_precomputed`,
  `palette_command_registry`, `saved_query_and_history`,
  `support_export_projection`);
- `lane_class` — the logical lane (`quick_open_lane`,
  `full_search_lane`, `symbol_jump_lane`, `palette_command_lane`,
  `docs_search_lane`, `semantic_lookup_lane`, `graph_overlay_lane`,
  `ai_explanation_lane`, `support_export_lane`);
- `authority_class` — re-exported from ADR-0005 (every search /
  navigation shard is `derived_knowledge` today; `durable_artifact_snapshot`
  names packs and support-export projections);
- `accelerator_or_source` — posture relative to the rendered truth
  claim: `truth_source`, `hybrid_contributor`, or
  `accelerator_only`;
- `truth_class_rule` — the maximum `result_truth_class` a fused
  row drawn from this shard may render as: `max_exact`,
  `max_imported`, `max_heuristic`, or `hybrid_only`;
- `canonical_owner` — the canonical owner from
  `artifacts/search/result_truth_labels.yaml#source_of_truth_ownership`
  so the no-shadow-search rule applies at the shard level;
- `freshness_class`, `readiness_state_ceiling`,
  `remote_or_imported_posture`, `cancellation_participation`,
  `streaming_posture`, and `invalidation_producers` — the shard's
  freshness / readiness / cancellation / streaming / invalidation
  contract.

Lanes the registry covers today (each lane MAY gain additional
shard rows with an additive-minor bump):

- **Per-root lexical** — full-search authoritative lexical /
  structural index.
- **Hot-set lexical** — quick-open recent-file / recent-edit bias.
  Readiness ceiling is `hot_set_ready`; the planner MUST NOT
  render `fully_indexed` with only this shard.
- **Per-root graph** — symbol-jump graph authority plus a
  structural-fallback lane.
- **Semantic embedding** — default-off supplement. Can only
  contribute to `hybrid` rows; a fused row whose only contributor
  is a semantic shard is non-conforming.
- **Docs pack** — first-party and mirrored / third-party docs
  packs. Rows render `imported`; `citation_anchor_refs` MUST be
  non-empty.
- **Provider overlay** — connected-provider resources. Accelerator
  only; a row rendered imported with only this shard MUST cite
  the overlay, and a provider-unauthorised row emits
  `provider_overlay_unauthorised` rather than silently dropping.
- **Imported / precomputed** — mirrored index packs built offline.
  Accelerator only; rows render `imported`.
- **Palette command registry** — exact-truth owner for palette
  rows. Semantic scoring is not applicable.
- **Graph overlay public projection** — public graph projection
  for the overlay surface.
- **AI explanation derived session** — hybrid-only contributor.
- **Saved-query / query-history registry** — exact-truth owner
  for durable saved-query / search-deep-link rows (see
  [Saved queries, scope bindings, and deep-link backlinks](#saved-queries-scope-bindings-and-deep-link-backlinks)).
- **Support-export projection** — read-only quoting of the
  canonical owners; never mints new copy.

## Result fusion

The fusion stage emits one `result_fusion_record` per visible
canonical entity the planner pass produced (and one per disclosed-
hidden row family). Every record binds:

- `planner_pass_id_ref` and `result_set_id_ref` — the authoritative
  search run identity (see below);
- `canonical_entity_id` — the stable id every collapsed input
  shares (that is why the planner fused them);
- `result_truth_class` + `ranking_reason_classes` — frozen from
  `schemas/search/search_result_truth.schema.json`;
- `shard_contributions[]` — one entry per contributing shard,
  with a typed role (`primary_truth_source`,
  `supplementary_contributor`, `accelerator_only`,
  `duplicate_collapsed_into_primary`, `hidden_by_policy_or_scope`,
  `unreachable_disclosed`) and a `ranking_weight_class`;
- `duplicate_collapse_rationale` — exactly one from the frozen
  vocabulary (e.g. `same_canonical_entity_id`, `same_graph_node_id`,
  `same_docs_anchor_id`, `prefix_subsumed_by_longer_match`,
  `no_collapse_unique_row` for rows with a single contributor);
- `readiness_state`, `freshness_class`, `semantic_fallback_state`,
  `hidden_result_disclosure`, and `stale_or_partial_explanation`
  — all frozen from the result-truth vocabulary;
- `citation_anchor_refs` — mandatory when `result_truth_class` is
  `imported` or when any contributing shard is an imported / docs
  pack;
- `search_result_packet_ref` — the opaque ref to the rendered
  `search_result_packet_record` this fusion record projects into.

### Readiness / truth-class ceilings across contributors

The planner MUST downgrade the fused row's `readiness_state` to
the **lowest** `readiness_state_ceiling` among its contributors,
and downgrade the fused row's `result_truth_class` to the **most
restrictive** `truth_class_rule` among its contributors. A fused
row whose only contributor is an `accelerator_only` shard is
non-conforming and MUST be refused with
`fusion_denial_reason = accelerator_cited_as_sole_exact`.

### Partial, stale, and not-yet-indexed states remain visible

The streaming frame, the partial-truth explanation, and the
hidden-result disclosure together guarantee the PRD rule
"Partial, stale, or not-yet-indexed states remain visible rather
than hidden inside a blank result set":

- Every streaming frame the planner emits carries a
  `streaming_frame_kind` and the rows it added, superseded, or
  finalised.
- Every fused row whose `readiness_state` is not `fully_indexed`,
  whose `result_truth_class` is `heuristic` or `hybrid`, whose
  `semantic_fallback_state` is below `semantic_available_as_supplement`,
  or whose `hidden_result_disclosure` is non-null carries a typed
  `stale_or_partial_explanation` whose `human_readable_summary`
  quotes the frozen copy.
- A fused row drawn only from the `hot_set_lexical` shard MUST
  carry `hot_set_only` in `partial_truth_causes` and quote
  `copy.partial_index_still_loading` or `copy.heuristic_only`.

### Remote or imported indexes are accelerators, not undisclosed truth

Every shard row whose `accelerator_or_source` is
`accelerator_only` or whose `remote_or_imported_posture` is
`imported_pack` / `provider_remote` projects into an `imported`
or `hybrid` fused row, never `exact`. The planner MUST:

- refuse `fusion_denial_reason = accelerator_cited_as_sole_exact`
  when an accelerator row is cited as the only contributor to an
  exact fused row;
- refuse `fusion_denial_reason = imported_pack_missing_citation_anchor`
  when an imported-pack contributor is missing a citation anchor.

## Query-session, planner-pass, and result-set identity

Four opaque ids pin **one authoritative search run** across
surfaces and support captures:

- `query_session_id` — reserved on `search_session_record`
  (`schemas/search/search_result_truth.schema.json`). Opens once
  per user-visible query; multiple planner passes may reference
  the same session.
- `planner_pass_id` — reserved on `planner_pass_record`. Every
  planner invocation mints exactly one.
- `result_set_id` — reserved on `planner_pass_record`. Every
  visible result set (including streaming partial result sets)
  mints exactly one; a corrective replace frame updates rows
  under the same `result_set_id` so downstream backlinks do not
  fork.
- `shard_snapshot_id` — reserved on `shard_snapshot_record`.
  Every shard snapshot the planner observes mints exactly one.

A durable artifact that needs to reconstruct "the planner pass
the user saw" (saved query, query-history entry, search deep
link, bookmark, AI-evidence anchor, support-export quote) names
all four ids via the `planner_pass_ref` shape in
`schemas/search/saved_query_and_scope_binding.schema.json`.

This is how support captures, saved-query re-runs, and parity
audits can point at one authoritative search run rather than a
hand-assembled replay.

## Saved queries, scope bindings, and deep-link backlinks

`schemas/search/saved_query_and_scope_binding.schema.json` fixes
the durable-artifact layer:

- `saved_query_record` — the saved query; never carries the raw
  query body. Binds a `query_classification`, a `lane_grouping_id`,
  and a `scope_binding_id_ref` to the last authoritative planner
  pass (`last_planner_pass_ref`).
- `query_history_entry_record` — minted for every planner pass
  the user observes. The durable backlink the saved-query
  registry, the search-deep-link registry, the AI-evidence
  overlay, and the support-export pipeline point at when they
  want to name "the planner pass the user saw".
- `scope_binding_record` — binds a set of shard rows, a
  `scope_filter_class`, a `lane_grouping_id`, and a
  `client_scopes` list to a saved query / deep link / AI-evidence
  anchor / support export. Every binding declares a
  `binding_scope_source_class` (`user_authored`,
  `imported_from_pack`, `generated_by_support_export`,
  `generated_by_ai_evidence`, `generated_by_session_restore`,
  `generated_by_parity_audit`); `imported_from_pack` bindings
  MUST cite at least one anchor.
- `search_deep_link_binding_record` — wraps a
  `search_deep_link_record` (`schemas/search/search_result_truth.schema.json`)
  with the planner-pass + scope-binding ids that minted it. The
  drift-state resolution stays on the deep-link record; the
  binding record owns the planner-pass pin.

Durable artifacts never invent a separate truth model. A saved
query does not carry a result set; it carries a pointer into the
planner / shard / packet vocabulary. The re-run is the audit-
trail: `saved_query_audit_event_record` events (`saved_query_rerun_opened`,
`saved_query_rerun_refused`, etc.) carry a typed
`saved_query_denial_reason` that fails closed. A saved query
whose shard row no longer exists, whose lane grouping was
retired, whose schema version is lagging, or whose policy epoch
has expired is refused, never silently reinterpreted.

## Cancellation, streaming, duplicates, and ranking explanation

- **Cancellation.** Every shard whose `cancellation_participation`
  is `cooperative_cancellation` MUST deliver an ack at the
  cancellation stage. `best_effort_cancellation` shards MAY drop
  in-flight work without an ack (reserved for stateless, cheap
  rows). `uninterruptible_short_prefix` shards (palette prefix
  match, saved-query registry) are not relied on for mid-frame
  cancellation. A missing ack from a cooperative shard refuses
  the session; the planner MUST NOT silently leak leftover work
  across sessions.
- **Streaming.** `streams_partial_with_marker` shards contribute
  to partial streaming frames; every frame names the fused rows
  added, superseded, or finalised. `corrective_replace_frame` is
  the **only** frame kind that invalidates previously streamed
  rows; the planner MUST re-emit or withdraw each superseded id,
  not silently drop it.
- **Duplicate merge.** Every fused row carries a typed
  `duplicate_collapse_rationale`. Single-contributor rows use
  `no_collapse_unique_row`. Cross-shard collapses name the
  identity axis (`same_graph_node_id`, `same_docs_anchor_id`,
  `same_path_identity`, `same_symbol_declaration_anchor`,
  `same_provider_resource_id`, `same_saved_query_binding`,
  `prefix_subsumed_by_longer_match`,
  `cross_shard_name_and_path_match`).
- **Ranking explanation.** Every contribution binds to one
  `ranking_weight_class` (`primary_signal`, `tie_breaker`,
  `recency_bias`, `frequency_bias`, `anchor_boost`,
  `semantic_supplement`, `structural_fallback_bias`,
  `policy_required_inclusion`). The fused row MUST enumerate the
  reasons that actually contributed; per-surface free-form
  reason strings are forbidden.

## What parity audits compare

A later parity audit between palette, quick open, full search,
symbol jump, docs search, semantic lookup, graph overlay,
AI-explanation overlay, and support export iterates the fixtures
under `fixtures/search/planner_cases/` and checks:

- every `planner_pass_record` names a `lane_grouping_id` from
  `artifacts/search/shard_rows.yaml#lane_groupings` and emits the
  stages the grouping declares;
- every `shard_candidate_contribution` binds to a shard row in
  the registry;
- every `result_fusion_record` enforces the readiness-ceiling,
  truth-class-ceiling, accelerator, imported-pack-citation, and
  hybrid-must-enumerate-two-contributors rules;
- every `streaming_frame_record` labelled
  `corrective_replace_frame` names the rows it supersedes;
- every durable artifact (`saved_query_record`,
  `query_history_entry_record`, `scope_binding_record`,
  `search_deep_link_binding_record`) points back at a
  `planner_pass_id` + `result_set_id` pair;
- no record carries raw query bodies, raw document bodies, raw
  symbol definitions, or raw URLs.

## Non-goals for this seed

- This seed does **not** ship a live indexer, a live ranking
  engine, a live semantic embedding service, or a live saved-
  query storage layer. It fixes the vocabulary those layers will
  project into when they land.
- This seed does **not** mint final ranking weights. Each
  contribution binds to a `ranking_weight_class` vocabulary; the
  actual numeric weights are planner-internal and never cross the
  RPC boundary.
- This seed does **not** replace the result-truth vocabulary. The
  rendered row contract remains
  `schemas/search/search_result_truth.schema.json`; the fusion
  record is the planner-internal provenance that produced the
  rendered packet.

## How to extend this seed

1. Adding a new shard row: bump `shard_rows_schema_version` in
   `artifacts/search/shard_rows.yaml`, add the row under
   `shard_rows`, list it in the matching `lane_groupings` entry,
   and add a worked fixture under `fixtures/search/planner_cases/`.
2. Adding a new planner stage, ranking-weight class, duplicate-
   collapse rationale, shard-contribution role, fusion denial
   reason, or streaming-frame kind: bump
   `result_fusion_schema_version` in
   `schemas/search/result_fusion_record.schema.json`, extend the
   frozen enum, and update this document and the ADR in the same
   change.
3. Adding a new saved-query status, scope-binding class, or
   deep-link backlink class: bump `saved_query_schema_version`
   in `schemas/search/saved_query_and_scope_binding.schema.json`,
   extend the frozen enum, and update this document and the ADR
   in the same change.
4. Never add a surface-local readiness / truth / ranking-reason
   / hidden-scope / partial-truth / deep-link-drift label. Those
   live on `docs/search/search_readiness_vocabulary.md` and a
   schema / ADR change is required to extend them.
