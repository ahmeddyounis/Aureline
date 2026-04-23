# Public query-family case fixtures

These fixtures are hand-authored `result_envelope_record`s that
anchor the public query-family contract frozen in
[`/docs/graph/identity_and_query_family_seed.md`](../../../docs/graph/identity_and_query_family_seed.md)
and validated by the schema at
[`/schemas/graph/query_family.schema.json`](../../../schemas/graph/query_family.schema.json).

Each fixture pins one `query_family_id`, one `query_intent_class`,
one `readiness_state`, one `result_truth_class`, and a
`result_envelope_version` of `1`. Rows cite `node_identity` or
`edge_family` ids — never raw source bodies, never raw provider
URLs, never raw query text. Monotonic timestamps and stable ids are
opaque — they are chosen to read well, not to reflect any wall
clock.

**Scope rules**

- Fixtures validate against the query-family schema; they do not
  encode wire bytes, subscription envelopes, or RPC envelopes.
- A new fixture MUST exercise at least one `query_family_id`, one
  `query_safety_class`, one `query_intent_class`, or one
  `result_row_class` the existing set does not yet cover, and MUST
  cite the doc section that motivates it.
- `readiness_state` and `result_truth_class` reuse the vocabulary
  owned by
  [`schemas/search/search_result_truth.schema.json`](../../../schemas/search/search_result_truth.schema.json);
  no parallel vocabulary is minted.
- Rows pair `identity_durability_class` /
  `freshness_confidence_compatibility_class` (for
  `node_identity_row`) or `edge_durability_class` /
  `freshness_confidence_compatibility_class` (for
  `edge_family_row`) exactly as required by the published boundary
  contract.

**Index**

| Fixture                                                                                    | Query family           | Intent                         | Readiness       | Truth class | Safety classes exercised                                                 | Doc section |
|--------------------------------------------------------------------------------------------|------------------------|--------------------------------|-----------------|-------------|--------------------------------------------------------------------------|-------------|
| [`symbol_jump_exact_case.json`](./symbol_jump_exact_case.json)                             | `symbol_jump`          | `lookup_by_identity`           | `fully_indexed` | `exact`     | `inspect_read_only`, `read_durable_only`                                  | §5.1        |
| [`docs_search_hybrid_case.json`](./docs_search_hybrid_case.json)                           | `docs_search`          | `lookup_by_label`              | `warm_index`    | `hybrid`    | `inspect_read_only`, `read_durable_only`                                  | §5.2        |
| [`topology_walk_cached_case.json`](./topology_walk_cached_case.json)                       | `topology_walk`        | `walk_typed_edges`             | `warm_index`    | `exact`     | `walk_scope_bounded`, `read_durable_only`, `public_api_safe`              | §5.3        |
| [`impact_explorer_inferred_case.json`](./impact_explorer_inferred_case.json)               | `impact_explorer`      | `collect_impact_set`           | `fully_indexed` | `heuristic` | `inspect_read_only`, `read_derived_permitted`                             | §5.4        |
| [`ai_context_assembly_cited_case.json`](./ai_context_assembly_cited_case.json)             | `ai_context_assembly`  | `collect_context_set`          | `fully_indexed` | `hybrid`    | `ai_context_restricted`, `read_derived_permitted`                         | §5.5        |
| [`support_export_walk_bundle_case.json`](./support_export_walk_bundle_case.json)           | `support_export_walk`  | `export_support_bundle_slice`  | `fully_indexed` | `exact`     | `support_export_bundle`, `walk_scope_bounded`                             | §5.6        |
| [`public_graph_query_scoped_case.json`](./public_graph_query_scoped_case.json)             | `public_graph_query`   | `walk_typed_edges`             | `fully_indexed` | `exact`     | `public_api_safe`, `walk_scope_bounded`, `read_durable_only`              | §5.7        |

**Coverage**

Across the seven fixtures every published `result_row_class` is
exercised at least once (`node_identity_row`, `edge_family_row`,
`missing_anchor_row`, `policy_hidden_row`, `scope_partial_row`).
Every `identity_durability_class` is exercised
(`durable_identity`, `derived_identity`, `synthetic_identity`).
Every `edge_durability_class` is exercised (`durable_edge`,
`derived_edge`; `stale_edge` and `missing_anchor_edge` are
exercised via the graph-seed fixtures under
[`/fixtures/graph/example_workspace_graphs/`](../example_workspace_graphs)
that these envelopes cite by id). Every
`freshness_confidence_compatibility_class` except
`live_low_confidence`, `stale_last_known_good`,
`replayed_bundle_snapshot`, and warming_partial is exercised here;
the remaining postures are exercised via workspace-graph fixtures
referenced by id.

**What this seed is NOT**

- Not a runtime query engine. The fixtures are hand-written
  envelopes; the full query planner lands with later search-planner
  and public graph query work.
- Not a replay format. Replay captures will cite these envelopes
  and rows by id but carry their own transport envelope.
- Not authoritative over raw source bytes, raw provider URLs, or
  raw credentials. Those never cross this boundary.
