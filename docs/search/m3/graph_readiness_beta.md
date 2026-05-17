# Semantic graph readiness and exact-vs-imported fact label beta

This reviewer doc is the contract for the semantic graph readiness
beta. The beta promotes one closed readiness and fact-lane vocabulary
across the graph consumer surfaces (`navigation`, `ai_context`,
`review`, `support_export`) so a single graph query envelope cannot
be projected as one truth label on one surface and a quietly stronger
label on another.

Each `graph_readiness_beta_case_record` binds one consumer surface
to:

- one `claimed_fact_lane` from the closed list
  (`exact_local_graph_fact`, `imported_graph_fact`,
  `inferred_graph_fact`, `partial_graph_fact`, `stale_graph_fact`,
  `waiting_on_graph_provider`, `out_of_scope_graph_fact`,
  `fallback_search_fact`) — the lane the surface intends to render or
  export;
- one `observed_envelope_lane` from the same closed list — the lane
  the underlying alpha graph fact cue packet actually computed;
- one `observed_readiness` from the alpha readiness vocabulary
  (`ready`, `hot_set_ready`, `partial`, `warming`, `stale`,
  `unavailable`, `out_of_scope`);
- one `claim_alignment_state` from the closed list (`aligned`,
  `weaker_claim_accepted`, `overclaim_blocked`) that the evaluator
  re-derives from the (surface, claimed, observed) triple rather than
  trusting prose;
- one `evidence_export` projection declaring that the fact-lane
  label, readiness token, consumer-surface label, and envelope packet
  ref travel through evidence export, alongside the metadata-safe
  baseline (no raw private material, no ambient authority, no
  destructive resets) so support bundles can preserve graph truth
  without re-running graph producers; and
- one `downgrade_label` from the closed readiness-beta vocabulary
  (`none`, `red_blocks_beta_row`, `yellow_fact_lane_partial`,
  `yellow_evidence_export_skew`, `degraded_to_fallback_search_only`,
  `stale_corpus_blocks_release_candidate`).

Implementation:
[`crates/aureline-graph/src/readiness/beta.rs`](../../../crates/aureline-graph/src/readiness/beta.rs).
Boundary schema:
[`schemas/search/graph_readiness_beta.schema.json`](../../../schemas/search/graph_readiness_beta.schema.json).
Protected fixture corpus:
[`fixtures/graph/m3/readiness_truth/`](../../../fixtures/graph/m3/readiness_truth/).
Baseline report:
[`artifacts/support/m3/graph_readiness_beta_report.md`](../../../artifacts/support/m3/graph_readiness_beta_report.md).
Integration drill:
[`crates/aureline-graph/tests/graph_readiness_beta.rs`](../../../crates/aureline-graph/tests/graph_readiness_beta.rs).
Alpha producer (graph fact cue packet):
[`crates/aureline-graph/src/readiness/mod.rs`](../../../crates/aureline-graph/src/readiness/mod.rs).

## Why this lane exists

The alpha readiness work froze the fact-cue vocabulary on the
producer side — every graph query envelope carries one of the closed
fact lanes and one of the closed readiness states. The beta lane
closes the gap on the *consumer* side: each surface has to declare
what lane it is projecting, which lane it actually observed, and
prove that the claim is not stronger than the evidence allows.
Without that contract a stale review pane and an exact-claiming
navigation row can render the same envelope as two different truths.

The beta projection refuses any case whose `claim_alignment_state`
disagrees with the derived state. Overclaiming graph certainty —
projecting `exact_local_graph_fact` while the envelope only carried
`imported_graph_fact` — is folded into the closed
`overclaim_blocked` state and downgraded with `red_blocks_beta_row`
and an `overclaim_blocked` open gap, blocking beta promotion of the
affected row.

## Lane strength and surface acceptance

The evaluator orders the fact lanes from strongest to weakest:

| Lane | Strength index |
| --- | --- |
| `exact_local_graph_fact` | 0 |
| `imported_graph_fact` | 1 |
| `inferred_graph_fact` | 2 |
| `partial_graph_fact` | 3 |
| `stale_graph_fact` | 4 |
| `waiting_on_graph_provider` | 5 |
| `out_of_scope_graph_fact` | 6 |
| `fallback_search_fact` | 7 |

A claim whose strength index is **lower** than the observed envelope
lane is an overclaim. A claim of equal strength is `aligned` only
when the consumer surface accepts the lane in its `aligned` set;
otherwise it is `weaker_claim_accepted` and must downgrade.

| Consumer surface | Lanes accepted as `aligned` |
| --- | --- |
| `navigation` | `exact_local_graph_fact` |
| `ai_context` | `exact_local_graph_fact`, `imported_graph_fact` |
| `review` | `exact_local_graph_fact`, `imported_graph_fact`, `inferred_graph_fact` |
| `support_export` | any lane (export preserves the label faithfully) |

`support_export` accepts every lane as aligned because its job is to
carry the truth label through the support packet without rewriting
it; it is downgraded only when the export pipeline would otherwise
drop the label (`yellow_evidence_export_skew`).

## Required coverage

The corpus seeds at least one case per consumer surface and at least
one case where the `observed_envelope_lane` is each of the required
fact lanes. At least one case must declare
`claim_alignment_state = overclaim_blocked` so the overclaim-guard
contract is exercised by a fixture rather than an anecdote.

| Required surface | Seeded by |
| --- | --- |
| `navigation` | `navigation_exact_local_aligned_case.yaml`, `navigation_waiting_weaker_claim_case.yaml`, `navigation_out_of_scope_blocked_case.yaml`, `navigation_exact_overclaim_blocked_case.yaml` |
| `ai_context` | `ai_context_inferred_weaker_claim_case.yaml`, `ai_context_partial_weaker_claim_case.yaml`, `ai_context_fallback_only_case.yaml` |
| `review` | `review_imported_aligned_case.yaml` |
| `support_export` | `support_export_stale_aligned_case.yaml` |

| Required envelope lane | Seeded by |
| --- | --- |
| `exact_local_graph_fact` | `navigation_exact_local_aligned_case.yaml` |
| `imported_graph_fact` | `review_imported_aligned_case.yaml`, `navigation_exact_overclaim_blocked_case.yaml` |
| `inferred_graph_fact` | `ai_context_inferred_weaker_claim_case.yaml` |
| `partial_graph_fact` | `ai_context_partial_weaker_claim_case.yaml` |
| `stale_graph_fact` | `support_export_stale_aligned_case.yaml` |
| `waiting_on_graph_provider` | `navigation_waiting_weaker_claim_case.yaml` |
| `out_of_scope_graph_fact` | `navigation_out_of_scope_blocked_case.yaml` |
| `fallback_search_fact` | `ai_context_fallback_only_case.yaml` |

## What the evaluator refuses

- `claim_alignment_state = aligned` when the (consumer_surface,
  claimed_fact_lane, observed_envelope_lane) triple does not satisfy
  the surface-acceptance rule above.
- `claim_alignment_state` whose derived value disagrees with the
  declared value.
- `aligned` rows that carry any non-`none` `downgrade_label` or
  non-`none` `open_gap_class`.
- Non-`aligned` rows (`weaker_claim_accepted`, `overclaim_blocked`)
  that drop the closed downgrade label or fail to record at least
  one closed `open_gap`.
- `overclaim_blocked` rows that downgrade with anything other than
  `red_blocks_beta_row` or that fail to record an
  `overclaim_blocked` open gap.
- `evidence_export` projections that drop `fact_lane_label`,
  `readiness_token`, `consumer_surface_label`, or
  `envelope_packet_ref`.
- `evidence_export` projections that admit raw private material,
  ambient authority, or fail to preserve user-authored files.
- Corpora missing any required consumer surface or observed envelope
  lane, or missing at least one `overclaim_blocked` case.

## What this lane does NOT own

- The producer-side cue packet model — that lives in
  `crates/aureline-graph/src/readiness/mod.rs` and the existing alpha
  fact cue fixture corpus under
  `fixtures/graph/imported_fact_cues/`.
- Live graph storage or query execution — owned by `GraphStore` in
  `crates/aureline-graph/src/store.rs`.
- New surface kinds, fact lanes, downgrade labels, open-gap classes,
  or readiness tokens. Extending any of those lands as a coordinated
  schema, Rust module, fixture, reviewer-doc, and baseline-report
  patch.

## Out of scope

- Live runtime measurement of per-surface latency or throughput.
- Cross-tenant ticket routing — the report is consumed locally by
  the support-export pipeline and the chrome.
- Re-deriving fact lanes from raw graph internals. The beta consumes
  the alpha-produced `GraphFactCuePacket` projection only.
