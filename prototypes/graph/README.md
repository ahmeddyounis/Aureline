# Semantic-workspace-graph seed prototype

Contract-first prototype for the object model frozen in
`docs/graph/workspace_graph_seed.md` and the boundary schema at
`schemas/graph/workspace_graph_seed.schema.json`.

Not a graph engine. Not a persistence layer. Not a performance target.
This crate's job is to make every node class, every edge class, every
evidence state, every provenance class, every freshness value, every
confidence level, every query-family / shard-affinity /
invalidation-producer tag, every topology-edge / impact-reason /
explainer-citation slot, and every identity / label rule the doc names
observable against a frozen scenario table so the vocabulary cannot
silently drift.

The crate lives at `crates/aureline-graph-proto/`.

## What this prototype exercises

Five scenarios — one per doc §7 sub-section — construct an in-memory
workspace-graph, run it through the identity / label rule validator,
and emit a byte-stable per-scenario structural record plus an
aggregate report.

| Scenario label | Exercises |
| -------------- | --------- |
| `local_root_workspace` | §7.1 — File, Directory, Symbol, Doc nodes on a local workspace root; `contains`, `defines_symbol`, `references_symbol`, `documented_by`, `scoped_by`. |
| `generated_artifact_lineage` | §7.2 — Generated-artifact + Ownership nodes; `generated_from` with `inferred_relation`, `owned_by`, `produces_artifact`, `consumes_artifact` with `topology_edge_slot`. |
| `provider_resources_and_citations` | §7.3 — Topology and ProviderResource nodes; `deployed_to`, `runs_in`, `hosted_by` with `topology_edge_slot`; `explainer_citation` slots. |
| `imported_root_vendor_drop` | §7.4 — ImportedRoot node; `imported_evidence`, `imported` freshness with `imported_from_external`, `ImportedExternal` provenance; `mirrors_upstream`. |
| `partial_workset_visibility` | §7.5 — WorksetScope, PolicyView, MissingAnchor nodes; `scoped_by` with `partial_visible` visibility, `stale_relation` with `upstream_input_stale`, `missing_anchor_for` edge with `missing_anchor` evidence. |

## Seven modules

| Module | Role |
| ------ | ---- |
| `vocab` | Frozen vocabulary enums (node/edge classes, evidence states, freshness, confidence, provenance, tags, topology, citations, audit event ids). |
| `model` | In-memory `WorkspaceGraph`, `GraphNode`, `GraphEdge`, and supporting slot records. |
| `validator` | Eleven identity / label rules returning typed `ValidationError`. |
| `hooks` | 13 hook counters (3 protected-hot-path, 10 observability). |
| `render` | Hand-rolled canonical JSON renderer. No serde, no wall-clock. |
| `scenarios` | Five frozen scenarios mirroring the fixtures one-for-one. |
| `harness` | Scenario table runner + aggregate + per-scenario JSON emission. |

## Byte-stable artifacts

```
./tools/graph_proto.sh --emit artifacts/graph/workspace_graph_seed.json
./tools/graph_proto.sh --emit-scenarios artifacts/graph/workspace_graph_seed_scenarios
./tools/graph_proto.sh --emit-graphs artifacts/graph/workspace_graph_seed_graphs
```

The wrapper pins `SOURCE_DATE_EPOCH`, `TZ=UTC`, `LC_ALL=C`. Synthetic
monotonic tokens (`mono:graph:SSSS:00:00:00.TTTT`) replace wall clock.
Counts-only metrics. Two runs on two hosts MUST produce identical
bytes.

Human-authored graph fixtures live at
`fixtures/graph/example_workspace_graphs/` and validate against the
boundary schema at `schemas/graph/workspace_graph_seed.schema.json`.

## Known holes (explicit — carry forward, not silent capabilities)

1. **No persistence layer.** Graphs live in RAM only for the duration
   of a scenario. The real store is sharded and backed by the durable
   cache seam; this prototype does not implement the shard fanout or
   the cache-durability contract.
2. **No search planner.** `query_family_tags` and
   `shard_affinity_tags` are attached and counted but never consumed
   by a planner. The eventual search planner reads these tags through
   the same schema without renaming them.
3. **No subscription fabric.** Graph snapshots do not ride a
   `SubscriptionEnvelope`. The reactive-state prototype owns that
   seam; the eventual graph producer MUST publish through it.
4. **No AI context assembler.** `explainer_citations` are authored by
   hand. The real AI context assembler composes them from the graph
   and the cited-explainer overlay, both of which ride on these
   records.
5. **No review-pack exporter.** `impact_reasons` with
   `mutation_journal_ref` / `review_ref` are attached but the exporter
   that bundles them for code review is out of scope here.
6. **No invalidation producer simulation.** `invalidation_producer_tags`
   label which producers invalidate a record; the prototype does not
   actually run those producers or cascade invalidation. That plumbing
   lives with the mutation-journal and subscription lanes.
7. **No imported-bundle verification.** The `imported_bundle_ref`
   slot carries a string; there is no signature check, no trust-boundary
   walk, no policy-gate enforcement. Bundle trust lives with the
   support-export lane.
8. **No replay-capture integration.** `replay_capture_ref` is declared
   in the provenance stamp but no replay session is simulated. Replay
   integration lands with the support-export + timeline lanes.
9. **Single workspace, single shard.** Every scenario uses
   `workspace_id = "ws:aureline"` and the same root scope. Cross-workspace
   fan-out, multi-shard reconciliation, and multi-tenant isolation are
   out of scope here.
10. **No confidence-lattice full enforcement.** Rule 6 covers the
    floor rule (any low/unknown contributor pulls the rollup to at
    least low). A complete lattice with explicit operator overrides
    lands with the observability design doc.
11. **No schema-conformance harness.** The prototype constructs graphs
    in Rust and does not round-trip them through a JSON schema
    validator. The fixtures are human-authored against the schema;
    the conformance lane wires the two together.
12. **No mutation-journal wiring.** `ImpactReason.mutation_journal_ref`
    and `review_ref` are recorded but not linked to a live journal.
    The mutation-journal crate consumes these slots when it lands.
13. **No policy-entitlement enforcement.** `PolicyView` nodes declare
    `hidden_member_count` and `policy_ref`; the policy authority that
    actually filters visible members is not wired in.
14. **No topology map renderer.** Topology edges carry
    `topology_edge_slot` but no topology-map UI or service graph view
    consumes them. That renderer ships with the topology lane.
15. **No adversarial scenario coverage.** The scenario table is
    happy-path plus expected edge cases (stale relation, missing anchor,
    imported evidence, policy-hidden). It does not fuzz invariants or
    violate rules deliberately; rule-violation coverage belongs to a
    later conformance harness that drives the validator with broken
    inputs.

## Carry-forward items

- **Benchmark lab** layers timing traces over the hook counters emitted
  here. The shell-spike trace family and the reactive-state invalidation
  trace family share hook-naming conventions so a future aggregator
  sees one vocabulary.
- **Search planner** reads `query_family_tags` and
  `shard_affinity_tags` through the same schema; it MUST NOT rename
  these tokens.
- **Subscription fabric** wraps `WorkspaceGraph` snapshots as
  subscription payloads; `invalidation_producer_tags` drive
  invalidation routing.
- **Mutation journal** populates `impact_reasons.mutation_journal_ref`
  and `review_ref` from live commits; it MUST emit against the same
  `impact_reason_class` vocabulary.
- **Cited-explainer overlay + AI context assembler** consume
  `explainer_citations` as authoritative evidence pointers; they MUST
  use the same `citation_class` vocabulary.
- **Support-export lane** bundles graphs for replay and review; the
  `provenance_stamp.imported_bundle_ref` and `replay_capture_ref`
  slots are the handoff.
- **Topology map + impact explorer** consume `topology_edge_slot` and
  `impact_reasons` respectively; both MUST reuse the existing edge
  vocabulary.

Adding new scenarios is additive-minor: extend the scenario table in
`crates/aureline-graph-proto/src/scenarios.rs`, add a human-authored
fixture under `fixtures/graph/example_workspace_graphs/`, and
regenerate artifacts via the wrapper script. Adding a node class,
edge class, evidence state, provenance class, freshness value,
confidence level, tag family, or slot reopens the design doc at
`docs/graph/workspace_graph_seed.md` and bumps
`WORKSPACE_GRAPH_SCHEMA_VERSION`.
