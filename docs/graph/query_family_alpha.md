# Graph query-family alpha runtime

This document defines the first runtime slice of the semantic graph query
surface. It narrows the published query-family contract in
[`identity_and_query_family_seed.md`](./identity_and_query_family_seed.md)
to the query classes that are implemented by
[`crates/aureline-graph`](../../crates/aureline-graph):

- `symbol_lookup`
- `import_neighborhood`
- `ownership_lookup`
- `impact_seed`
- `explainer_citation_seed`

The machine-readable alpha schema lives at
[`/schemas/graph/query_family_alpha.schema.json`](../../schemas/graph/query_family_alpha.schema.json).
The protected fixture lives at
[`/fixtures/graph/query_family_alpha/launch_wedge_symbols_imports_ownership.json`](../../fixtures/graph/query_family_alpha/launch_wedge_symbols_imports_ownership.json).

The runtime consumes the existing canonical graph seed model from
[`workspace_graph_seed.md`](./workspace_graph_seed.md) and
[`crates/aureline-graph-proto`](../../crates/aureline-graph-proto). It does
not mint a second node, edge, freshness, confidence, or scope vocabulary.

## Runtime contract

`GraphStore` persists one validated `workspace_graph_record` snapshot. A
snapshot is admitted only after the canonical graph seed validator accepts
node/body alignment, unique ids, edge endpoint resolution, tag lists,
freshness framing, confidence rollups, provenance anchors, imported evidence
framing, missing-anchor framing, topology slots, and generated-artifact lineage
rules.

Each query emits a `graph_query_envelope` with:

- `schema_version`
- `envelope_id`
- `query_request_id`
- `query_class`
- `query_family_tag`
- `workspace_graph_id`
- `workspace_id`
- `emitted_at`
- `readiness`
- `partial_truth_causes`
- ordered `rows`

Rows reference canonical `node_id` or `edge_id` values. They may include
`display_label`, `relative_path`, and `symbol_ref` as navigation projections,
but consumers must still treat the graph id as the identity anchor.

## Query classes

| Query class | Published family | Runtime rows | Freshness and confidence behavior |
|---|---|---|---|
| `symbol_lookup` | `symbol_jump` | `symbol_node` rows | Prefer authoritative symbol resolver rows. Warming, stale, imported, replayed, low, and unknown rows stay disclosed. |
| `import_neighborhood` | `semantic_code_search` | `imports_module` edges plus endpoint file/symbol nodes | Direct parser evidence is authoritative for the snapshot. Inferred, stale, missing-anchor, and partial-scope rows must render their cause. |
| `ownership_lookup` | `ownership_lookup` | `owned_by` edges plus owner nodes | Ownership authority is bounded to the graph epoch and source anchors, usually CODEOWNERS or equivalent ownership rules. |
| `impact_seed` | `impact_explorer` | Stored `impacts` edges only | This seed never claims full transitive impact depth. It returns only admitted impact rows. |
| `explainer_citation_seed` | `cited_explainer_walk` | Stored `cites` and `explains` edges plus cited nodes | This seed returns citation anchors only. It does not generate explanatory prose. |

## Consumer rule

Search and navigation consume the graph envelope directly through
`PlannerPathSnapshot::from_graph_query_envelope`. That adapter maps graph rows
into the existing planner path model while preserving:

- canonical graph row identity;
- graph epoch;
- readiness;
- freshness class;
- confidence and partial-truth causes;
- symbol/file navigation projections.

Consumers must not unpack graph results into a private object model that drops
freshness, confidence, scope, or missing-anchor state.

## Acceptance proof

The protected fixture exercises the launch-wedge set:

- `symbol_lookup` resolves `node:symbol:greet_fn`;
- `import_neighborhood` returns an `imports_module` edge and endpoint nodes;
- `ownership_lookup` returns an `owned_by` edge and owner node.

The runtime tests validate the graph before persistence, execute each query,
and verify row identity, readiness, row count, symbol navigation projections,
and descriptor tokens. The search-planner consumer test verifies that a graph
query envelope becomes a `graph_backed` planner snapshot without a private
search-side row contract.
