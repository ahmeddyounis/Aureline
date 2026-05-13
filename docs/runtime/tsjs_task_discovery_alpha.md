# TS/JS Package-Script Task Discovery Alpha

This lane adds a read-only package-script discovery contract for TS/JS
launch-wedge workspaces. The implementation lives in
[`crates/aureline-runtime/src/discovery/package_scripts`](../../crates/aureline-runtime/src/discovery/package_scripts)
and emits `package_script_discovery_record` plus
`package_script_run_contract_record` payloads.

## Contract

The discoverer:

- resolves a package-script execution context through
  `ExecutionContextRequest::package_script_task_seed`;
- attaches the existing Node detector report to the canonical execution
  context;
- parses `package.json#scripts` with structured JSON parsing;
- exposes all script source refs, including non-runnable scripts;
- creates run contracts only for the bounded launch-wedge set:
  `build`, `test`, `test:*`, `typecheck`, `lint`, `dev`, `start`, and
  matching build/lint/typecheck prefixed variants;
- records missing, ambiguous, or unsupported Node/package-manager states
  before dispatch;
- launches by direct package-manager argv, for example
  `program = "pnpm"`, `args = ["run", "build"]`.

The run contract does not store a shell command string and does not wrap
scripts in `sh -c` or platform-specific terminal glue. Adjacent lifecycle
hooks such as `prebuild` or `postbuild` are disclosed as source refs so the
package manager’s script lifecycle is not hidden.

## Task-Event Consumer

`PackageScriptRunContract::launch_event_stream` is the first consumer surface.
Ready contracts project to the canonical task stream as queued + started
events. Blocked contracts project to queued + blocked events with the same
workspace, run, attempt, target, trace, execution-context, raw-envelope, shell,
activity, and support-export contracts used by other task lanes.

`PackageScriptRunContract::rerun_with_context` keeps the run id stable,
increments the attempt id, and records exact-vs-current context drift when a
freshly resolved current context differs from the original attempt.

## Fixtures

Protected fixtures live under
[`fixtures/runtime/tsjs_task_discovery_alpha`](../../fixtures/runtime/tsjs_task_discovery_alpha):

- `ready_pnpm`
- `missing_node_runtime`
- `unsupported_yarn`

## Verify

```sh
cargo test -p aureline-runtime package_scripts
```
