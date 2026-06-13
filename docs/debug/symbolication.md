# Symbolication manifests, reports, and cross-surface fidelity contract

This contract freezes the M5 symbolication truth shared by debug,
profiler/replay, preview, browser-runtime, and support/export
surfaces.

Authoritative product anchors:

- `.t2/docs/Aureline_PRD.md` section 5.43 and Appendix AB.
- `.t2/docs/Aureline_Technical_Design_Document.md` section 7.12.7.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections 17.30 and 23.66.

This contract also composes with the repo-native truth already frozen in:

- [`docs/debug/artifact_resolution_seed.md`](./artifact_resolution_seed.md)
- [`docs/execution/debug_truth_contract.md`](../execution/debug_truth_contract.md)
- [`schemas/debug/debug_artifact_manifest.schema.json`](../../schemas/debug/debug_artifact_manifest.schema.json)
- [`schemas/execution/mapping_quality.schema.json`](../../schemas/execution/mapping_quality.schema.json)
- [`schemas/execution/crash_dump_card.schema.json`](../../schemas/execution/crash_dump_card.schema.json)

## Scope

The packet owned by `crates/aureline-debug/src/symbolication/mod.rs` binds:

- symbol or source-map manifests to exact-build identity, build id,
  target triple, debug format, artifact digest, retention posture, and
  local/mirrored/imported source identity;
- mirror-policy rows to signer identity, access policy, retention, and
  the local-first rule that mirrored lookup only follows a local miss;
- local or mirrored symbolication reports to the exact-build match
  state, the sources used, unresolved-frame counts, and the redaction
  class; and
- one shared fidelity vocabulary across downstream surfaces:
  `exact`, `approximate`, `symbol_only`, and `unresolved`.

The packet does not parse dumps, fetch live symbols, or implement a
hosted symbol server. It keeps the reviewed truth packet that those
systems must emit and consume.

## Contract rules

- Exact-build symbolication is fail-closed. A report may claim
  `exact` only when its build-match state is
  `exact_build_verified` and every counted frame is exact.
- Approximate mapping stays visible as approximate. It may use a local
  or mirrored candidate, but it never pretends stale, line-only, or
  digest-drifted mapping is exact.
- Symbol-only mapping names symbol identity without claiming source
  lines.
- Unresolved mapping covers no-candidate and rejected-mismatch paths.
  Exact-build mismatches remain visible before navigation.
- Mirrored symbol or source-map use is always disclosed. A mirrored
  row must cite a mirror-policy record and that policy must declare
  `requires_local_miss_before_lookup = true`.
- Support/export surfaces preserve the same symbol-source, fidelity,
  build-match, unresolved-count, and redaction truth as debug and
  profiler surfaces.

## First consumers

- debug frame stack and crash/dump inspectors;
- profiler hotspot and trace-viewer rows;
- preview and browser-runtime inspect-to-source rows; and
- support export and incident/crash review packets.

## Checked-in artifacts

- Packet:
  [`artifacts/debug/symbolication_contract.json`](../../artifacts/debug/symbolication_contract.json)
- Evidence note:
  [`artifacts/debug/symbolication_contract.md`](../../artifacts/debug/symbolication_contract.md)
- Schema:
  [`schemas/debug/symbolication_contract.schema.json`](../../schemas/debug/symbolication_contract.schema.json)
- Fixtures:
  [`fixtures/debug/symbolication/`](../../fixtures/debug/symbolication/)
