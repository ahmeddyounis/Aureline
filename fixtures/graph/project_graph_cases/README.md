# Project Graph Case Fixtures

These fixtures exercise the project-graph and incremental-indexing
contracts frozen in
[`/docs/graph/project_graph_and_indexing_seed.md`](../../../docs/graph/project_graph_and_indexing_seed.md).

Schemas:

- [`/schemas/graph/project_node.schema.json`](../../../schemas/graph/project_node.schema.json)
- [`/schemas/index/index_work_item.schema.json`](../../../schemas/index/index_work_item.schema.json)

## Cases

| Fixture | Schema | Coverage |
|---|---|---|
| [`rust_workspace_hot_set_project_graph.json`](./rust_workspace_hot_set_project_graph.json) | project graph | repo, module, package, target, environment, framework, semantic graph bridge, task-event and execution-context bridges, AI/review/support/doctor bridges |
| [`managed_remote_partial_index_bundle.json`](./managed_remote_partial_index_bundle.json) | indexing | hot-set plan, local/remote/managed shards, work items, invalidation, readiness export, omitted/unfetched/uninitialized/partial/stale labels |

## Rules

- Fixtures use opaque refs. Raw source bodies, raw command lines, raw
  environment bodies, raw provider URLs, raw credentials, and raw query
  text do not appear here.
- Project nodes are the identity authority for repositories, modules,
  packages, targets, environments, and framework facts. Semantic graph
  ids deepen those facts but do not replace them.
- Indexing fixtures preserve the same completeness labels expected to
  travel into search, review, AI, Project Doctor, and support exports.
