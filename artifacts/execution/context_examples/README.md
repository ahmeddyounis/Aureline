# Execution-context snapshot seed examples

These artifacts are short, reviewable examples of the shared execution-context
snapshot shape every task launch, terminal session seed, and debug-prep seed
emits. They are the companion to
[`/schemas/execution/context_snapshot.schema.json`](../../../schemas/execution/context_snapshot.schema.json)
and the seed cases in
[`/fixtures/execution/context_diff_cases/`](../../../fixtures/execution/context_diff_cases/).

## What each example shows

Each file is one `context_snapshot_record` — or, for the inspector-view
example, one `context_inspector_view_record` — assembled against a
seeded `execution_context_record` from
[`/fixtures/runtime/execution_context_examples/`](../../../fixtures/runtime/execution_context_examples/).
The snapshot quotes the underlying record by id instead of copying it.

| Example | Source surface | Underlying execution-context fixture | Notes |
|---|---|---|---|
| [`task_launch_snapshot.json`](./task_launch_snapshot.json) | `task_launch` | `local_task_launch.json` | Local host cargo-test launch. Fully resolved; no degraded fields. |
| [`terminal_session_seed_snapshot.json`](./terminal_session_seed_snapshot.json) | `terminal_session_seed` | `remote_ssh_attach.json` | Remote SSH terminal seed with a warming reachability state and an ssh_key_material temporary-handle class. |
| [`debug_prep_seed_snapshot.json`](./debug_prep_seed_snapshot.json) | `debug_prep_seed` | `devcontainer_launch.json` | Debug prep against a devcontainer with a reused prebuild snapshot. |
| [`inspector_view_task_launch.json`](./inspector_view_task_launch.json) | `env_inspect_cli` | same as `task_launch_snapshot.json` | Compact inspector-view projection over the task-launch snapshot. |

## Scope rules

- Every example carries `context_snapshot_schema_version: 1` and validates
  against the shared snapshot schema.
- Snapshots record class labels, frozen tokens, opaque ids, hashes, and
  counts only. Raw env bodies, raw command lines, and raw secret values
  never appear in this directory.
- Snapshots that cite a temporary secret handle carry a
  `secret_posture_summary` with the ADR-0007 secret class label and a
  count. The raw handle material is not projected.
- Opaque ids and timestamps are chosen for review clarity rather than to
  mirror a real machine.

## Coverage contract

The example set MUST keep at least one snapshot for each of the three
surfaces the shared contract binds together (`task_launch`,
`terminal_session_seed`, `debug_prep_seed`), and at least one
inspector-view projection. Adding a snapshot for a new surface or a new
redaction posture is welcome; removing a surface that this directory
already covers is a breaking change.
