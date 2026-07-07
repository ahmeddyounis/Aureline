# M5 test-explorer / watch / triage component consumer fixtures (M05-913)

These fixtures are the checked-in projection of the M05-913 consumer-adoption
packet that proves the seven frozen M5 test components (test-tree row, inline
result marker, session-summary bar, watch-mode banner, failure-triage panel,
quarantine-review sheet, and environment-matrix card) are reused across the
status-bar summary, activity center, coverage / flaky / snapshot intelligence,
pipeline overlays, imported-CI views, and support packets — keeping result
freshness, target class, watch state, quarantine semantics, and
imported-versus-live result origin aligned, and auto-narrowing the visible claim
when a result is imported, a target drifts, watch fidelity degrades, or
quarantine visibility is restricted.

- `support_export.json` — the metadata-only support export (mirrors
  `artifacts/release/m5-test-component-consumer-proof/support_export.json`).
- `matrix.csv` — the machine-readable adoption matrix.

Regenerate with:

```sh
cargo run -p aureline-runtime --bin aureline_runtime_test_component_consumers -- support-export
cargo run -p aureline-runtime --bin aureline_runtime_test_component_consumers -- csv
```

The canonical source of truth is `seeded_m5_test_component_consumers_packet()` in
the `add_shared_status_bar_activity_center_coverage_flaky_snapshot_pipeline_imported_ci_and_support_consumers_...`
module; the checked-in JSON must stay byte-aligned with it (enforced by
`checked_in_export_matches_seeded_builder`).
