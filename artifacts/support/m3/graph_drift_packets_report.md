# Graph drift packets and support-export parity baseline report

This artifact is the reviewer-facing baseline rendering of the graph
drift report produced by the
[`drift_packets`](../../../crates/aureline-graph/src/drift_packets/mod.rs)
module from the protected corpus under
[`/fixtures/graph/m3/drift_packets/`](../../../fixtures/graph/m3/drift_packets/).
It records the consumer surface, readiness state, freshness class,
scope class, data-lane lineage, derived drift indicator, downgrade
label, and open-gap classes for every graph-backed beta surface
decision in the corpus. The report stays metadata-safe: it never
carries raw private material or ambient authority, and every row is
drawn from the closed drift-packet vocabularies.

Schema: `schemas/graph/drift_packet.schema.json`
(record kind `graph_drift_report_record`, version 1).
Reviewer doc: [`docs/support/m3/graph_drift_packets_beta.md`](../../../docs/support/m3/graph_drift_packets_beta.md).
Corpus manifest:
[`fixtures/graph/m3/drift_packets/manifest.yaml`](../../../fixtures/graph/m3/drift_packets/manifest.yaml).

## Matrix rows

| Packet ID | Consumer surface | Subject ref | Readiness | Freshness | Scope | Lineage | Drift indicator | Downgrade label | Open-gap classes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `drift:ai_context:fallback:only` | `ai_context` | `graph:symbol:greet_fn` | `partial` | `unknown` | `full_local` | `fallback_search_lineage` | `fallback_only` | `degraded_to_fallback_search_only` | `fallback_truth_only` |
| `drift:ai_context:inferred:lineage_skew` | `ai_context` | `graph:provider:issue_42` | `ready` | `authoritative` | `full_local` | `inferred_derived_lineage` | `lineage_skew` | `yellow_lineage_skew` | `lineage_pending` |
| `drift:nav:exact_local:aligned` | `navigation` | `graph:symbol:greet_fn` | `ready` | `authoritative` | `full_local` | `exact_local_graph_lineage` | `aligned` | `none` | `none` |
| `drift:nav:out_of_scope:blocked` | `navigation` | `graph:workset:other_repo` | `out_of_scope` | `unknown` | `out_of_scope` | `out_of_scope_lineage` | `blocked_by_scope` | `red_drift_blocks_beta_row` | `drift_blocked` |
| `drift:nav:warming:warning` | `navigation` | `graph:symbol:greet_fn` | `warming` | `warming` | `full_local` | `warming_provider_lineage` | `warming_warning` | `yellow_freshness_skew` | `freshness_pending` |
| `drift:review:imported:lineage_skew` | `review` | `graph:file:vendor_acme_lib_rs` | `ready` | `imported` | `full_local` | `imported_provider_lineage` | `lineage_skew` | `yellow_lineage_skew` | `lineage_pending` |
| `drift:support_export:partial:scope_skew` | `support_export` | `graph:symbol:greet_fn` | `partial` | `authoritative` | `sparse_local` | `partial_scope_lineage` | `scope_skew` | `yellow_scope_skew` | `scope_pending` |
| `drift:support_export:stale:warning` | `support_export` | `graph:symbol:greet_fn` | `stale` | `stale` | `full_local` | `stale_cached_lineage` | `stale_warning` | `yellow_freshness_skew` | `freshness_pending` |

## Per-consumer-surface summary

| Consumer surface | Packets | Aligned | Drift | Fallback only | Downgrade required |
| --- | --- | --- | --- | --- | --- |
| `navigation` | 3 | 1 | 2 | 0 | 2 |
| `ai_context` | 2 | 0 | 2 | 1 | 2 |
| `review` | 1 | 0 | 1 | 0 | 1 |
| `support_export` | 2 | 0 | 2 | 0 | 2 |

## Per-lineage summary

| Data-lane lineage | Packets | Aligned | Drift | Downgrade required |
| --- | --- | --- | --- | --- |
| `exact_local_graph_lineage` | 1 | 1 | 0 | 0 |
| `imported_provider_lineage` | 1 | 0 | 1 | 1 |
| `inferred_derived_lineage` | 1 | 0 | 1 | 1 |
| `partial_scope_lineage` | 1 | 0 | 1 | 1 |
| `stale_cached_lineage` | 1 | 0 | 1 | 1 |
| `warming_provider_lineage` | 1 | 0 | 1 | 1 |
| `out_of_scope_lineage` | 1 | 0 | 1 | 1 |
| `fallback_search_lineage` | 1 | 0 | 1 | 1 |

## Open gaps

- `drift:ai_context:fallback:only` (`fallback_truth_only`): no graph
  evidence is available for the subject; AI context degrades to
  fallback search.
- `drift:ai_context:inferred:lineage_skew` (`lineage_pending`): AI
  context relied on inferred relations rather than exact local graph
  edges.
- `drift:nav:out_of_scope:blocked` (`drift_blocked`): subject is
  outside the active scope; navigation must widen scope before
  serving the row.
- `drift:nav:warming:warning` (`freshness_pending`): graph provider
  is warming; navigation should wait or downgrade to fallback search.
- `drift:review:imported:lineage_skew` (`lineage_pending`): review
  surface drew on imported provider lineage; reviewer must inspect
  provenance before treating the row as exact local truth.
- `drift:support_export:partial:scope_skew` (`scope_pending`):
  support export covered only a sparse slice of the workspace.
- `drift:support_export:stale:warning` (`freshness_pending`): support
  export drew on a stale cached snapshot.

## Safety baseline

- `raw_private_material_excluded = true` on every packet and on the
  report.
- `ambient_authority_excluded = true` on every packet and on the
  report.
- `destructive_resets_present = false` on every packet.
- `preserves_user_authored_files = true` on every packet and on every
  evidence-export projection.
- Every `evidence_export` projection preserves the readiness token,
  freshness token, scope label, lineage label, consumer-surface
  label, and `envelope_packet_ref` so the in-product chrome and the
  exported packet quote the same truth.

## Out of scope

- Live runtime measurement of per-surface latency or throughput.
- Cross-tenant ticket routing — the report is consumed locally by
  the support-export pipeline and the chrome.
- Adding new readiness states, freshness classes, scope classes,
  data-lane lineages, drift indicators, downgrade labels, open-gap
  classes, or consumer surfaces without updating the schema, the
  Rust module, the reviewer doc, this report, and the protected
  corpus together.
