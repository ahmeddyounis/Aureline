# TS/JS Formatter, Linter, and Test-Adapter Quality Alpha

This document records the first bounded TypeScript/JavaScript quality-tool path
in `aureline-language`.

## Owned Runtime Surface

The runtime lives under `crates/aureline-language/src/tsjs/quality/` and exposes
`TsJsQualityWedge`. The wedge emits one `tsjs_quality_alpha_snapshot` containing:

- a shared `diagnostic_bus_snapshot` for formatter, linter, and test-adapter
  signals;
- tool-provider rows for Prettier, ESLint, and Vitest-style lanes;
- execution-plane task hooks with command ids, execution-context refs,
  diagnostic refs, and normalized task-event refs; and
- an execution-plane projection that joins visible diagnostics to rerunnable or
  blocked hooks.

Raw source, raw command lines, stdout/stderr bytes, and provider logs are not
embedded. Records carry opaque refs, tool/profile identity, freshness, scope,
support posture, and export-safe summaries.

## Scope And Fallback Rules

- Formatter, linter, and test-adapter signals enter the existing diagnostic bus
  instead of creating a parallel quality-problem model.
- Every task hook binds to the active execution context and names the canonical
  command id that would rerun the quality action.
- Whole-document formatting uses preview posture when a mutation is possible.
- Read-only lint and test runs can rerun directly when the provider is ready.
- Missing or policy-blocked tools still produce provider state, a normalized
  diagnostic, and a blocked task hook; other quality lanes remain usable.
- Test failures stay `runtime_test_or_debug` evidence and remain distinct from
  static formatter or linter findings.

## Protected Proof Path

The protected fixture is
`fixtures/language/tsjs_quality_alpha/quality_cases.json`. It covers:

- a missing formatter where lint and test hooks remain runnable;
- a policy-blocked linter where formatter and test hooks remain runnable;
- normalized diagnostics for formatter, linter, and test-adapter lanes; and
- execution-plane projections that preserve provenance and rerun posture.

Run:

```sh
cargo test -p aureline-language --test tsjs_quality_alpha
python3 -m json.tool fixtures/language/tsjs_quality_alpha/quality_cases.json >/dev/null
```
