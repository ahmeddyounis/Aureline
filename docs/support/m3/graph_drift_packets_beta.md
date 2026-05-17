# Graph drift packets and support-export parity (beta)

This document is the reviewer-facing contract for the graph drift
packet beta projection. It defines the closed vocabularies, the
acceptance contract a fixture row must satisfy, the support-export
parity rules, and the relationship to the live alpha graph fact cue
packet projection that backs navigation, AI context, review, and
support export surfaces in-product.

A graph drift packet binds **one graph consumer surface** to **one
alpha graph fact cue packet ref** and captures the packet-level
**readiness state**, **freshness class**, **scope class**, and
**data-lane lineage** in a single metadata-safe exportable object.
The drift indicator is re-derived from the
`(readiness, freshness, scope, lineage)` quadruple so prose cannot lie
about drift, and a closed `downgrade_label` downgrades a failing row
without inventing new vocabulary.

## Why this exists

Before this projection, blocked users had to infer graph freshness
from timing alone — "it feels slow", "the row looks old" — and support
operators had to re-run graph producers to find out what the
in-product chrome rendered. A drift packet captures the readiness
state, freshness class, scope class, and data-lane lineage of one
graph-backed surface decision in one exportable object so:

- The chrome can render the same fields the support packet carries.
- The support and review surfaces can reference the same packet fields
  users see in-product, without re-running graph producers.
- Operators can audit graph drift from the support export instead of
  guessing from latency.

## Source contract

- Schema: [`/schemas/graph/drift_packet.schema.json`](../../../schemas/graph/drift_packet.schema.json)
  (records `graph_drift_packet_record`, `graph_drift_report_record`,
  version 1).
- Crate module:
  [`/crates/aureline-graph/src/drift_packets/mod.rs`](../../../crates/aureline-graph/src/drift_packets/mod.rs).
- Fixture corpus:
  [`/fixtures/graph/m3/drift_packets/`](../../../fixtures/graph/m3/drift_packets/)
  with [`manifest.yaml`](../../../fixtures/graph/m3/drift_packets/manifest.yaml).
- Baseline report:
  [`/artifacts/support/m3/graph_drift_packets_report.md`](../../../artifacts/support/m3/graph_drift_packets_report.md).
- Alpha cue projection backing the packet:
  [`/crates/aureline-graph/src/readiness/mod.rs`](../../../crates/aureline-graph/src/readiness/mod.rs).
- Beta readiness projection sharing the consumer-surface vocabulary:
  [`/crates/aureline-graph/src/readiness/beta.rs`](../../../crates/aureline-graph/src/readiness/beta.rs).

## Closed vocabularies

### `consumer_surface`

| Token | Meaning |
| --- | --- |
| `navigation` | Symbol jump, breadcrumbs, quick open, graph-backed navigation. |
| `ai_context` | AI context selection, context inspector, prompt evidence handoff. |
| `review` | Diff review, impact review, review seeds. |
| `support_export` | Support packets, evidence exports, escalation bundles. |

### `readiness_state`

`ready`, `hot_set_ready`, `partial`, `warming`, `stale`,
`unavailable`, `out_of_scope`. Mirrors the alpha
`GraphQueryReadiness` vocabulary so the drift packet always quotes the
same readiness token the alpha envelope observed.

### `freshness_class`

`authoritative`, `hot_set`, `warming`, `cached`, `stale`, `replayed`,
`imported`, `unknown`. Derived from the alpha graph `Freshness`
vocabulary, with `unknown` reserved for fallback-search or
empty-envelope packets that have no freshness frame to quote.

### `scope_class`

`full_local`, `sparse_local`, `full_managed`, `sparse_managed`,
`mixed_local_and_managed`, `out_of_scope`. The scope class is the
input the surface fed to the alpha graph query envelope; the drift
packet preserves it on disk so the export reader knows whether the
row covered the full workspace or only a sparse slice.

### `data_lane_lineage`

`exact_local_graph_lineage`, `imported_provider_lineage`,
`inferred_derived_lineage`, `partial_scope_lineage`,
`stale_cached_lineage`, `warming_provider_lineage`,
`out_of_scope_lineage`, `fallback_search_lineage`. The lineage class
is the closed truth-source label for the packet as a whole; it is
derived from the strongest (lowest-strength-index) truth lane present
in the cue packet so fallback noise does not mask exact lineage.

### `drift_indicator`

`aligned`, `freshness_skew`, `scope_skew`, `lineage_skew`,
`stale_warning`, `warming_warning`, `blocked_by_scope`,
`fallback_only`. The indicator is **re-derived** by the evaluator from
the `(readiness, freshness, scope, lineage)` quadruple; the evaluator
refuses any packet whose declared indicator disagrees with the derived
indicator, so a fixture cannot lie about drift.

### `downgrade_label`

`none`, `red_drift_blocks_beta_row`, `yellow_freshness_skew`,
`yellow_scope_skew`, `yellow_lineage_skew`,
`degraded_to_fallback_search_only`,
`stale_corpus_blocks_release_candidate`. Aligned rows must pin
`none`; every non-aligned row must pin one of the other labels. The
evaluator also re-derives the label from the drift indicator so the
mapping cannot drift silently.

### `open_gap_class`

`none`, `freshness_pending`, `scope_pending`, `lineage_pending`,
`evidence_export_pending`, `fallback_truth_only`, `drift_blocked`. A
downgraded row must record at least one non-none open gap; an aligned
row must declare none.

## Acceptance contract

The evaluator
[`GraphDriftPacketEvaluator`](../../../crates/aureline-graph/src/drift_packets/mod.rs)
refuses a row when any of these contracts are broken:

1. `drift_indicator` disagrees with the indicator derived from
   `(readiness_state, freshness_class, scope_class, data_lane_lineage)`.
2. `drift_indicator = aligned` and any non-none `downgrade_label`,
   any non-none `open_gap_class`, or vice versa.
3. `downgrade_label` does not match the label derived from
   `drift_indicator`.
4. `evidence_export` drops `preserves_readiness_token`,
   `preserves_freshness_token`, `preserves_scope_label`,
   `preserves_lineage_label`, `preserves_consumer_surface_label`, or
   `preserves_envelope_packet_ref`.
5. `evidence_export` admits raw private material or ambient authority,
   or drops `preserves_user_authored_files`.
6. The packet declares `safety.destructive_resets_present = true`.
7. The packet drops `safety.preserves_user_authored_files`, admits
   `safety.raw_private_material_excluded = false`, or admits
   `safety.ambient_authority_excluded = false`.
8. The packet's `references` block drops the pinned doc, schema, or
   report ref.

The corpus is also refused unless:

- Every required `consumer_surface` is seeded by at least one packet.
- Every required `data_lane_lineage` is seeded by at least one packet
  as the packet's `data_lane_lineage`.
- At least one packet declares a non-aligned `drift_indicator` so the
  drift contract is exercised by a fixture.

## Support-export parity

The same packet fields the in-product chrome renders travel through
the support packet without re-running graph producers:

- `consumer_surface`, `readiness_state`, `freshness_class`,
  `scope_class`, `data_lane_lineage`, and `drift_indicator` are pinned
  on the packet record itself.
- `evidence_export` declares that the support packet preserves the
  readiness token, freshness token, scope label, lineage label, and
  consumer-surface label.
- `envelope_packet_ref` points back at the alpha
  [`GraphFactCuePacket`](../../../crates/aureline-graph/src/readiness/mod.rs)
  so support and review surfaces can re-join evidence without
  re-deriving the truth.
- The packet record is metadata-only: it never carries raw private
  material or ambient authority, and the safety baseline forbids
  destructive resets and preserves user-authored files.

## Out of scope

- Cross-tenant ticket routing. Drift packets are consumed locally by
  the support-export pipeline and the chrome.
- Live runtime measurement of per-surface latency or throughput. The
  drift packet quotes the readiness and freshness tokens the alpha
  envelope already observed.
- Adding new readiness states, freshness classes, scope classes,
  data-lane lineages, drift indicators, downgrade labels, open-gap
  classes, or consumer surfaces without updating the schema, the Rust
  module, this reviewer doc, the baseline report, and the protected
  corpus together.
