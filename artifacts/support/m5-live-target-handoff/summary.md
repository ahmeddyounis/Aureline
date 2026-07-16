# Live-Target Handoff Packets: One Validation Across Surfaces

- Packet: `m5-live-target-handoff:stable:0001`
- Surface: `M5 live-target handoff packets (one validation across surfaces)`
- Handoff bindings: 15 (11 blocked)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-16T00:00:00Z)

## Handoff bindings

- **Retirement / last-supported snapshot (handoff to current line)** [`lth-retirement-release`]: object `retirement_snapshot` on `release_center`, outcome `handoff_cleared`, route `in_process_workspace`, authority `read_only_inspect`->`read_only_inspect`, role `snapshot_labeling`
- **Retirement / last-supported snapshot (handoff to current line)** [`lth-retirement-shell`]: object `retirement_snapshot` on `shell`, outcome `blocked_needs_prerequisite`, route `in_process_workspace`, authority `read_only_inspect`->`read_only_inspect`, role `snapshot_labeling`
- **Retirement / last-supported snapshot (handoff to current line)** [`lth-retirement-cli`]: object `retirement_snapshot` on `cli_export`, outcome `blocked_by_policy`, route `in_process_workspace`, authority `read_only_inspect`->`read_only_inspect`, role `snapshot_labeling`
- **Captured support / export evidence (handoff to current object)** [`lth-support-evidence-support`]: object `support_export_evidence` on `support`, outcome `handoff_cleared`, route `remote_managed_service`, authority `scoped_edit`->`scoped_edit`, role `provenance_attribution`
- **Captured support / export evidence (handoff to current object)** [`lth-support-evidence-help`]: object `support_export_evidence` on `help_docs`, outcome `blocked_target_unavailable`, route `remote_managed_service`, authority `scoped_edit`->`scoped_edit`, role `provenance_attribution`
- **Captured support / export evidence (handoff to current object)** [`lth-support-evidence-companion`]: object `support_export_evidence` on `companion_export`, outcome `blocked_by_policy`, route `remote_managed_service`, authority `scoped_edit`->`scoped_edit`, role `provenance_attribution`
- **Archived runbook execution packet (handoff to current run)** [`lth-runbook-archive`]: object `archived_runbook_packet` on `runbook_archive`, outcome `handoff_cleared`, route `companion_browser_surface`, authority `elevated_admin`->`elevated_admin`, role `live_target_handoff`
- **Archived runbook execution packet (handoff to current run)** [`lth-runbook-review`]: object `archived_runbook_packet` on `review_incident`, outcome `blocked_target_unavailable`, route `companion_browser_surface`, authority `elevated_admin`->`elevated_admin`, role `live_target_handoff`
- **Archived runbook execution packet (handoff to current run)** [`lth-runbook-program`]: object `archived_runbook_packet` on `program_governance`, outcome `blocked_needs_prerequisite`, route `companion_browser_surface`, authority `elevated_admin`->`elevated_admin`, role `live_target_handoff`
- **Imported / offline route evidence (handoff to current route)** [`lth-imported-shell`]: object `imported_offline_route_evidence` on `shell`, outcome `blocked_target_unavailable`, route `cli_reopen_path`, authority `read_only_inspect`->`read_only_inspect`, role `imported_offline_disclosure`
- **Imported / offline route evidence (handoff to current route)** [`lth-imported-runbook`]: object `imported_offline_route_evidence` on `runbook_archive`, outcome `blocked_by_policy`, route `cli_reopen_path`, authority `read_only_inspect`->`read_only_inspect`, role `imported_offline_disclosure`
- **Imported / offline route evidence (handoff to current route)** [`lth-imported-cli`]: object `imported_offline_route_evidence` on `cli_export`, outcome `blocked_needs_prerequisite`, route `cli_reopen_path`, authority `read_only_inspect`->`read_only_inspect`, role `imported_offline_disclosure`
- **Review / incident snapshot (handoff to current object)** [`lth-review-review`]: object `review_incident_snapshot` on `review_incident`, outcome `handoff_cleared`, route `in_process_workspace`, authority `scoped_edit`->`scoped_edit`, role `mutation_blocked_posture`
- **Review / incident snapshot (handoff to current object)** [`lth-review-shell`]: object `review_incident_snapshot` on `shell`, outcome `blocked_target_unavailable`, route `in_process_workspace`, authority `scoped_edit`->`scoped_edit`, role `mutation_blocked_posture`
- **Review / incident snapshot (handoff to current object)** [`lth-review-companion`]: object `review_incident_snapshot` on `companion_export`, outcome `blocked_by_policy`, route `in_process_workspace`, authority `scoped_edit`->`scoped_edit`, role `mutation_blocked_posture`
