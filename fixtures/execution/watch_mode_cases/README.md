# Test watch-mode, inline-result, and environment-matrix worked cases

These fixtures are short, reviewable scenarios that anchor the contract
frozen in
[`/docs/execution/test_watch_and_environment_contract.md`](../../../docs/execution/test_watch_and_environment_contract.md)
and validated by:

- [`/schemas/execution/watch_controller_state.schema.json`](../../../schemas/execution/watch_controller_state.schema.json)
- [`/schemas/execution/inline_test_result.schema.json`](../../../schemas/execution/inline_test_result.schema.json)
- [`/schemas/execution/environment_matrix_row.schema.json`](../../../schemas/execution/environment_matrix_row.schema.json)

Each fixture is one record rendered as a worked scenario. The set exists
so a reviewer can read watch fidelity, inline marker truth, and
environment comparison truth across one corpus rather than reverse-
engineering per-surface prose.

## Scope rules

- Fixtures carry the matching `*_schema_version: 1` const.
- Fixtures MUST NOT encode raw command lines, raw stdout / stderr byte
  streams, raw env bodies, raw API request / response bodies, raw
  absolute paths, raw URLs, raw secret values, raw test names, raw
  assertion bodies, raw notebook cell bodies, raw artifact bytes, or raw
  stack traces. Only class labels, frozen tokens, opaque ids, counts,
  timestamps, and review-safe summaries are admissible.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.
- Run, attempt, discovery, context-snapshot, command-dispatch,
  watch-controller, environment-row, artifact-event, support-export,
  event-lineage, audit-event, provider-import, and target-identity refs
  are opaque.

## Index

| Fixture | Record kind | Key coverage |
|---|---|---|
| [`live_local_watch.yaml`](./live_local_watch.yaml) | `watch_controller_state_record` | `state_class = live`, event-driven debounce, bounded batch, local target healthy, degradation visible in editor/test tree/CLI/support surfaces |
| [`reduced_container_watch.yaml`](./reduced_container_watch.yaml) | `watch_controller_state_record` | `state_class = reduced`, power/thermal throttle narrows batching while container target remains runnable |
| [`polling_remote_watch.yaml`](./polling_remote_watch.yaml) | `watch_controller_state_record` | `state_class = polling`, remote watcher falls back to fixed interval polling with latency disclosure |
| [`paused_power_saver_watch.yaml`](./paused_power_saver_watch.yaml) | `watch_controller_state_record` | `state_class = paused`, power state pauses watch and prior results must stale after their evidence window |
| [`backlog_batched_watch.yaml`](./backlog_batched_watch.yaml) | `watch_controller_state_record` | `state_class = backlog`, pending changes exceed cycle budget and result surfaces carry backlog disclosure |
| [`degraded_adapter_watch.yaml`](./degraded_adapter_watch.yaml) | `watch_controller_state_record` | `state_class = degraded`, adapter health is narrowed while watch can continue with reduced confidence |
| [`unavailable_provider_watch.yaml`](./unavailable_provider_watch.yaml) | `watch_controller_state_record` | `state_class = unavailable`, provider-imported read-only evidence cannot produce new live cycles |
| [`inline_imported_ci_stale.yaml`](./inline_imported_ci_stale.yaml) | `inline_test_result_record` | provider-imported CI evidence whose evidence window expired; imported/stale chips required |
| [`inline_notebook_remapped_target_changed.yaml`](./inline_notebook_remapped_target_changed.yaml) | `inline_test_result_record` | notebook cell identity changed and target drifted; marker remains as stale/remapped rather than disappearing |
| [`environment_local_row.yaml`](./environment_local_row.yaml) | `environment_matrix_row_record` | local authoritative live row with exact-target comparability |
| [`environment_container_row.yaml`](./environment_container_row.yaml) | `environment_matrix_row_record` | container authoritative row with compatible-rerun comparability |
| [`environment_remote_row.yaml`](./environment_remote_row.yaml) | `environment_matrix_row_record` | remote agent row requiring manual re-resolve before rerun |
| [`environment_ci_row.yaml`](./environment_ci_row.yaml) | `environment_matrix_row_record` | CI imported read-only row with provider parity pending |
| [`environment_notebook_row.yaml`](./environment_notebook_row.yaml) | `environment_matrix_row_record` | notebook kernel row with notebook-vs-file divergence caution |
| [`environment_provider_backed_row.yaml`](./environment_provider_backed_row.yaml) | `environment_matrix_row_record` | provider-backed read-only row blocked from incompatible comparison |

## Coverage contract

The fixture set MUST keep:

- one watch-controller row for each state class `live`, `reduced`,
  `polling`, `paused`, `degraded`, `backlog`, and `unavailable`;
- at least one watch-controller row proving degradation is visible in
  result-consuming surfaces, not only diagnostics;
- at least one inline-result row with imported CI evidence that is not
  allowed to render as fresh local truth;
- at least one inline-result row preserving a moved/remapped or
  changed-target marker as a typed stale state;
- environment-matrix rows covering local, container, remote, CI,
  notebook, and provider-backed target classes;
- environment rows covering exact target, compatible rerun, manual
  re-resolve, comparison pending, caution, and blocked incompatible
  comparability postures.

Removing a layer of coverage is a breaking change.

## Pre-implementation note

At this milestone there is still no live watch controller, inline test
marker UI, notebook test runner, environment matrix card, or provider
parity surface wired up. These fixtures remain pre-implementation
governance artifacts.
