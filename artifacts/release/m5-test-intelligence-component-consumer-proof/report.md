# M5 Test-Intelligence Component Consumers

- Packet: `m5-test-intelligence-component-consumers:stable:0001`
- As of: `2026-07-10T00:00:00Z`
- Rows: 12 across 6 consumer classes and 7 / 7 frozen families
- Families reused across classes: 5
- Imported + verified current-run both present: true

## Rows

- **consumer:editor-gutter:coverage-overlay-marker** (Coverage-overlay marker) — surface=editor_gutter_overlay class=editor_surface family=coverage_overlay_marker provenance=verified_current_run label_parity=preserved narrow=full
- **consumer:editor-summary:coverage-summary-bar** (Coverage-summary bar) — surface=editor_coverage_summary class=editor_surface family=coverage_summary_bar provenance=verified_current_run label_parity=disclosed_narrowed narrow=shard_scope_omitted
- **consumer:test-tree:flaky-state-badge** (Flaky-state badge) — surface=test_tree_panel class=test_tree family=flaky_state_badge provenance=suspected_flaky label_parity=disclosed_narrowed narrow=flakiness_unconfirmed
- **consumer:test-tree:retry-history-row** (Retry-history row) — surface=test_tree_panel class=test_tree family=retry_history_row provenance=verified_current_run label_parity=preserved narrow=full
- **consumer:review-diff:coverage-summary-bar** (Coverage-summary bar) — surface=review_coverage_diff class=review_surface family=coverage_summary_bar provenance=imported_ci_artifact label_parity=disclosed_narrowed narrow=evidence_imported
- **consumer:review-snapshot:snapshot-review-card** (Snapshot-review card) — surface=review_snapshot_card class=review_surface family=snapshot_review_card provenance=verified_current_run label_parity=preserved narrow=full
- **consumer:cli-summary:coverage-import-merge-sheet** (Coverage-import / merge sheet) — surface=cli_quality_summary class=cli_summary family=coverage_import_merge_sheet provenance=imported_ci_artifact label_parity=disclosed_narrowed narrow=evidence_imported+shard_scope_omitted
- **consumer:cli-summary:test-generation-suggestion-card** (Test-generation suggestion card) — surface=cli_quality_summary class=cli_summary family=test_generation_suggestion_card provenance=verified_current_run label_parity=disclosed_narrowed narrow=generated_assumptions_unverified
- **consumer:imported-ci:coverage-overlay-marker** (Coverage-overlay marker) — surface=imported_ci_detail_view class=imported_ci_detail family=coverage_overlay_marker provenance=imported_ci_artifact label_parity=disclosed_narrowed narrow=evidence_imported
- **consumer:imported-ci:flaky-state-badge** (Flaky-state badge) — surface=imported_ci_detail_view class=imported_ci_detail family=flaky_state_badge provenance=reproduced_flaky label_parity=preserved narrow=full
- **consumer:support:snapshot-review-card** (Snapshot-review card) — surface=support_export_packet class=support_export family=snapshot_review_card provenance=stale_prior_result label_parity=disclosed_narrowed narrow=provenance_stale
- **consumer:support:test-generation-suggestion-card** (Test-generation suggestion card) — surface=support_export_packet class=support_export family=test_generation_suggestion_card provenance=cached_local_result label_parity=disclosed_narrowed narrow=provenance_stale+generated_assumptions_unverified
