# Search result-identity, ranking-reason, action-binding, and export packet — proof packet

## Summary

The stable result-truth packet at
`artifacts/search/m4/search_result_truth_packet.json` proves that every
search-result row emits the four v24 objects (`SearchResultRef`,
`RankingReason`, `SearchActionBinding`, and the wrapping
`SearchExportPacket`) and that every consumer projection preserves the
closed fact-label vocabulary, the action-binding fallback policy, the
captured-vs-live status, and the hidden / omitted scope counters.

The packet covers all six fact-label tokens with one row each:

| Row | Fact label | Result kind | Fallback mode | History policy |
|---|---|---|---|---|
| `row:exact:workspace_file` | `exact` | `workspace_file` | `direct` | `record_history_entry` |
| `row:context_promoted:recent_target` | `context_promoted` | `recent_target` | `direct` | `reuse_existing_entry` |
| `row:semantic:symbol` | `semantic` | `symbol` | `direct` | `record_history_entry` |
| `row:partial_index:docs_anchor` | `partial_index` | `docs_anchor` | `rerun_live_query` | `reuse_existing_entry` |
| `row:withheld_latency:imported_artifact` | `withheld_latency` | `imported_artifact` | `open_captured_snapshot` | `suppress_for_captured_replay` |
| `row:policy_hidden:graph_entity` | `policy_hidden` | `graph_entity` | `policy_narrowed` | `policy_forbids_history` |

## How the packet protects dedupe lineage

The `row:exact:workspace_file` row collapses a lexical-filename match and
a lexical-path match into one visible row. Both contributing source
strata are preserved in `dedupe_lineage` with their canonical anchors so
support and AI consumers can inspect why a single visible row may
represent multiple candidate sources. The validator emits
`dedupe_dropped_source_stratum` or `dedupe_dropped_canonical_anchor`
blockers if either field collapses; the narrowed corpus case
`fixtures/search/m4/result_truth_packet/dedupe_anchor_dropped_blocks_stable.json`
exercises this drift.

## How the packet protects withheld-latency truth

The `row:withheld_latency:imported_artifact` row is shown from a captured
remote-cache snapshot because the live remote lane exceeded its latency
budget. The row carries:

- `freshness: captured_snapshot`
- `confidence: heuristic`
- `withheld_candidate_note` describing the latency budget breach
- `fallback_mode: open_captured_snapshot`
- `history_policy: suppress_for_captured_replay`

The validator blocks promotion with `dedupe_dropped_fallback_mode` if a
withheld-latency row collapses its fallback mode to `direct`; the
narrowed corpus case
`fixtures/search/m4/result_truth_packet/withheld_latency_direct_fallback_blocks_stable.json`
exercises this drift.

## How the packet protects policy-hidden honesty

The `row:policy_hidden:graph_entity` row narrows a graph entity by trust
posture. The hidden-member count is preserved (one hidden member, carried
on the scope-counters block), but the hidden member list is not exposed.
The action binding sets `fallback_mode: policy_narrowed` and
`history_policy: policy_forbids_history`. The validator blocks promotion
with `missing_withheld_candidate_note` if a policy-hidden row drops its
withheld-candidate note.

## How the packet protects consumer projections

The packet binds six required consumer projections (`search_shell`,
`docs_help`, `ai_context_inspector`, `cli_headless`, `support_export`,
`release_proof_index`). Each projection must preserve the same packet id,
the result refs, the closed ranking-reason vocabulary, the action-binding
fallback mode and history policy, the captured-vs-live status, the
hidden / omitted counts, and JSON export — while excluding raw query
text, raw bodies, secrets, and ambient credentials. The narrowed corpus
case
`fixtures/search/m4/result_truth_packet/captured_vs_live_dropped_blocks_stable.json`
exercises a projection that drops captured-vs-live status; the validator
emits both `consumer_projection_drift` and `captured_vs_live_dropped`
blockers.

## How the packet protects scope counters

The packet carries seven scope counters (visible, loaded, all-matching,
hidden-by-current-scope, hidden-by-policy, hidden-by-remote-cache,
omitted-by-latency-budget). The validator emits
`hidden_omitted_counts_dropped` on any consumer projection that drops
the counters. Export and share flows preserve every counter so support
and automation can tell whether the user saw a snapshot, a rerun, or a
narrowed scope.

## Source contract refs

- Schema: `schemas/search/search_result_truth_packet.schema.json`
- Reviewer doc: `docs/search/m4/result-identity-ranking-reasons-and-export-packets.md`
- Rust implementation: `crates/aureline-search/src/result_truth_packet/mod.rs`
- Fixture corpus: `fixtures/search/m4/result_truth_packet/`
- Certified-archetype scorecards (latency budgets for `withheld_latency`):
  `artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml`
