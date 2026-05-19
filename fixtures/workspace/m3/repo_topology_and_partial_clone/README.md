# Repository-topology beta and partial-clone fixtures

These JSON fixtures bind the four beta descriptors frozen in
[`/docs/workspace/m3/repo_topology_beta.md`](../../../../docs/workspace/m3/repo_topology_beta.md)
into one cross-surface projection per scenario:

- [`/schemas/workspace/repo_root_descriptor.schema.json`](../../../../schemas/workspace/repo_root_descriptor.schema.json)
- [`/schemas/workspace/fetch_depth_descriptor.schema.json`](../../../../schemas/workspace/fetch_depth_descriptor.schema.json)
- [`/schemas/workspace/submodule_link.schema.json`](../../../../schemas/workspace/submodule_link.schema.json)
- [`/schemas/workspace/lfs_hydration_descriptor.schema.json`](../../../../schemas/workspace/lfs_hydration_descriptor.schema.json)

Each fixture is metadata-safe: raw absolute paths, credentials, raw
remote URLs, raw file bodies, and raw object bytes never appear; only
opaque refs and the typed labels frozen on the schemas cross the
boundary.

| Fixture | Scenario |
|---|---|
| [`primary_full_local_truth.json`](./primary_full_local_truth.json) | Primary root with full history, no LFS, no submodules — every surface may claim full coverage. |
| [`primary_shallow_partial_clone_promisor.json`](./primary_shallow_partial_clone_promisor.json) | Primary root with shallow history and partial-clone filter — blame and search must downgrade and offer deepen + fetch. |
| [`monorepo_with_uninitialized_submodule.json`](./monorepo_with_uninitialized_submodule.json) | Primary root with one uninitialized submodule — review may not claim child content; mutation stays read-only until init. |
| [`monorepo_lfs_pointer_only.json`](./monorepo_lfs_pointer_only.json) | Primary root with pointer-only LFS assets — publish exports metadata-only; edit denied until hydrate. |
| [`submodule_root_initialized.json`](./submodule_root_initialized.json) | Initialized submodule root with drift below pin — mutation target is the child root. |
| [`nested_independent_root.json`](./nested_independent_root.json) | Nested independent repository — parent mutation denied; surface must switch target root. |
| [`linked_worktree_root.json`](./linked_worktree_root.json) | Linked worktree root — sparse projection narrows coverage; mutation routes to the worktree child root. |

Removing any of these scenarios without a replacement fixture is a
breaking contract change.
