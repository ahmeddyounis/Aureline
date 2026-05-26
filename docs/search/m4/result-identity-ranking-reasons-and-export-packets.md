# Result identity, ranking reasons, scope counters, and fact labels

This is the stable contract every search-result row must speak when it
projects into product chrome, docs/help, AI-context, CLI/headless, support
export, and the release proof index. The runtime owner is the
`aureline_search::result_truth_packet` module.

Search rows now emit four stable objects:

- `SearchResultRef` — stable result identity (`result_id`, `result_kind`,
  canonical object refs, anchor/span, source stratum, snapshot/commit/worktree
  ref, freshness, confidence, dedupe lineage). Two ranking passes that
  surface the same row MUST mint the same `result_id`; two rows for the
  same path on two strata MUST receive distinct ids and preserve every
  contributing stratum in `dedupe_lineage`.
- `RankingReason` — structured `Why this result?` object: closed
  `fact_label` token (`exact`, `context_promoted`, `semantic`,
  `partial_index`, `withheld_latency`, `policy_hidden`), promoted signals,
  suppressed signals, `tie_break_class`, withheld-candidate note, and
  partiality note.
- `SearchActionBinding` — `open_target_ref`, alternate behaviors,
  required surface capabilities, `fallback_mode`, and `history_policy`.
  Action behaviour MUST NOT be inferred from whichever UI control launched
  the row.
- `SearchExportPacket` (renamed `SearchResultTruthPacket` in code to avoid
  collision with the saved-query export wrapper) — preserves the captured
  rows, the hidden/omitted scope counters, the captured-vs-live status,
  and the consumer projections that re-render the packet verbatim.

## The fact-label vocabulary

The closed `FactLabelClass` vocabulary is the v24 promise to every
consumer:

| Token | Emitted when |
|---|---|
| `exact` | Row is a direct exact match. |
| `context_promoted` | Row was promoted by recents / pinned / hot-set bias. |
| `semantic` | Row came from semantic / vector retrieval. |
| `partial_index` | Row came from a partial or warming index lane. |
| `withheld_latency` | A faster live lane exceeded its latency budget; row is shown from a captured snapshot. |
| `policy_hidden` | One or more candidates were narrowed by trust / policy posture. |

Consumers MUST preserve every token across product, CLI/headless, AI, and
support paths — withheld and policy-hidden states cannot silently drop
while the row still renders.

## Dedupe lineage and contributing strata

When the runtime collapses two or more candidates into one visible row, the
`dedupe_lineage` field preserves every contributing source stratum and its
canonical anchor. Support and AI consumers inspect this list when they
need to explain why one visible row may represent multiple candidate
sources. The validator emits `dedupe_dropped_source_stratum` and
`dedupe_dropped_canonical_anchor` blockers if the field collapses.

## Captured-vs-live status and history policy

`captured_vs_live` is one of `live`, `captured_snapshot`,
`rerun_replaced_snapshot`, or `narrowed_scope_rerun`. Action bindings on
withheld-latency rows MUST set `fallback_mode = open_captured_snapshot`
and `history_policy = suppress_for_captured_replay`. Action bindings on
policy-hidden rows MUST set `fallback_mode = policy_narrowed` and
`history_policy = policy_forbids_history`. The validator emits
`dedupe_dropped_fallback_mode` and `captured_vs_live_dropped` blockers when
either invariant is violated.

## Scope counters

The `scope_counters` field carries `visible_rows`, `loaded_rows`,
`all_matching_rows`, and four hidden / omitted counters
(`hidden_by_current_scope_rows`, `hidden_by_policy_rows`,
`hidden_by_remote_cache_rows`, `omitted_by_latency_budget_rows`). Export
and share flows MUST preserve every counter so support and automation can
tell whether the user saw a snapshot, a rerun, or a narrowed scope.

## Required consumer projections

Every claimed stable packet MUST carry a preserved projection for each of:

- `search_shell` (the product result pane)
- `docs_help` (the docs/help surface)
- `ai_context_inspector` (the AI context picker / inspector)
- `cli_headless` (the headless CLI emitter)
- `support_export` (the support bundle)
- `release_proof_index` (the release proof index)

Each projection must preserve the same packet id, the result refs (id +
dedupe lineage), the closed ranking-reason vocabulary, the action-binding
fallback mode and history policy, the captured-vs-live status, the
hidden/omitted counts, and support JSON export — all while excluding raw
query text, raw bodies, secrets, and ambient credentials. The validator
emits `missing_consumer_projection`, `consumer_projection_drift`,
`captured_vs_live_dropped`, and `hidden_omitted_counts_dropped` blockers
otherwise.

## Promotion state

`SearchResultTruthPromotionState` is one of `stable`,
`narrowed_below_stable`, or `blocks_stable`. The packet's stored
promotion state must match the state derived from the validation
findings; the validator emits `promotion_state_mismatch` otherwise.

## Source contract refs

- Schema: `schemas/search/search_result_truth_packet.schema.json`
- Rust implementation: `crates/aureline-search/src/result_truth_packet/mod.rs`
- Reviewer artifact: `artifacts/search/m4/result-identity-ranking-reasons-and-export-packets.md`
- Checked-in packet: `artifacts/search/m4/search_result_truth_packet.json`
- Fixture corpus: `fixtures/search/m4/result_truth_packet/`
