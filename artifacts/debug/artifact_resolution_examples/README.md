# Debug artifact resolution examples

Concrete `debug_artifact_entry_record` examples for every mapping
case plus the worked `debug_artifact_manifest_record` that binds
debugger UI, support bundle, and release artifact graph surfaces to
the same debug-artifact refs.

Companion artifacts:

- [`/schemas/debug/debug_artifact_manifest.schema.json`](../../../schemas/debug/debug_artifact_manifest.schema.json)
  — machine-readable boundary for the manifest and entry records.
- [`/fixtures/debug/mapping_cases/`](../../../fixtures/debug/mapping_cases/)
  — reviewer-facing mapping-case records each entry resolves.
- [`/docs/debug/artifact_resolution_seed.md`](../../../docs/debug/artifact_resolution_seed.md)
  — reviewer workflow, resolution rules, and parity-seed semantics.

## Files

| File | Case id |
|---|---|
| [`primary_manifest.json`](./primary_manifest.json) | `debug.manifest.aureline.parity_seed.v1` — worked manifest bundling headline entries. |
| [`entry_native_symbol_pdb_resolved.json`](./entry_native_symbol_pdb_resolved.json) | `debug.mapping.native_symbol_pdb.resolved` |
| [`entry_native_symbol_dwarf_split_symbols_missing.json`](./entry_native_symbol_dwarf_split_symbols_missing.json) | `debug.mapping.native_symbol_dwarf.split_symbols_missing` |
| [`entry_native_symbol_dsym_mismatch_build_id.json`](./entry_native_symbol_dsym_mismatch_build_id.json) | `debug.mapping.native_symbol_dsym.mismatch_build_id` |
| [`entry_source_map_js_stale_mapping.json`](./entry_source_map_js_stale_mapping.json) | `debug.mapping.source_map_js.stale_mapping` |
| [`entry_source_map_css_partial_mapping.json`](./entry_source_map_css_partial_mapping.json) | `debug.mapping.source_map_css.partial_mapping` |
| [`entry_crash_minidump_resolved_with_siblings.json`](./entry_crash_minidump_resolved_with_siblings.json) | `debug.mapping.crash_minidump.resolved_with_siblings` |
| [`entry_crash_core_dump_pending_upload_consent.json`](./entry_crash_core_dump_pending_upload_consent.json) | `debug.mapping.crash_core_dump.pending_upload_consent` |
| [`entry_generated_source_openapi_resolved.json`](./entry_generated_source_openapi_resolved.json) | `debug.mapping.generated_source.openapi_resolved` |
| [`entry_generated_source_spec_unknown.json`](./entry_generated_source_spec_unknown.json) | `debug.mapping.generated_source.spec_unknown` |
| [`entry_coverage_lcov_workspace_output.json`](./entry_coverage_lcov_workspace_output.json) | `debug.mapping.coverage_lcov.workspace_output` |
| [`entry_profile_pprof_side_loaded.json`](./entry_profile_pprof_side_loaded.json) | `debug.mapping.profile_pprof.side_loaded` |

## Reading an entry

Each entry conforms to the `debug_artifact_entry_record` in
[`/schemas/debug/debug_artifact_manifest.schema.json`](../../../schemas/debug/debug_artifact_manifest.schema.json).
Every entry declares:

- a stable `debug_artifact_ref` used by every consuming surface;
- the `workspace_binding` (workspace / target / run context / profile);
- the `build_identity_linkage` (exact-build, baseline-build,
  external-build, or pending identity) plus
  `expected_build_match_fields` the resolver must match;
- the ordered `resolution_sources` the resolver consulted with each
  source's `trust_state`;
- the terminal `resolution_state` (resolved, resolved_degraded_quality,
  one of the unresolved_* states, or pending_capture) with the
  matching closed `mismatch_reasons` and `degraded_quality_reasons`;
- the `generator_identity` when the artifact class names one
  (generated sources, source maps, coverage, profile);
- a `content_linkage` block (digest, content-addressed ref, module
  identity, sibling identity refs);
- the `storage_mode`, `support_export_posture`, and
  `redaction_class` the support bundle preview and release-evidence
  surface honour; and
- the `surface_linkage` block that pins the debugger UI row, the
  support-bundle anchor, the release artifact-graph node (when the
  entry is release-graph-bearing), and — for crash artifacts — the
  crash-envelope and symbolication-report refs the entry joins.

## Parity check

The primary manifest and the individual entry records share the same
`debug_artifact_ref` for each logical entry. A parity audit walks the
manifest entries, the corresponding fixture case files under
[`/fixtures/debug/mapping_cases/`](../../../fixtures/debug/mapping_cases/),
and the individual entry records here, and verifies token-for-token:

- `debug_artifact_ref`;
- `artifact_class`;
- `resolution_state`;
- `mismatch_reasons` and `degraded_quality_reasons`;
- `build_identity_linkage.expected_build_match_fields`;
- `surface_linkage.*` refs.

A surface that drifts from any of these tokens is non-conforming.
