# M5 Test-Explorer / Watch / Triage Component Consumers

- Packet: `m5-test-component-consumers:stable:0001`
- As of: `2026-07-07T00:00:00Z`
- Rows: 11 across 4 consumer classes and 7 / 7 frozen families
- Families reused across classes: 4
- Imported + local-live both present: true

## Rows

- **consumer:status-bar:session-summary-bar** (Session-summary bar) — surface=status_bar_summary class=day_to_day_editor family=session_summary_bar origin=live_local label_parity=preserved narrow=full
- **consumer:activity-center:test-tree-row** (Test-tree row) — surface=activity_center class=day_to_day_editor family=test_tree_row origin=live_local label_parity=preserved narrow=full
- **consumer:coverage:inline-result-marker** (Inline result marker) — surface=coverage_intelligence class=quality_intelligence family=inline_result_marker origin=imported_ci label_parity=disclosed_narrowed narrow=results_imported
- **consumer:flaky:failure-triage-panel** (Failure-triage panel) — surface=flaky_intelligence class=quality_intelligence family=failure_triage_panel origin=live_local label_parity=preserved narrow=full
- **consumer:snapshot:environment-matrix-card** (Environment-matrix card) — surface=snapshot_review class=quality_intelligence family=environment_matrix_card origin=live_local label_parity=disclosed_narrowed narrow=target_compatibility_drift
- **consumer:pipeline:watch-mode-banner** (Watch-mode banner) — surface=pipeline_overlay class=pipeline_imported family=watch_mode_banner origin=imported_ci label_parity=disclosed_narrowed narrow=results_imported+watch_fidelity_degraded
- **consumer:imported-ci:test-tree-row** (Test-tree row) — surface=imported_ci_view class=pipeline_imported family=test_tree_row origin=imported_ci label_parity=disclosed_narrowed narrow=results_imported
- **consumer:imported-ci:session-summary-bar** (Session-summary bar) — surface=imported_ci_view class=pipeline_imported family=session_summary_bar origin=imported_teammate label_parity=disclosed_narrowed narrow=results_imported
- **consumer:support:quarantine-review-sheet** (Quarantine-review sheet) — surface=support_packet class=support_export family=quarantine_review_sheet origin=live_local label_parity=disclosed_narrowed narrow=quarantine_visibility_restricted
- **consumer:support:failure-triage-panel** (Failure-triage panel) — surface=support_packet class=support_export family=failure_triage_panel origin=replayed_snapshot label_parity=disclosed_narrowed narrow=results_imported
- **consumer:support:environment-matrix-card** (Environment-matrix card) — surface=support_packet class=support_export family=environment_matrix_card origin=imported_teammate label_parity=disclosed_narrowed narrow=results_imported+target_compatibility_drift
