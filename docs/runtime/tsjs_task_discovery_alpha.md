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
- exposes all script source refs admitted by the bounded manifest envelope,
  including non-runnable scripts;
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

## Discovery Resource And Trust Boundary

Runtime metadata reads are regular-file-only, UTF-8, identity-checked before
and after the read, and contained by the canonical workspace root. Discovery
does not follow manifest, version-file, lockfile, or parent-directory symlinks.
`package.json` and other Node/toolchain metadata files are limited to 2 MiB.
If a known manifest, version file, manager configuration, or lockfile is
present but cannot satisfy that boundary, the affected runtime or package
manager resolves to `unsupported`; an ambient/profile tool is not allowed to
silently replace the unreadable workspace expectation. The detector retains an
export-safe `unreadable_source` provenance card for repair and review. Runtime
and manager tokens are limited to 512 bytes and reject control,
directional-formatting, and zero-width formatting characters without echoing
the rejected value into provenance or diagnostics.
The scripts object is limited to 4,096 entries, script names to 256 bytes, and
individual script bodies to 16 KiB. A non-string script value or any exceeded
bound rejects the script map instead of executing or exporting a partial,
silently narrowed set. The result stays non-runnable with an honesty marker and
a repair-safe error; raw manifest or script bodies are not copied into the
error.

`package_script_discovery_record` schema version 2 removes the raw
`script_body` field from script descriptors. Version-1 JSON remains readable by
version-2 consumers because the old field is ignored, but version-2 writers do
not recreate or export it. Consumers that need to inspect a script body must
reopen the cited JSON pointer through the governed workspace/VFS path after the
applicable trust decision; discovery, task events, logs, and support exports
retain only the script name, source ref, classification, and digest-bearing
event metadata.

Runnable contracts retain the full discovered script map only as redacted,
in-memory authority. The authority is deliberately omitted from serialized
records. The execution boundary must call
`revalidate_workspace_manifest` (or the combined
`launch_event_stream_for_workspace`) immediately before dispatch; missing
authority, unreadable metadata, or any changed script or lifecycle hook fails
closed and requires rediscovery. The serialized `dispatch` member is review
evidence; Rust consumers cannot read it directly and must use
`validated_dispatch` to obtain executable argv. Control characters and
directional or zero-width formatting controls are rejected in script names
before those names can enter source refs, argv, event summaries, logs, or
support exports.

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
