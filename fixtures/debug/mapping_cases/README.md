# Debug artifact mapping-case corpus

This corpus carries the reviewer-facing mapping cases for the
`debug_artifact_entry_record` seed frozen in
[`/docs/debug/artifact_resolution_seed.md`](../../../docs/debug/artifact_resolution_seed.md).
Each case exercises one distinct axis the resolution contract must
keep distinguishable across native symbols (PDB / dSYM / DWARF),
JS / TS / CSS source maps, crash artifacts (minidump / core dump /
snapshot), generated-source mappings (with known and unknown spec),
and coverage / profile artifacts (workspace-attached and
side-loaded).

Companion artifacts:

- [`/schemas/debug/debug_artifact_manifest.schema.json`](../../../schemas/debug/debug_artifact_manifest.schema.json)
  — machine-readable boundary for the manifest and entry records
  the cases resolve into.
- [`/artifacts/debug/artifact_resolution_examples/`](../../../artifacts/debug/artifact_resolution_examples/)
  — concrete `debug_artifact_entry_record` examples that each case
  resolves, plus the worked primary manifest demonstrating parity.
- [`/docs/debug/artifact_resolution_seed.md`](../../../docs/debug/artifact_resolution_seed.md)
  — reviewer workflow, mismatch / degraded-quality semantics, and
  parity rules.
- [`/docs/build/exact_build_identity_model.md`](../../../docs/build/exact_build_identity_model.md)
  — exact-build identity source every release-anchored case joins
  to.
- [`/docs/support/exact_build_symbolication_smoke.md`](../../../docs/support/exact_build_symbolication_smoke.md)
  — local crash-symbolication smoke path the minidump case joins
  to.

## Files

| File | Purpose |
|---|---|
| [`mapping_cases_manifest.yaml`](./mapping_cases_manifest.yaml) | Overall corpus manifest: frozen vocabulary, case list, coverage contract. |
| [`native_symbol_pdb_resolved.json`](./native_symbol_pdb_resolved.json) | Release-anchored PDB resolved via release artifact graph. |
| [`native_symbol_dwarf_split_symbols_missing.json`](./native_symbol_dwarf_split_symbols_missing.json) | Linux DWARF sidecar with a missing split-symbols archive. |
| [`native_symbol_dsym_mismatch_build_id.json`](./native_symbol_dsym_mismatch_build_id.json) | Apple dSYM whose module UUID does not match the running binary. |
| [`source_map_js_stale_mapping.json`](./source_map_js_stale_mapping.json) | JS source map digest matches, mapping is stale. |
| [`source_map_css_partial_mapping.json`](./source_map_css_partial_mapping.json) | CSS source map with missing name fields. |
| [`crash_minidump_resolved_with_siblings.json`](./crash_minidump_resolved_with_siblings.json) | Minidump resolved alongside its crash-symbols archive. |
| [`crash_core_dump_pending_upload_consent.json`](./crash_core_dump_pending_upload_consent.json) | Core dump awaiting upload consent; stays local. |
| [`generated_source_openapi_resolved.json`](./generated_source_openapi_resolved.json) | OpenAPI-generated TypeScript client fully resolved. |
| [`generated_source_spec_unknown.json`](./generated_source_spec_unknown.json) | Vendored generated client with unknown spec. |
| [`coverage_lcov_workspace_output.json`](./coverage_lcov_workspace_output.json) | LCOV coverage from workspace output with branches unavailable. |
| [`profile_pprof_side_loaded.json`](./profile_pprof_side_loaded.json) | Side-loaded pprof profile under opt_in_only posture. |

## How to read a case

Each `*.json` file is a reviewer-facing record summarising the
scenario, the build-identity linkage, the generator identity when
applicable, and the expected terminal `resolution_state` with any
mismatch / degraded-quality tokens. Every case also carries the
`debug_artifact_ref` and the `parity_surfaces` block that pins the
same id on the debugger UI row, the support-bundle anchor, and the
release artifact-graph node.

The matching `debug_artifact_entry_record` under
[`/artifacts/debug/artifact_resolution_examples/`](../../../artifacts/debug/artifact_resolution_examples/)
is the full record the resolver emits. A parity audit compares the
case file and the entry record token-for-token.
