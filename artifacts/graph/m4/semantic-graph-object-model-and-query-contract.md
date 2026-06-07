# Semantic graph object model and query contract artifact

The stable M4 graph packet freezes one shared object id space, one query-family
vocabulary, bounded invalidation, and evidence-linked reconstruction for graph
consumers.

Canonical packet:
`artifacts/graph/m4/semantic-graph-object-model-and-query-contract.json`

Canonical schema:
`schemas/graph/semantic-workspace-graph.schema.json`

Fixture corpus:
`fixtures/graph/m4/semantic-graph-object-model-and-query-contract`

The packet validates as `stable` only when:

- all six required object classes are present with producer, schema, freshness,
  confidence, visibility, retention, provenance, and evidence metadata;
- query families are exactly `lookup`, `neighborhood`, `explain-why`, `impact`,
  `ownership`, `diff-between-snapshots`, and `path-to-evidence`;
- ordinary invalidation is scoped to affected subgraphs, while full rebuilds are
  visible schema, producer-version, or workspace-epoch events;
- search, review, docs, navigation, AI context, onboarding, topology, and
  support export actions embed stable ids or query handles;
- topology canvases, table/list fallbacks, breadcrumbs, inspectors, explainers,
  tours, and exports reuse the same id, scope, freshness, and confidence model;
- support export can reconstruct the exact graph objects, query handles, query
  families, invalidation events, and evidence links that produced user-visible
  answers.
