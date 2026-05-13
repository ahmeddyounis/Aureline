# Graph fact-truth cue alpha

This document defines the first consumer-ready graph fact cue layer. It sits
on top of the graph query-family runtime in
[`query_family_alpha.md`](./query_family_alpha.md) and the workspace graph
seed in [`workspace_graph_seed.md`](./workspace_graph_seed.md). It does not
create a second graph, search, freshness, or confidence model.

The runtime implementation lives in
[`crates/aureline-graph/src/readiness`](../../crates/aureline-graph/src/readiness).
The protected fixture lives at
[`fixtures/graph/imported_fact_cues/readiness_and_fact_cues.json`](../../fixtures/graph/imported_fact_cues/readiness_and_fact_cues.json).

## Runtime contract

`GraphFactCuePacket` projects one graph query envelope, or one declared
fallback-search packet, into labels that product surfaces can render without
unpacking raw graph internals.

Each packet carries:

- `consumer_surface`: `navigation`, `ai_context_selection`, `review_seed`, or
  `support_export`;
- `readiness`: the graph or fallback readiness token;
- `truth_lanes`: the unique fact-truth lanes present in the packet;
- ordered `cues`;
- `export_preserves_fact_labels`: true only when every cue carries its truth
  lane in `export_labels`.

Each cue carries only safe projection fields: graph ref, display label, row
class, truth lane, readiness, action posture, confidence token, freshness token,
edge evidence token, partial-truth causes, optional navigation projections, and
export labels. It intentionally does not export full freshness frames,
scope-ref arrays, source anchors, or raw source payloads.

## Truth lanes

| Truth lane | Meaning | Default action posture |
|---|---|---|
| `exact_local_graph_fact` | Proven by current local graph rows for the declared scope. | `direct_navigation` |
| `imported_graph_fact` | Graph-backed but imported from a bundle, provider overlay, or docs pack. | `read_only_imported` |
| `inferred_graph_fact` | Graph-backed but inferred or derived rather than directly observed. | `inspect_before_use` |
| `stale_graph_fact` | Graph-backed but stale, cached, or replayed. | `inspect_before_use` |
| `partial_graph_fact` | Graph-backed but incomplete for the declared scope. | `refresh_or_narrow_scope` |
| `policy_hidden_graph_fact` | Present only as a policy-limited projection. | `blocked_by_policy_or_scope` |
| `missing_anchor_graph_fact` | The row points at a missing graph anchor. | `blocked_by_policy_or_scope` |
| `waiting_on_graph_provider` | Graph rows are not ready because a graph producer or richer provider is pending. | `waiting_on_provider` |
| `out_of_scope_graph_fact` | The subject is outside the active graph scope. | `blocked_by_policy_or_scope` |
| `fallback_search_fact` | Candidate came from fallback search rather than graph truth. | `fallback_search_only` |

## Consumer rules

- Navigation may directly open `exact_local_graph_fact` rows. Any other graph
  lane needs a visible cue, and fallback search must remain labeled as fallback.
- AI context selection must preserve graph-backed lanes separately from
  fallback search candidates so prompt evidence cannot imply graph certainty.
- Review seeds must preserve imported, inferred, stale, partial, missing-anchor,
  and policy-limited lanes in the review packet.
- Support and evidence exports must carry `truth_lane`, `readiness`, and
  `export_labels` for every cue.

## Acceptance proof

The protected runtime test covers six acceptance lanes:

- exact local symbol lookup from the local workspace graph;
- imported ownership lookup from the vendor-drop graph;
- inferred impact seed from the provider-resource graph;
- stale symbol lookup from a stale graph row;
- waiting provider cue from an unavailable graph envelope;
- fallback search cue with no graph backing.

The export test serializes `GraphFactCuePacket` and verifies that the fact
labels survive in `truth_lane` and `export_labels` while raw graph internals do
not appear in the surface packet.
