# Archived-Snapshot Viewers & Analysis-Only Banners: One Vocabulary Across Surfaces

- Packet: `m5-archived-snapshot-viewer-consumers:stable:0001`
- Surface: `M5 archived-snapshot viewers & analysis-only banners (one vocabulary across surfaces)`
- Consumer bindings: 15 (10 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-16T00:00:00Z)

## Consumer bindings

- **Retirement / last-supported snapshot (analysis-only archive)** [`asvc-retirement-release`]: object `retirement_snapshot` on `release_center`, posture `live_target_openable`, role `snapshot_labeling`
- **Retirement / last-supported snapshot (analysis-only archive)** [`asvc-retirement-shell`]: object `retirement_snapshot` on `shell`, posture `live_target_openable`, role `snapshot_labeling`
- **Retirement / last-supported snapshot (analysis-only archive)** [`asvc-retirement-cli`]: object `retirement_snapshot` on `cli_export`, posture `exported_redacted`, role `snapshot_labeling`
- **Captured support / export evidence (analysis-only bundle viewer)** [`asvc-support-evidence-support`]: object `support_export_evidence` on `support`, posture `exported_redacted`, role `provenance_attribution`
- **Captured support / export evidence (analysis-only bundle viewer)** [`asvc-support-evidence-help`]: object `support_export_evidence` on `help_docs`, posture `live_target_openable`, role `provenance_attribution`
- **Captured support / export evidence (analysis-only bundle viewer)** [`asvc-support-evidence-companion`]: object `support_export_evidence` on `companion_export`, posture `exported_redacted`, role `provenance_attribution`
- **Archived runbook execution packet (historical run, validated reopen)** [`asvc-runbook-archive`]: object `archived_runbook_packet` on `runbook_archive`, posture `live_target_openable`, role `live_target_handoff`
- **Archived runbook execution packet (historical run, validated reopen)** [`asvc-runbook-review`]: object `archived_runbook_packet` on `review_incident`, posture `metadata_only_exit`, role `live_target_handoff`
- **Archived runbook execution packet (historical run, validated reopen)** [`asvc-runbook-program`]: object `archived_runbook_packet` on `program_governance`, posture `exported_redacted`, role `live_target_handoff`
- **Imported / offline route evidence (offline-only, not live truth)** [`asvc-imported-shell`]: object `imported_offline_route_evidence` on `shell`, posture `imported_offline_only`, role `imported_offline_disclosure`
- **Imported / offline route evidence (offline-only, not live truth)** [`asvc-imported-runbook`]: object `imported_offline_route_evidence` on `runbook_archive`, posture `imported_offline_only`, role `imported_offline_disclosure`
- **Imported / offline route evidence (offline-only, not live truth)** [`asvc-imported-cli`]: object `imported_offline_route_evidence` on `cli_export`, posture `exported_redacted`, role `imported_offline_disclosure`
- **Review / incident snapshot (analysis-only evidence reopen flow)** [`asvc-review-review`]: object `review_incident_snapshot` on `review_incident`, posture `live_target_openable`, role `mutation_blocked_posture`
- **Review / incident snapshot (analysis-only evidence reopen flow)** [`asvc-review-shell`]: object `review_incident_snapshot` on `shell`, posture `metadata_only_exit`, role `mutation_blocked_posture`
- **Review / incident snapshot (analysis-only evidence reopen flow)** [`asvc-review-companion`]: object `review_incident_snapshot` on `companion_export`, posture `exported_redacted`, role `mutation_blocked_posture`
