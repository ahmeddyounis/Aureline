# Repository-topology edge-case fixtures

These YAML fixtures exercise the repository-topology and object-
availability vocabulary frozen in
[`/docs/workspace/repository_topology_edge_case_contract.md`](../../../docs/workspace/repository_topology_edge_case_contract.md)
and validated by:

- [`/schemas/workspace/repo_topology_state.schema.json`](../../../schemas/workspace/repo_topology_state.schema.json)
- [`/schemas/workspace/object_availability_state.schema.json`](../../../schemas/workspace/object_availability_state.schema.json)

Each fixture is metadata-safe: raw absolute paths, credentials, remote
URLs, file bodies, and object bytes are replaced with opaque refs.

| Fixture | Schema | Key class exercised |
|---|---|---|
| [`sparse_workset_omitted_paths.yaml`](./sparse_workset_omitted_paths.yaml) | `repo_topology_state_record` | sparse/workset omitted paths, `outside_current_slice`, widen affordance |
| [`partial_clone_unfetched_object.yaml`](./partial_clone_unfetched_object.yaml) | `object_availability_state_record` | promisor object, `not_fetched`, fetch affordance |
| [`shallow_history_blame_boundary.yaml`](./shallow_history_blame_boundary.yaml) | `object_availability_state_record` | shallow history, `shallow_boundary`, deepen affordance |
| [`submodule_uninitialized_pinned_commit.yaml`](./submodule_uninitialized_pinned_commit.yaml) | `repo_topology_state_record` | submodule root, pinned commit, init affordance |
| [`nested_repo_wrong_target_denied.yaml`](./nested_repo_wrong_target_denied.yaml) | `repo_topology_audit_event_record` | nested repo boundary and wrong-target-root denial |
| [`lfs_pointer_only_hydration.yaml`](./lfs_pointer_only_hydration.yaml) | `object_availability_state_record` | LFS pointer-only state, hydrate affordance |
| [`absent_path_reconstruction.yaml`](./absent_path_reconstruction.yaml) | `object_availability_state_record` | content absent from repository, no fetch/hydrate/init affordance |

Coverage is intentional. Removing any topology class above requires a
replacement fixture in the same change.
