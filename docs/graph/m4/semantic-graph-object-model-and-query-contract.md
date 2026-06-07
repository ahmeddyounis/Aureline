# Semantic graph object model and query contract

This is the stable graph contract consumed by search, review, docs, navigation,
AI context, onboarding, topology, and support export surfaces.

The Rust source of truth is
`crates/aureline-graph/src/semantic_graph_object_model_and_query_contract/mod.rs`.
The checked-in packet is
`artifacts/graph/m4/semantic-graph-object-model-and-query-contract.json`, and the
schema is `schemas/graph/semantic-workspace-graph.schema.json`.

## Stable object classes

Every stable graph object carries object kind, stable id, producer identity,
schema version, freshness timestamp, confidence tier, visibility scope,
retention class, provenance refs, and evidence refs.

Required object classes:

- `workspace_root_workset`
- `file_document`
- `symbol_api`
- `relationship_edge`
- `docs_knowledge`
- `operational_artifact`

Relationship-edge objects also carry source and target object ids. Surfaces must
reuse these ids in maps, table/list fallbacks, breadcrumbs, AI inspectors,
onboarding tours, review explainers, and support exports.

## Stable query families

The stable query-family vocabulary is closed to:

- `lookup`
- `neighborhood`
- `explain-why`
- `impact`
- `ownership`
- `diff-between-snapshots`
- `path-to-evidence`

Any consumer that depends on a richer private family must mark the consumer as
`narrowed_below_stable` until that dependency is migrated or recorded in an ADR.

## Freshness, scope, and confidence

Graph-backed consumers reuse one freshness vocabulary:

- `live`
- `stale`
- `warming`
- `partial_scope`
- `cached`
- `provider_unavailable`

The shared scope vocabulary is `current_root`, `selected_workset`,
`full_workspace`, `remote_cache`, `outside_current_scope`, and
`policy_limited`. Confidence is one of `immediate_lexical`, `structural`,
`semantic`, `verified_runtime`, `user_curated`, `inferred`, or `withheld`.

Non-live freshness, policy-limited scope, outside-current-scope rows, inferred
confidence, and withheld confidence require a visible truth label.

## Invalidation

Ordinary graph refreshes must use `smallest_subgraph` with affected object refs.
Full graph rebuilds are valid only for:

- `full_rebuild_schema_boundary`
- `full_rebuild_producer_version_boundary`
- `full_rebuild_workspace_epoch_boundary`

`arbitrary_full_graph_rebuild` is a blocker. Full-rebuild boundaries must be
visible to consumers and carry the schema, producer-version, or workspace-epoch
evidence that caused the rebuild.

## Consumer bindings and export

Each stable consumer action embeds stable object ids or stable query handles.
The support reconstruction projection preserves object ids, query handles,
query-family tokens, invalidation event refs, and evidence links while excluding
raw private material and ambient authority.
