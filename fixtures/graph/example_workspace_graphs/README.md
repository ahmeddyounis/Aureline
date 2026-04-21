# Semantic-workspace-graph example fixtures

These fixtures are short, reviewable scenarios that anchor the object
model frozen in
[`/docs/graph/workspace_graph_seed.md`](../../../docs/graph/workspace_graph_seed.md)
and validated by the schema at
[`/schemas/graph/workspace_graph_seed.schema.json`](../../../schemas/graph/workspace_graph_seed.schema.json).

Each fixture is one `workspace_graph_record`. Every node and edge
inside a fixture carries a stable id, one class from the frozen
vocabulary, one provenance stamp, one freshness frame, one confidence
level, at least one query-family / shard-affinity / invalidation-producer
tag, and at least one workset / scope ref. Monotonic timestamps and
stable ids are opaque — they are chosen to read well, not to reflect any
wall clock.

**Scope rules**

- Fixtures validate against the workspace-graph seed schema; they do
  not encode wire bytes, subscription envelopes, or RPC envelopes.
- A new fixture MUST exercise at least one frozen node class, one edge
  class, one evidence state, or one freshness value that the existing
  set does not yet cover, and MUST cite the doc section that motivates
  it.
- Filesystem-identity records reuse the five-layer vocabulary frozen in
  [`/schemas/filesystem/save_target_token.schema.json`](../../../schemas/filesystem/save_target_token.schema.json);
  no identity fields are redefined here.
- Generated-artifact nodes carry `lineage_record_ref` into the
  generated-artifact-lineage schema at
  [`/schemas/workspace/generated_artifact_lineage.schema.json`](../../../schemas/workspace/generated_artifact_lineage.schema.json);
  this seed does not re-mint the lineage vocabulary.
- Workset / scope / reachability / environment vocabularies re-export
  the runtime execution-context schema at
  [`/schemas/runtime/execution_context.schema.json`](../../../schemas/runtime/execution_context.schema.json).

**Index**

| Fixture                                                                          | Node classes exercised                                                                                   | Edge classes exercised                                                 | Evidence states exercised                               | Freshness values exercised      | Doc section |
|----------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------|---------------------------------------------------------|---------------------------------|-------------|
| [`local_root_workspace.json`](./local_root_workspace.json)                       | `directory_node`, `file_node`, `symbol_node`, `doc_node`, `ownership_node`, `workset_scope_node`         | `contains`, `defines_symbol`, `documented_by`, `owned_by`, `scoped_by` | `direct_evidence`                                       | `authoritative`                 | §7.1        |
| [`generated_artifact_lineage.json`](./generated_artifact_lineage.json)           | `file_node`, `topology_node`, `generated_artifact_node`                                                  | `produces_artifact`, `generated_from`                                  | `direct_evidence`                                       | `authoritative`, `cached`       | §7.2        |
| [`provider_resources_and_citations.json`](./provider_resources_and_citations.json) | `provider_resource_node`, `topology_node`, `symbol_node`, `file_node`, `doc_node`                       | `hosted_by`, `references_symbol`, `impacts`, `cites`                   | `direct_evidence`, `inferred_relation`                  | `authoritative`                 | §7.3        |
| [`imported_root_vendor_drop.json`](./imported_root_vendor_drop.json)             | `imported_root_node`, `file_node`, `ownership_node`, `workset_scope_node`                                | `contains`, `owned_by`, `scoped_by`                                    | `direct_evidence`, `imported_evidence`                  | `authoritative`, `imported`     | §7.4        |
| [`partial_workset_visibility.json`](./partial_workset_visibility.json)           | `workset_scope_node`, `policy_view_node`, `missing_anchor_node`, `file_node`, `symbol_node`              | `defines_symbol`, `scoped_by`, `references_symbol`, `missing_anchor_for` | `direct_evidence`, `stale_relation`, `missing_anchor` | `authoritative`, `stale`        | §7.5        |

**Coverage**

Across the five fixtures every frozen node class except
`policy_view_node` carries exercise from at least two fixtures;
`policy_view_node` is exercised once in `partial_workset_visibility`.
Every evidence state is exercised at least once. Every provenance
class named in the doc is exercised at least once across the set
(`authoritative_producer`, `imported_external`, `policy_projected`).

**What this seed is NOT**

- Not a runtime graph engine. The fixtures are hand-written snapshots;
  the prototype crate under `/crates/aureline-graph-proto/` constructs
  mirror Rust values and enforces identity / label rules.
- Not a replay format. Replay captures will cite these nodes and edges
  by id but carry their own envelope.
- Not authoritative over raw source bytes, raw provider URLs, or raw
  credentials. Those never cross this boundary.
