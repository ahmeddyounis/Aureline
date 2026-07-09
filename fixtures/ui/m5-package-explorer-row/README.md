# M5 package explorer row fixtures

Protected fixtures for the `package_explorer_row` component implemented in
`aureline_deps::implement_package_explorer_rows_with_scope_relation_registry_and_signal_truth`.

Each fixture is an export-safe `PackageExplorerRowPacket` that validates against
[`schemas/ui/m5-package-explorer-row.schema.json`](../../../schemas/ui/m5-package-explorer-row.schema.json)
and passes `PackageExplorerRowPacket::validate`.

- `transitive_read_only.json` — spotlights a purely transitive row that names its
  parent instead of offering a direct install/update/remove action.
- `offline_snapshot_degraded.json` — an offline-snapshot resolution that never
  reads as a clean upstream install; the degradation note stays explicit.

Regenerate with:

```
GEN_PACKAGE_EXPLORER_ROW_ARTIFACTS=1 cargo test -p aureline-deps --lib \
  implement_package_explorer_rows_with_scope_relation_registry_and_signal_truth::tests::generate_artifacts
```
