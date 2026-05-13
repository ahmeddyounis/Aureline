# Execution Entry Truth Alpha

This lane exposes the canonical execution-context summary across the shell
entry points that can run or prepare work: terminal, task, test, debug-prep,
and AI-tool surfaces.

## Contract

The resolver remains owned by
[`crates/aureline-runtime/src/execution_context`](../../crates/aureline-runtime/src/execution_context/mod.rs).
The shell projection lives in
[`crates/aureline-shell/src/run_context`](../../crates/aureline-shell/src/run_context/mod.rs)
and does not mint target, toolchain, trust, prebuild, helper-boundary, or
provenance vocabulary.

Every entry surface consumes a `run_context_summary_record` with:

- target class, canonical target id, working directory, and boundary cue;
- toolchain class, toolchain id, and resolved version;
- trust state, identity mode, policy epoch, and workset scope;
- target confidence reasons, prebuild reuse metadata, cache disposition, and
  mixed-version helper posture;
- resolver input decisions, provenance id, resolver version, degraded-field
  reasons, and structured explanation reason codes.

The existing task and debug-prep seed records embed this summary. The terminal
pane can join it by `execution_context_ref`. Test and AI-tool entries use the
same `execution_entry_surface_record` projection until deeper dedicated
surfaces land.

## Exact Rerun Diff

`run_context_comparison_record` compares the exact-rerun context with the
current environment before dispatch. Diff rows use the same field paths and
tokens as the summary, including target, cwd, toolchain, trust, policy epoch,
scope, cache, prebuild, mixed-version, and degraded-field tokens. Any changed
row sets `requires_review_before_dispatch` so the UI can show the drift before
the user acts.

## Fixtures

Protected fixtures live under
[`fixtures/runtime/execution_entry_points`](../../fixtures/runtime/execution_entry_points):

- `all_entry_points_share_summary.json` proves all five launch-capable entry
  points consume the same summary shape.
- `exact_rerun_current_drift.json` proves exact-rerun versus current
  environment drift is visible before dispatch.

## Verify

```sh
cargo test -p aureline-shell run_context
cargo test -p aureline-shell tasks_seed::tests debug_seed::tests
cargo test -p aureline-shell terminal_pane::tests::snapshot_projects_active_session_without_degraded_chip
```
