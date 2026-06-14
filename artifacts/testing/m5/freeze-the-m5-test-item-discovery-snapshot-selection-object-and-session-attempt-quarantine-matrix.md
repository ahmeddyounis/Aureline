# M5 Test-Intelligence Qualification Matrix

- Packet: `m5-test-qualification-matrix:stable:0001`
- Label: `M5 Test-Intelligence Qualification Matrix`
- Rows: 9 (9 claimed, 1 downgraded)
- Surfaces: 9 / 9
- Evidence freshness SLO: 168 hours (last refresh: 2026-06-13T00:00:00Z)

## Rows

- **test-row:framework-test-explorer:0001** (framework_test_explorer): claim `beta` -> effective `beta`
  - Framework test explorer with durable item identity, complete discovery, and a local live session
  - item=`stable` discovery=`complete_discovery` selection=`durable_identity_selection` session=`local_live_session` verdict=`not_imported_ci`
  - quarantine=`stable_again` proposals: none
- **test-row:notebook-test-cells:0001** (notebook_test_cells): claim `preview` -> effective `preview`
  - Notebook test cells with partial-but-visible discovery and a durable cell-identity selection
  - item=`stable` discovery=`partial_visible_discovery` selection=`durable_identity_selection` session=`local_live_session` verdict=`not_imported_ci`
  - quarantine=`stable_again` proposals: none
- **test-row:ai-test-generation:0001** (ai_test_generation): claim `beta` -> effective `beta`
  - AI test-generation surface whose generate/codemod proposals preview a diff and gate behind explicit apply
  - item=`stable` discovery=`complete_discovery` selection=`durable_identity_selection` session=`local_live_session` verdict=`not_imported_ci`
  - quarantine=`stable_again` proposals: generate_test, apply_codemod
- **test-row:review-test-panel:0001** (review_test_panel): claim `beta` -> effective `beta`
  - Review test panel reconciling local attempts with imported CI evidence over a query-matched selection
  - item=`stable` discovery=`complete_discovery` selection=`query_matched_selection` session=`mixed_local_imported_session` verdict=`authoritative_imported_read_only`
  - quarantine=`suspected_flaky` proposals: none
- **test-row:ci-import-overlay:0001** (ci_import_overlay): claim `beta` -> effective `beta`
  - Imported CI overlay with read-only item identity, provider-imported discovery, and a provider-scoped session
  - item=`imported_read_only` discovery=`provider_imported_discovery` selection=`provider_scoped_selection` session=`imported_ci_session` verdict=`authoritative_imported_read_only`
  - quarantine=`unknown` proposals: none
- **test-row:coverage-surface:0001** (coverage_surface): claim `stable` -> effective `stable`
  - Coverage surface with durable identity, complete discovery, and a rerun-last attempt lineage
  - item=`stable` discovery=`complete_discovery` selection=`durable_identity_selection` session=`rerun_attempt_lineage` verdict=`fresh_local_reconfirmation`
  - quarantine=`stable_again` proposals: none
- **test-row:flaky-quarantine-board:0001** (flaky_quarantine_board): claim `beta` -> effective `beta`
  - Flaky/quarantine board keeping a muted test visible, filterable, and exportable with renewal/expiry semantics
  - item=`stable` discovery=`complete_discovery` selection=`durable_identity_selection` session=`rerun_attempt_lineage` verdict=`not_imported_ci`
  - quarantine=`muted` proposals: none
- **test-row:snapshot-golden-review:0001** (snapshot_golden_review): claim `beta` -> effective `beta`
  - Snapshot/golden review whose accept-snapshot and update-golden proposals preview a diff before explicit apply
  - item=`stable` discovery=`complete_discovery` selection=`durable_identity_selection` session=`local_live_session` verdict=`not_imported_ci`
  - quarantine=`stable_again` proposals: accept_snapshot, update_golden
- **test-row:support-export:0001** (support_export_projection): claim `beta` -> effective `held`
  - Support/export projection of a test row whose session-attempt class is not yet identified
  - item=`stable` discovery=`complete_discovery` selection=`durable_identity_selection` session=`unidentified` verdict=`not_imported_ci`
  - quarantine=`stable_again` proposals: none
  - Degraded: Session-attempt class not yet identified for this projected row; held below preview until a session plan and attempt lineage are published
