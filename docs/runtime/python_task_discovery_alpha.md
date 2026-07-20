# Python Pytest Task Discovery Alpha

This lane adds read-only pytest discovery and run contracts for Python
launch-wedge workspaces. The implementation lives in
[`crates/aureline-runtime/src/discovery/pytest`](../../crates/aureline-runtime/src/discovery/pytest)
and emits `pytest_discovery_record` plus `pytest_run_contract_record`
payloads.

## Contract

The discoverer:

- resolves a test execution context through `ExecutionContextRequest::test_seed`;
- attaches the existing Python environment detector report to the canonical
  execution context;
- statically discovers pytest-compatible `test_*.py` and `*_test.py` files;
- records top-level `test_*` functions and `Test*::test_*` methods as stable
  pytest selectors;
- creates an all-discovered-tests contract plus per-item contracts so rerun
  actions do not require manual selector re-entry;
- records missing, ambiguous, or unsupported Python interpreter and environment
  manager states before dispatch;
- launches by direct runner argv, such as `uv run pytest <selector>`,
  `poetry run pytest <selector>`, or `<python> -m pytest <selector>`.

The run contract stores program, argv, working directory, invocation mode,
source refs, and selector identity. It does not store a shell command string or
hide fallback interpreter, target, trust, or policy state.

## Discovery Resource And Trust Boundary

Python/toolchain metadata and pytest source reads are regular-file-only, UTF-8,
identity-checked before and after the read, and contained by the canonical
workspace root. Manifest, version-file, config, test-file, and directory
symlinks are never followed. Each metadata or pytest source file is limited to
2 MiB.

If a known Python manifest, version file, environment marker, lockfile, or
virtual-environment path is present but cannot satisfy that boundary, the
affected interpreter or environment manager resolves to `unsupported`;
ambient/profile tools cannot silently replace unreadable workspace intent. An
export-safe `unreadable_source` provenance card records the repair reason.
Version tokens are limited to 512 bytes and interpreter references to 4,096
bytes; control, directional-formatting, and zero-width formatting characters
are rejected without echoing the value into provenance or diagnostics.

The pytest crawl inspects at most 100,000 directory entries, queues at most
20,000 canonical directories, retains at most 10,000 test files and 20,000 test
items, reads at most 32 MiB of source in total, and accepts relative paths no
longer than 2,048 bytes. At most 256 discovery issues are retained. Reaching a
bound produces `partial` discovery with an export-safe omitted-scope reason;
the retained selectors remain usable, while omitted scope is never represented
as complete or authoritative discovery. Relative paths containing control,
directional-formatting, or zero-width formatting characters are omitted before
they can become selectors, event fields, logs, or support-export source refs.

## Task-Event Consumer

`PytestRunContract::launch_event_stream` projects ready contracts to the
canonical task stream as queued + started events. Blocked contracts project to
queued + blocked events with the same workspace, run, attempt, target, trace,
execution-context, raw-envelope, shell, activity, and support-export contracts
used by other task lanes.

`PytestRunContract::rerun_with_context` keeps the run id and pytest selector
stable, increments the attempt id, and records exact-vs-current context drift
when a freshly resolved current context differs from the original attempt.

## Fixtures

Protected fixtures live under
[`fixtures/runtime/python_task_discovery_alpha`](../../fixtures/runtime/python_task_discovery_alpha):

- `ready_uv`
- `missing_python_runtime`
- `unsupported_conda`

## Verify

```sh
cargo test -p aureline-runtime pytest
```
