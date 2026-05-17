# Semantic graph readiness and exact-vs-imported fact label beta baseline report

This artifact is the reviewer-facing baseline rendering of the
graph-readiness beta report produced by the
[`beta`](../../../crates/aureline-graph/src/readiness/beta.rs)
module from the protected corpus under
[`/fixtures/graph/m3/readiness_truth/`](../../../fixtures/graph/m3/readiness_truth/).
It records the consumer surface, claimed fact lane, observed
envelope lane, alpha readiness state, derived claim-alignment state,
downgrade label, and open-gap classes for every graph consumer claim
in the beta corpus. The report stays metadata-safe: it never carries
raw private material or ambient authority, and every row is drawn
from the closed readiness-beta vocabularies.

Schema: `schemas/search/graph_readiness_beta.schema.json`
(record kind `graph_readiness_beta_report_record`, version 1).
Reviewer doc: [`docs/search/m3/graph_readiness_beta.md`](../../../docs/search/m3/graph_readiness_beta.md).
Corpus manifest:
[`fixtures/graph/m3/readiness_truth/manifest.yaml`](../../../fixtures/graph/m3/readiness_truth/manifest.yaml).

## Matrix rows

| Case ID | Consumer surface | Subject ref | Claimed fact lane | Observed envelope lane | Observed readiness | Alignment state | Downgrade label | Open-gap classes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `case:ai_context:fallback:only` | `ai_context` | `graph:symbol:greet_fn` | `fallback_search_fact` | `fallback_search_fact` | `partial` | `weaker_claim_accepted` | `degraded_to_fallback_search_only` | `fallback_truth_only` |
| `case:ai_context:inferred:weaker_claim` | `ai_context` | `graph:provider:issue_42` | `inferred_graph_fact` | `inferred_graph_fact` | `ready` | `weaker_claim_accepted` | `yellow_fact_lane_partial` | `fact_lane_pending` |
| `case:ai_context:partial:weaker_claim` | `ai_context` | `graph:symbol:greet_fn` | `partial_graph_fact` | `partial_graph_fact` | `partial` | `weaker_claim_accepted` | `yellow_fact_lane_partial` | `fact_lane_pending` |
| `case:nav:exact_local:aligned` | `navigation` | `graph:symbol:greet_fn` | `exact_local_graph_fact` | `exact_local_graph_fact` | `ready` | `aligned` | `none` | `none` |
| `case:nav:exact_overclaim:blocked` | `navigation` | `graph:file:vendor_acme_lib_rs` | `exact_local_graph_fact` | `imported_graph_fact` | `stale` | `overclaim_blocked` | `red_blocks_beta_row` | `overclaim_blocked` |
| `case:nav:out_of_scope:blocked` | `navigation` | `graph:workset:other_repo` | `out_of_scope_graph_fact` | `out_of_scope_graph_fact` | `out_of_scope` | `weaker_claim_accepted` | `red_blocks_beta_row` | `consumer_surface_pending` |
| `case:nav:waiting:weaker_claim` | `navigation` | `graph:symbol:greet_fn` | `waiting_on_graph_provider` | `waiting_on_graph_provider` | `warming` | `weaker_claim_accepted` | `degraded_to_fallback_search_only` | `consumer_surface_pending` |
| `case:review:imported:aligned` | `review` | `graph:file:vendor_acme_lib_rs` | `imported_graph_fact` | `imported_graph_fact` | `stale` | `aligned` | `none` | `none` |
| `case:support_export:stale:aligned` | `support_export` | `graph:symbol:greet_fn` | `stale_graph_fact` | `stale_graph_fact` | `stale` | `aligned` | `none` | `none` |

## Per-fact-lane summary

| Observed envelope lane | Cases | Aligned | Weaker claim accepted | Overclaim blocked | Downgrade required |
| --- | --- | --- | --- | --- | --- |
| `exact_local_graph_fact` | 1 | 1 | 0 | 0 | 0 |
| `imported_graph_fact` | 2 | 1 | 0 | 1 | 1 |
| `inferred_graph_fact` | 1 | 0 | 1 | 0 | 1 |
| `partial_graph_fact` | 1 | 0 | 1 | 0 | 1 |
| `stale_graph_fact` | 1 | 1 | 0 | 0 | 0 |
| `waiting_on_graph_provider` | 1 | 0 | 1 | 0 | 1 |
| `out_of_scope_graph_fact` | 1 | 0 | 1 | 0 | 1 |
| `fallback_search_fact` | 1 | 0 | 1 | 0 | 1 |

## Per-consumer-surface summary

| Consumer surface | Cases | Aligned | Weaker claim accepted | Overclaim blocked | Downgrade required |
| --- | --- | --- | --- | --- | --- |
| `navigation` | 4 | 1 | 2 | 1 | 3 |
| `ai_context` | 3 | 0 | 3 | 0 | 3 |
| `review` | 1 | 1 | 0 | 0 | 0 |
| `support_export` | 1 | 1 | 0 | 0 | 0 |

## Open gaps

- `case:ai_context:inferred:weaker_claim` (`fact_lane_pending`): AI
  context downgrades because an `inferred_graph_fact` does not
  satisfy the surface's aligned-lane set
  (`exact_local_graph_fact`, `imported_graph_fact`).
- `case:ai_context:partial:weaker_claim` (`fact_lane_pending`):
  partial subscope coverage downgrades for the same reason.
- `case:nav:waiting:weaker_claim` (`consumer_surface_pending`):
  navigation cannot claim exact while the graph provider is warming
  and falls back to fallback search.
- `case:nav:out_of_scope:blocked` (`consumer_surface_pending`):
  navigation blocks the row in red until the user adjusts scope.
- `case:ai_context:fallback:only` (`fallback_truth_only`): no graph
  evidence is available; AI context degrades to fallback search.
- `case:nav:exact_overclaim:blocked` (`overclaim_blocked`):
  navigation tried to claim an exact local graph fact while the cue
  packet only carries imported evidence; the beta row is blocked in
  red.

## Safety baseline

- `raw_private_material_excluded = true` on every case and the
  report.
- `ambient_authority_excluded = true` on every case and the report.
- `destructive_resets_present = false` on every case.
- `preserves_user_authored_files = true` on every case and on every
  evidence-export projection.
- Every `evidence_export` projection preserves `fact_lane_label`,
  `readiness_token`, `consumer_surface_label`, and
  `envelope_packet_ref` so the truth label travels through support
  packets without re-running graph producers.

## Out-of-scope

- Live runtime measurement of per-surface latency or throughput.
- Cross-tenant ticket routing — the report is consumed locally by
  the support-export pipeline and the chrome.
- Adding new downgrade labels, open-gap classes, fact lanes,
  consumer surfaces, or claim-alignment states without updating the
  schema, the Rust module, the reviewer doc, this report, and the
  protected corpus together.
