# Debug session, mapping-quality, and crash-dump card cases

This corpus carries the reviewer-facing fixtures for the debug-
session truth contract frozen in
[`/docs/execution/debug_truth_contract.md`](../../../docs/execution/debug_truth_contract.md).
Each case exercises one distinct axis the contract must keep
distinguishable across launch / attach, live-shared, captured /
restored / imported postures and across exact / approximate /
imported-symbol / build-mismatch mapping-quality classes.

Companion artifacts:

- [`/schemas/execution/debug_session.schema.json`](../../../schemas/execution/debug_session.schema.json)
  — machine-readable boundary for the debug-session record family.
- [`/schemas/execution/mapping_quality.schema.json`](../../../schemas/execution/mapping_quality.schema.json)
  — closed mapping-quality vocabulary every frame and source-jump
  resolves through.
- [`/schemas/execution/crash_dump_card.schema.json`](../../../schemas/execution/crash_dump_card.schema.json)
  — captured / restored / imported dump-card record.
- [`/schemas/debug/debug_artifact_manifest.schema.json`](../../../schemas/debug/debug_artifact_manifest.schema.json)
  — symbol / source-map / crash artifact resolution boundary the
  truth contract joins to.
- [`/docs/debug/artifact_resolution_seed.md`](../../../docs/debug/artifact_resolution_seed.md)
  — resolution seed every fixture's `debug_artifact_ref` pins to.

## Files

| File | Purpose |
|---|---|
| [`exact_build_local_attach.yaml`](./exact_build_local_attach.yaml) | Exact-build local attach with full symbols, held-pending-purity getter, and secret-class redacted watch row. |
| [`source_map_mismatch.yaml`](./source_map_mismatch.yaml) | Source-map digest mismatch; navigation denied; address-only fallback. |
| [`imported_symbols.yaml`](./imported_symbols.yaml) | Third-party crate frame; `mapping_imported_symbol_external_dependency_only`; opens an inspect-only summary card. |
| [`captured_minidump.yaml`](./captured_minidump.yaml) | Local Windows minidump; no-upload posture; metadata-only redaction; restart / reattach denied at the parent session. |
| [`shared_debug_overlay.yaml`](./shared_debug_overlay.yaml) | Follower view of a shared-debug overlay; inspect-only posture preserving the presenter's core truth. |

## How to read a case

Each `*.yaml` is a worked record under one of the three schemas in
`schemas/execution/debug_*` or `schemas/execution/mapping_*`. The
`__fixture__` block names the scenario and the exercised vocabulary.
The body is the record itself — every field that crosses the schema
boundary is admissible at this point in the contract.

Every fixture asserts the parity rules:

- the same `debug_artifact_ref` resolves on the debugger UI row, the
  support-bundle anchor, and the release artifact-graph node;
- captured / restored / imported postures forbid silent restart /
  reattach / step / continue;
- build-mismatched mappings forbid silent navigation;
- imported-symbol frames never collapse into exact-source frames.
