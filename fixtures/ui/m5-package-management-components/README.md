# M5 package-management component fixtures

Protected fixtures for row M05-972 (batch B115). Each file is a full, valid
`M5PackageComponentMatrixPacket` that exercises a narrowed package-management
scenario. They validate under
`schemas/ui/m5-package-management-component-matrix.schema.json` and are loaded by
the module's `checked_narrowed_fixtures_validate` test.

- `script_risk_notice_unknown_hook.json` — the script-risk notice narrows its
  registry-degradation vocabulary to `unknown_or_stale` + `mirror_backed`,
  proving unknown install-hook risk stays labeled rather than downgraded to "no
  scripts".
- `grouped_update_planner_offline_snapshot.json` — the grouped-update planner
  narrows to `offline_snapshot_only` + `unknown_or_stale`, proving a batch update
  planned against an offline snapshot never claims upstream freshness.

Regenerate with:

```
GEN_PACKAGE_MANAGEMENT_COMPONENT_ARTIFACTS=1 \
  cargo test -p aureline-deps --lib \
  freeze_the_m5_package_management_component_matrix::tests::generate_artifacts
```
