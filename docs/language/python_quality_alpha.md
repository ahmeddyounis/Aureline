# Python Formatter, Linter, and Test-Adapter Quality Alpha

This document records the first bounded Python quality-tool path in
`aureline-language`.

## Owned Runtime Surface

The runtime lives under `crates/aureline-language/src/python/quality/` and
exposes `PythonQualityWedge`. The wedge emits one
`python_quality_alpha_snapshot` containing:

- a shared `diagnostic_bus_snapshot` for formatter, linter, test-adapter, and
  interpreter-availability signals;
- tool-provider rows for Black, Ruff, and pytest-style lanes;
- execution-plane task hooks with command ids, execution-context refs,
  interpreter refs, diagnostic refs, and normalized task-event refs; and
- an execution-plane projection that joins visible diagnostics to rerunnable or
  blocked hooks.

Raw source, raw command lines, stdout/stderr bytes, and provider logs are not
embedded. Records carry opaque refs, tool/profile identity, interpreter state,
freshness, scope, support posture, and export-safe summaries.

## Scope And Fallback Rules

- Formatter, linter, and test-adapter signals enter the existing diagnostic bus
  instead of creating a parallel quality-problem model.
- Every task hook binds to the active execution context and selected Python
  interpreter before exposing a rerun action.
- Missing interpreter selection blocks formatter, lint, test discovery, and test
  run hooks with an interpreter-specific rerun posture.
- Whole-document formatting uses structured-diff preview posture when a mutation
  is possible.
- Read-only lint, test discovery, and test runs can rerun directly when the
  interpreter and provider are ready.
- Missing Ruff or pytest still produce provider state, a normalized diagnostic,
  and blocked task hooks; other quality lanes remain usable.
- Test failures stay `runtime_test_or_debug` evidence and remain distinct from
  static formatter or linter findings.

## Protected Proof Path

The protected fixture is
`fixtures/language/python_quality_alpha/quality_cases.json`. It covers:

- missing interpreter selection, where all quality hooks are blocked with an
  explicit interpreter posture;
- missing Ruff lint tooling while Black formatting and pytest hooks remain
  runnable; and
- missing pytest tooling while Black formatting and Ruff linting remain
  runnable.

Run:

```sh
cargo test -p aureline-language --test python_quality_alpha
python3 -m json.tool fixtures/language/python_quality_alpha/quality_cases.json >/dev/null
```
