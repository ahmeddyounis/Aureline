# Historical-Evidence Drill Corpus: Fixtures and Regression Drills

- Packet: `m5-historical-evidence-drill:stable:0001`
- Surface: `M5 historical-evidence drill corpus (fixtures and regression drills)`
- Drill bindings: 15 (4 clear the live-target handoff)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-16T00:00:00Z)

## Drill bindings

- **Last-supported retirement snapshot fixture** [`hed-retirement-release`]: object `retirement_snapshot` on `release_center`, drill `preserved_live_target_handoff`, state `preserved_live_target_joinable`, outcome `handoff_cleared`, blocker `none_cleared`, content-available `true`
- **Last-supported retirement snapshot fixture** [`hed-retirement-shell`]: object `retirement_snapshot` on `shell`, drill `retired_line_reopen`, state `retired_line_no_live_counterpart`, outcome `blocked_needs_prerequisite`, blocker `route_unavailable`, content-available `true`
- **Last-supported retirement snapshot fixture** [`hed-retirement-cli`]: object `retirement_snapshot` on `cli_export`, drill `missing_live_target`, state `missing_live_target_metadata_only`, outcome `blocked_target_unavailable`, blocker `missing_target`, content-available `true`
- **Captured support / export evidence bundle fixture** [`hed-support-support`]: object `support_export_evidence` on `support`, drill `preserved_live_target_handoff`, state `preserved_live_target_joinable`, outcome `handoff_cleared`, blocker `none_cleared`, content-available `true`
- **Captured support / export evidence bundle fixture** [`hed-support-help`]: object `support_export_evidence` on `help_docs`, drill `expired_snapshot_metadata_only_fallback`, state `expired_snapshot_metadata_fallback`, outcome `blocked_by_policy`, blocker `expired_snapshot`, content-available `false`
- **Captured support / export evidence bundle fixture** [`hed-support-companion`]: object `support_export_evidence` on `companion_export`, drill `evidence_only_reopen_after_version_schema_drift`, state `imported_offline_evidence_only`, outcome `blocked_target_unavailable`, blocker `imported_offline_evidence_only`, content-available `true`
- **Runbook / incident archived packet fixture** [`hed-runbook-runbook`]: object `archived_runbook_packet` on `runbook_archive`, drill `preserved_live_target_handoff`, state `preserved_live_target_joinable`, outcome `handoff_cleared`, blocker `none_cleared`, content-available `true`
- **Runbook / incident archived packet fixture** [`hed-runbook-review`]: object `archived_runbook_packet` on `review_incident`, drill `stale_imported_evidence`, state `stale_imported_evidence`, outcome `blocked_needs_prerequisite`, blocker `trust_block`, content-available `true`
- **Runbook / incident archived packet fixture** [`hed-runbook-program`]: object `archived_runbook_packet` on `program_governance`, drill `missing_live_target`, state `missing_live_target_metadata_only`, outcome `blocked_target_unavailable`, blocker `missing_target`, content-available `true`
- **Imported / offline route packet fixture** [`hed-imported-shell`]: object `imported_offline_route_evidence` on `shell`, drill `evidence_only_reopen_after_version_schema_drift`, state `imported_offline_evidence_only`, outcome `blocked_target_unavailable`, blocker `imported_offline_evidence_only`, content-available `true`
- **Imported / offline route packet fixture** [`hed-imported-runbook`]: object `imported_offline_route_evidence` on `runbook_archive`, drill `stale_imported_evidence`, state `stale_imported_evidence`, outcome `blocked_needs_prerequisite`, blocker `trust_block`, content-available `true`
- **Imported / offline route packet fixture** [`hed-imported-cli`]: object `imported_offline_route_evidence` on `cli_export`, drill `expired_snapshot_metadata_only_fallback`, state `expired_snapshot_metadata_fallback`, outcome `blocked_by_policy`, blocker `expired_snapshot`, content-available `false`
- **Review / incident archived snapshot fixture** [`hed-review-review`]: object `review_incident_snapshot` on `review_incident`, drill `preserved_live_target_handoff`, state `preserved_live_target_joinable`, outcome `handoff_cleared`, blocker `none_cleared`, content-available `true`
- **Review / incident archived snapshot fixture** [`hed-review-shell`]: object `review_incident_snapshot` on `shell`, drill `retired_line_reopen`, state `retired_line_no_live_counterpart`, outcome `blocked_needs_prerequisite`, blocker `route_unavailable`, content-available `true`
- **Review / incident archived snapshot fixture** [`hed-review-companion`]: object `review_incident_snapshot` on `companion_export`, drill `evidence_only_reopen_after_version_schema_drift`, state `imported_offline_evidence_only`, outcome `blocked_target_unavailable`, blocker `imported_offline_evidence_only`, content-available `true`
