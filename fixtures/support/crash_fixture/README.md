# Exact-build crash symbolication smoke fixture

This corpus proves one minimal crash-to-report path using the same
exact-build fields the release artifacts already publish. The fixture is
local-only, synthetic, and intentionally small; it exists to exercise
the contracts, not to model a full crash-upload service.

Companion artifacts:

- [`/tools/support/symbolicate_smoke.sh`](../../../tools/support/symbolicate_smoke.sh)
  — local fail-closed smoke runner.
- [`/docs/support/exact_build_symbolication_smoke.md`](../../../docs/support/exact_build_symbolication_smoke.md)
  — reviewer workflow and failure semantics.
- [`/artifacts/support/crash_artifact_retention_seed.json`](../../../artifacts/support/crash_artifact_retention_seed.json)
  — crash-artifact retention and redaction seed reused by the report.
- [`/fixtures/build/exact_build_examples/`](../../build/exact_build_examples/)
  — the canonical exact-build identity fixtures the smoke path resolves.

## Files

| File | Purpose |
|---|---|
| [`crash_envelope.json`](./crash_envelope.json) | synthetic crash envelope carrying the primary exact-build ref, copied build fields, module identities, and chronology state |
| [`crash_envelope_build_mismatch.json`](./crash_envelope_build_mismatch.json) | intentional mismatch case that must fail closed |
| [`crash_dump_manifest.json`](./crash_dump_manifest.json) | metadata-only dump/core manifest for the captured crash artifact |
| [`symbolication_input_manifest.json`](./symbolication_input_manifest.json) | placeholder symbol/source-map wiring that points at the exact-build identity fixtures and retention rows |
| [`expected_symbolication_report.json`](./expected_symbolication_report.json) | deterministic report emitted by the smoke script on the happy path |

## Happy path

```sh
./tools/support/symbolicate_smoke.sh --out-dir target/symbolication-smoke
diff -u fixtures/support/crash_fixture/expected_symbolication_report.json target/symbolication-smoke/symbolication_report.json
```

## Fail-closed path

```sh
./tools/support/symbolicate_smoke.sh \
  --crash-envelope fixtures/support/crash_fixture/crash_envelope_build_mismatch.json \
  --out-dir target/symbolication-smoke-mismatch
```

The second command must exit non-zero and name the mismatched
exact-build field.
