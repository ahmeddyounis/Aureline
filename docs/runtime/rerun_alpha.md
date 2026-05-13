# Rerun Alpha

The runtime rerun lane lives in
[`crates/aureline-runtime/src/rerun`](../../crates/aureline-runtime/src/rerun/mod.rs).
It remembers the last task and last test launch contracts with the exact
`ExecutionContext` that produced each prior attempt.

Before `Rerun Last Task` or `Rerun Last Test` prepares a new attempt, callers
must pass a freshly resolved current execution context. The runtime emits a
`rerun_target_comparison_record` that compares exact-prior target truth against
current target truth across target class, canonical target id, cwd, reachability,
confidence, toolchain, trust, policy epoch, scope, prebuild metadata,
mixed-version posture, and degraded fields.

## Contract

- `cmd:task.rerun_last` and `cmd:test.rerun_last` are the canonical keyboard
  commands for launch wedges.
- `RerunTargetMode::ExactPriorTarget` preserves the prior execution-context
  target for the next attempt.
- `RerunTargetMode::CurrentResolvedTarget` updates the next attempt to the
  freshly resolved current target.
- Any exact-vs-current drift sets `review_required` before dispatch; convenience
  rerun paths may not hide target, helper, trust, or prebuild drift.
- Prepared attempts reuse the package-script and pytest run contracts, so run
  id stays stable while attempt id increments.

## Fixtures

Protected fixtures live under
[`fixtures/runtime/rerun_exact_vs_current`](../../fixtures/runtime/rerun_exact_vs_current):

- `last_task_current_target_drift.json`
- `last_test_exact_prior_target.json`

## Verify

```sh
cargo test -p aureline-runtime rerun
cargo test -p aureline-commands seeded_registry_loads
cargo test -p aureline-input preset
```
